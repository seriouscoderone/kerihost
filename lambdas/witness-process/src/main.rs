//! Lambda handler for processing KERI events
//!
//! POST /process - Receive and process KERI events
//!
//! This handler:
//! 1. Parses incoming CESR-encoded events
//! 2. Validates and processes them through the witness
//! 3. Returns KERI-honest responses with confidence levels

use aws_lambda_events::apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse};
use aws_lambda_events::encodings::Body;
use aws_lambda_events::http::HeaderMap;
use kerihost_db::DynamoDbDatabase;
use kerihost_witness::{ProcessResult, Witness, WitnessConfig};
use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde_json::json;
use std::sync::Arc;
use tokio::sync::OnceCell;
use tracing::{info, error};

/// Global witness instance (initialized once)
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

    // Get request body
    let body = match &event.payload.body {
        Some(b) => b.clone(),
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

    // Decode body (may be base64 encoded)
    let raw: Vec<u8> = if event.payload.is_base64_encoded {
        match base64_decode(&body) {
            Ok(bytes) => bytes,
            Err(e) => {
                return Ok(response(
                    400,
                    json!({
                        "error": format!("Failed to decode base64: {}", e),
                        "asOf": now
                    }),
                ));
            }
        }
    } else {
        body.into_bytes()
    };

    // Process the event
    match witness.process_notice(&raw).await {
        Ok(ProcessResult::Accepted { receipt, state }) => {
            info!(
                prefix = %state.prefix,
                sn = %state.sn,
                "Event accepted"
            );

            let receipt_cesr = receipt.map(|r| {
                serde_json::to_value(&r).unwrap_or(json!(null))
            });

            Ok(response(
                200,
                json!({
                    "status": "accepted",
                    "receipt": receipt_cesr,
                    "state": {
                        "prefix": state.prefix,
                        "sn": state.sn,
                        "digest": state.latest_digest
                    },
                    "confidence": state.metadata.confidence,
                    "witnessesSeen": state.metadata.witnesses_seen,
                    "witnessesRequired": state.metadata.witnesses_required,
                    "asOf": now
                }),
            ))
        }
        Ok(ProcessResult::Escrowed { reason }) => {
            info!(reason = %reason, "Event escrowed");

            Ok(response(
                202,
                json!({
                    "status": "escrowed",
                    "reason": reason.to_string(),
                    "confidence": "LOCAL_ONLY",
                    "asOf": now
                }),
            ))
        }
        Ok(ProcessResult::Duplicate) => {
            Ok(response(
                200,
                json!({
                    "status": "duplicate",
                    "asOf": now
                }),
            ))
        }
        Err(e) => {
            error!(error = %e, "Failed to process event");

            Ok(response(
                400,
                json!({
                    "error": e.to_string(),
                    "asOf": now
                }),
            ))
        }
    }
}

/// Decode base64 string
fn base64_decode(s: &str) -> Result<Vec<u8>, String> {
    // Simple base64 decode without external crate
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    let mut result = Vec::new();
    let bytes: Vec<u8> = s.bytes().filter(|&b| b != b'\n' && b != b'\r').collect();

    for chunk in bytes.chunks(4) {
        let mut bits: u32 = 0;
        let mut count = 0;

        for &b in chunk {
            if b == b'=' {
                break;
            }
            let idx = CHARS.iter().position(|&c| c == b)
                .ok_or_else(|| format!("Invalid base64 char: {}", b as char))?;
            bits = (bits << 6) | (idx as u32);
            count += 1;
        }

        match count {
            2 => {
                result.push((bits >> 4) as u8);
            }
            3 => {
                result.push((bits >> 10) as u8);
                result.push((bits >> 2) as u8);
            }
            4 => {
                result.push((bits >> 16) as u8);
                result.push((bits >> 8) as u8);
                result.push(bits as u8);
            }
            _ => {}
        }
    }

    Ok(result)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Initialize tracing
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
    fn test_base64_decode() {
        let encoded = "SGVsbG8gV29ybGQ=";
        let decoded = base64_decode(encoded).unwrap();
        assert_eq!(String::from_utf8(decoded).unwrap(), "Hello World");
    }

    #[test]
    fn test_response_format() {
        let resp = response(200, json!({"status": "ok"}));
        assert_eq!(resp.status_code, 200);
    }
}
