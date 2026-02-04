//! Lambda handler for OOBI endpoints
//!
//! GET /introduce - Get witness OOBI
//! GET /oobi/{id} - Resolve OOBI for an identifier

use aws_lambda_events::apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse};
use aws_lambda_events::encodings::Body;
use aws_lambda_events::http::HeaderMap;
use kerihost_db::DynamoDbDatabase;
use kerihost_witness::{oobi::Oobi, Witness, WitnessConfig};
use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde_json::json;
use std::sync::Arc;
use tokio::sync::OnceCell;
use tracing::info;

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

    let path = event.payload.path.as_deref().unwrap_or("/");

    // Handle /introduce endpoint
    if path == "/introduce" || path == "/" {
        info!("Serving witness OOBI");

        let oobi = Oobi::witness(&witness.prefix, &witness.oobi_url());

        return Ok(response(
            200,
            json!({
                "oobi": oobi.to_url_string(),
                "witness": witness.prefix,
                "asOf": now
            }),
        ));
    }

    // Handle /oobi/{id} endpoint
    if path.starts_with("/oobi/") {
        let parts: Vec<&str> = path
            .trim_start_matches("/oobi/")
            .split('/')
            .collect();

        if parts.is_empty() || parts[0].is_empty() {
            return Ok(response(
                400,
                json!({
                    "error": "Missing identifier in OOBI path",
                    "asOf": now
                }),
            ));
        }

        let prefix = parts[0];
        info!(prefix = %prefix, "Resolving OOBI");

        // Get state for the identifier
        match witness.get_state(prefix).await {
            Ok(Some(state)) => {
                // Return state and OOBI info
                Ok(response(
                    200,
                    json!({
                        "state": state,
                        "oobi": Oobi::controller(prefix, &witness.prefix, &witness.oobi_url()).to_url_string(),
                        "asOf": now
                    }),
                ))
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
                Ok(response(
                    500,
                    json!({
                        "error": e.to_string(),
                        "asOf": now
                    }),
                ))
            }
        }
    } else {
        Ok(response(
            404,
            json!({
                "error": "Not found",
                "path": path,
                "asOf": now
            }),
        ))
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
    fn test_oobi_url_generation() {
        let oobi = Oobi::witness("BWitness123", "https://witness.example.com");
        assert_eq!(
            oobi.to_url_string(),
            "https://witness.example.com/oobi/BWitness123"
        );
    }

    #[test]
    fn test_controller_oobi() {
        let oobi = Oobi::controller(
            "DController123",
            "BWitness123",
            "https://witness.example.com",
        );
        assert_eq!(
            oobi.to_url_string(),
            "https://witness.example.com/oobi/DController123/witness/BWitness123"
        );
    }
}
