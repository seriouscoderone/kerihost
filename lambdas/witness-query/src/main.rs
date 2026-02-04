//! Lambda handler for querying KERI state
//!
//! POST /query - Query KEL, state, or receipts
//!
//! This handler supports:
//! - state: Get current key state for an identifier
//! - kel: Get events from the KEL
//! - receipts: Get receipts for an event

use aws_lambda_events::apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse};
use aws_lambda_events::encodings::Body;
use aws_lambda_events::http::HeaderMap;
use kerihost_db::DynamoDbDatabase;
use kerihost_witness::{Witness, WitnessConfig};
use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;
use tokio::sync::OnceCell;
use tracing::{info, error};

/// Global witness instance
static WITNESS: OnceCell<Witness<DynamoDbDatabase>> = OnceCell::const_new();

/// Initialize the witness
async fn init_witness() -> Witness<DynamoDbDatabase> {
    let db = Arc::new(DynamoDbDatabase::from_env().await);
    let config = WitnessConfig::from_env();
    Witness::new(None, db, config)
}

/// Get or initialize the witness
async fn get_witness() -> &'static Witness<DynamoDbDatabase> {
    WITNESS.get_or_init(init_witness).await
}

/// Query request
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
struct QueryRequest {
    /// Type of query
    query_type: String,
    /// Identifier prefix
    prefix: Option<String>,
    /// Event digest (for receipts query)
    event_digest: Option<String>,
    /// Start sequence number (for kel query)
    start_sn: Option<u64>,
    /// End sequence number (for kel query)
    end_sn: Option<u64>,
}

/// Create API Gateway response
fn response(status: i64, body: serde_json::Value) -> ApiGatewayProxyResponse {
    let mut headers = HeaderMap::new();
    headers.insert("content-type", "application/json".parse().unwrap());
    headers.insert("access-control-allow-origin", "*".parse().unwrap());

    ApiGatewayProxyResponse {
        status_code: status,
        multi_value_headers: headers.clone(),
        headers,
        body: Some(Body::Text(body.to_string())),
        is_base64_encoded: false,
    }
}

/// Lambda handler
async fn handler(
    event: LambdaEvent<ApiGatewayProxyRequest>,
) -> Result<ApiGatewayProxyResponse, Error> {
    let witness = get_witness().await;
    let now = chrono::Utc::now().to_rfc3339();

    // Parse query request
    let query: QueryRequest = match &event.payload.body {
        Some(body) => serde_json::from_str(body).map_err(|e| {
            error!(error = %e, "Failed to parse query request");
            e
        })?,
        None => {
            return Ok(response(
                400,
                json!({
                    "error": "Missing request body",
                    "asOf": now
                }),
            ));
        }
    };

    match query.query_type.as_str() {
        "state" => {
            let prefix = match query.prefix {
                Some(p) => p,
                None => {
                    return Ok(response(
                        400,
                        json!({
                            "error": "Missing prefix for state query",
                            "asOf": now
                        }),
                    ));
                }
            };

            match witness.get_state(&prefix).await {
                Ok(Some(state)) => {
                    info!(prefix = %prefix, "State query successful");
                    Ok(response(200, json!({ "state": state, "asOf": now })))
                }
                Ok(None) => {
                    Ok(response(
                        404,
                        json!({
                            "error": "Identifier not found",
                            "prefix": prefix,
                            "asOf": now
                        }),
                    ))
                }
                Err(e) => {
                    error!(error = %e, "State query failed");
                    Ok(response(
                        500,
                        json!({
                            "error": e.to_string(),
                            "asOf": now
                        }),
                    ))
                }
            }
        }

        "kel" => {
            let prefix = match query.prefix {
                Some(p) => p,
                None => {
                    return Ok(response(
                        400,
                        json!({
                            "error": "Missing prefix for kel query",
                            "asOf": now
                        }),
                    ));
                }
            };

            let start = query.start_sn.unwrap_or(0);
            let end = query.end_sn;

            match witness.get_kel(&prefix, start, end).await {
                Ok(events) => {
                    info!(prefix = %prefix, count = %events.len(), "KEL query successful");
                    Ok(response(
                        200,
                        json!({
                            "events": events,
                            "count": events.len(),
                            "asOf": now
                        }),
                    ))
                }
                Err(e) => {
                    error!(error = %e, "KEL query failed");
                    Ok(response(
                        500,
                        json!({
                            "error": e.to_string(),
                            "asOf": now
                        }),
                    ))
                }
            }
        }

        "receipts" => {
            let digest = match query.event_digest {
                Some(d) => d,
                None => {
                    return Ok(response(
                        400,
                        json!({
                            "error": "Missing event_digest for receipts query",
                            "asOf": now
                        }),
                    ));
                }
            };

            match witness.get_receipts(&digest).await {
                Ok(receipts) => {
                    info!(digest = %digest, count = %receipts.len(), "Receipts query successful");
                    Ok(response(
                        200,
                        json!({
                            "receipts": receipts,
                            "count": receipts.len(),
                            "asOf": now
                        }),
                    ))
                }
                Err(e) => {
                    error!(error = %e, "Receipts query failed");
                    Ok(response(
                        500,
                        json!({
                            "error": e.to_string(),
                            "asOf": now
                        }),
                    ))
                }
            }
        }

        _ => Ok(response(
            400,
            json!({
                "error": format!("Unknown query type: {}", query.query_type),
                "asOf": now
            }),
        )),
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .json()
        .init();

    lambda_runtime::run(service_fn(handler)).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_request_parse() {
        let json = r#"{"query_type": "state", "prefix": "DTest123"}"#;
        let query: QueryRequest = serde_json::from_str(json).unwrap();
        assert_eq!(query.query_type, "state");
        assert_eq!(query.prefix, Some("DTest123".to_string()));
    }

    #[test]
    fn test_query_request_kel() {
        let json = r#"{"query_type": "kel", "prefix": "DTest123", "start_sn": 0, "end_sn": 10}"#;
        let query: QueryRequest = serde_json::from_str(json).unwrap();
        assert_eq!(query.query_type, "kel");
        assert_eq!(query.start_sn, Some(0));
        assert_eq!(query.end_sn, Some(10));
    }
}
