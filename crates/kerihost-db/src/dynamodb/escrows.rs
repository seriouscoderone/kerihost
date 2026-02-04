//! Escrow storage implementation for DynamoDB

use super::DynamoDbDatabase;
use crate::error::{DbError, DbResult};
use crate::traits::{EscrowReason, EscrowStore, EscrowedEvent};
use async_trait::async_trait;
use aws_sdk_dynamodb::types::AttributeValue;
use kerihost_core::SignedEvent;
use std::collections::HashMap;

/// Default escrow TTL in seconds (1 hour)
const DEFAULT_ESCROW_TTL: u64 = 3600;

#[async_trait]
impl EscrowStore for DynamoDbDatabase {
    async fn escrow_event(&self, event: &SignedEvent, reason: EscrowReason) -> DbResult<()> {
        let escrowed = EscrowedEvent::new(event.clone(), reason, DEFAULT_ESCROW_TTL);
        let escrowed_json =
            serde_json::to_string(&escrowed).map_err(|e| DbError::Serialization(e.to_string()))?;

        // Sort key format: reason#digest for efficient queries by reason
        let sk = format!("{}#{}", reason, event.event.digest);

        let mut item = HashMap::new();
        item.insert(
            "aid".to_string(),
            AttributeValue::S(event.event.prefix.clone()),
        );
        item.insert("reason_digest".to_string(), AttributeValue::S(sk));
        item.insert("escrowed".to_string(), AttributeValue::S(escrowed_json));
        item.insert(
            "digest".to_string(),
            AttributeValue::S(event.event.digest.clone()),
        );
        item.insert(
            "reason".to_string(),
            AttributeValue::S(reason.to_string()),
        );
        item.insert("ttl".to_string(), AttributeValue::N(escrowed.ttl.to_string()));
        // GSI requires created as a number (epoch millis)
        let created_millis = chrono::DateTime::parse_from_rfc3339(&escrowed.created)
            .map(|dt| dt.timestamp_millis())
            .unwrap_or_else(|_| chrono::Utc::now().timestamp_millis());
        item.insert(
            "created".to_string(),
            AttributeValue::N(created_millis.to_string()),
        );

        self.client
            .put_item()
            .table_name(&self.config.escrows_table)
            .set_item(Some(item))
            .send()
            .await
            .map_err(|e| DbError::DynamoDb(e.to_string()))?;

        Ok(())
    }

    async fn get_escrowed(&self, prefix: &str) -> DbResult<Vec<EscrowedEvent>> {
        let result = self
            .client
            .query()
            .table_name(&self.config.escrows_table)
            .key_condition_expression("aid = :aid")
            .expression_attribute_values(":aid", AttributeValue::S(prefix.to_string()))
            .send()
            .await
            .map_err(|e| DbError::DynamoDb(e.to_string()))?;

        let mut escrowed = Vec::new();
        if let Some(items) = result.items {
            for item in items {
                if let Some(escrowed_json) = item.get("escrowed").and_then(|v| v.as_s().ok()) {
                    let e: EscrowedEvent = serde_json::from_str(escrowed_json)?;
                    escrowed.push(e);
                }
            }
        }

        Ok(escrowed)
    }

    async fn get_all_escrowed(&self) -> DbResult<Vec<EscrowedEvent>> {
        // Scan all escrows - in production, consider pagination
        let result = self
            .client
            .scan()
            .table_name(&self.config.escrows_table)
            .send()
            .await
            .map_err(|e| DbError::DynamoDb(e.to_string()))?;

        let mut escrowed = Vec::new();
        if let Some(items) = result.items {
            for item in items {
                if let Some(escrowed_json) = item.get("escrowed").and_then(|v| v.as_s().ok()) {
                    let e: EscrowedEvent = serde_json::from_str(escrowed_json)?;
                    escrowed.push(e);
                }
            }
        }

        Ok(escrowed)
    }

    async fn promote_escrowed(&self, event_digest: &str) -> DbResult<Option<SignedEvent>> {
        // First, find the escrowed event by digest
        // We need to scan since digest is not a key
        let result = self
            .client
            .scan()
            .table_name(&self.config.escrows_table)
            .filter_expression("digest = :digest")
            .expression_attribute_values(":digest", AttributeValue::S(event_digest.to_string()))
            .send()
            .await
            .map_err(|e| DbError::DynamoDb(e.to_string()))?;

        let item = match result.items.and_then(|items| items.into_iter().next()) {
            Some(item) => item,
            None => return Ok(None),
        };

        // Get the event
        let escrowed_json = item
            .get("escrowed")
            .and_then(|v| v.as_s().ok())
            .ok_or_else(|| DbError::Other("Missing escrowed field".to_string()))?;
        let escrowed: EscrowedEvent = serde_json::from_str(escrowed_json)?;

        // Delete the escrow
        let aid = item
            .get("aid")
            .and_then(|v| v.as_s().ok())
            .ok_or_else(|| DbError::Other("Missing aid field".to_string()))?;
        let reason_digest = item
            .get("reason_digest")
            .and_then(|v| v.as_s().ok())
            .ok_or_else(|| DbError::Other("Missing reason_digest field".to_string()))?;

        self.client
            .delete_item()
            .table_name(&self.config.escrows_table)
            .key("aid", AttributeValue::S(aid.to_string()))
            .key("reason_digest", AttributeValue::S(reason_digest.to_string()))
            .send()
            .await
            .map_err(|e| DbError::DynamoDb(e.to_string()))?;

        Ok(Some(escrowed.event))
    }

    async fn remove_escrowed(&self, event_digest: &str) -> DbResult<()> {
        // Find and delete the escrowed event
        let result = self
            .client
            .scan()
            .table_name(&self.config.escrows_table)
            .filter_expression("digest = :digest")
            .expression_attribute_values(":digest", AttributeValue::S(event_digest.to_string()))
            .send()
            .await
            .map_err(|e| DbError::DynamoDb(e.to_string()))?;

        if let Some(items) = result.items {
            for item in items {
                let aid = item.get("aid").and_then(|v| v.as_s().ok());
                let reason_digest = item.get("reason_digest").and_then(|v| v.as_s().ok());

                if let (Some(aid), Some(rd)) = (aid, reason_digest) {
                    self.client
                        .delete_item()
                        .table_name(&self.config.escrows_table)
                        .key("aid", AttributeValue::S(aid.to_string()))
                        .key("reason_digest", AttributeValue::S(rd.to_string()))
                        .send()
                        .await
                        .map_err(|e| DbError::DynamoDb(e.to_string()))?;
                }
            }
        }

        Ok(())
    }
}
