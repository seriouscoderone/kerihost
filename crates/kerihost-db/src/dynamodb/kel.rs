//! KEL storage implementation for DynamoDB

use super::DynamoDbDatabase;
use crate::error::{DbError, DbResult};
use crate::traits::KelStore;
use async_trait::async_trait;
use aws_sdk_dynamodb::types::AttributeValue;
use kerihost_core::SignedEvent;
use std::collections::HashMap;

/// Zero-pad sequence number for sort key
fn sn_to_sk(sn: u64) -> String {
    format!("{:016x}", sn)
}

/// Parse sequence number from sort key
#[allow(dead_code)]
fn sk_to_sn(sk: &str) -> Option<u64> {
    u64::from_str_radix(sk, 16).ok()
}

#[async_trait]
impl KelStore for DynamoDbDatabase {
    async fn append_event(&self, event: &SignedEvent) -> DbResult<()> {
        let prefix = &event.event.prefix;
        let sn = event.event.sn;
        let sk = sn_to_sk(sn);

        // Serialize event
        let event_json =
            serde_json::to_string(event).map_err(|e| DbError::Serialization(e.to_string()))?;

        let mut item = HashMap::new();
        item.insert("aid".to_string(), AttributeValue::S(prefix.clone()));
        item.insert("sn".to_string(), AttributeValue::S(sk));
        item.insert("digest".to_string(), AttributeValue::S(event.event.digest.clone()));
        item.insert("event".to_string(), AttributeValue::S(event_json));

        if let Some(ref prior) = event.event.prior_digest {
            item.insert("prior_digest".to_string(), AttributeValue::S(prior.clone()));
        }

        // Build conditional expression
        // For non-inception events, we simply check that this exact (aid, sn) doesn't exist yet
        // The prior digest validation is handled by the processor before calling append_event
        let condition_expr = "attribute_not_exists(aid) AND attribute_not_exists(sn)";

        self.client
            .put_item()
            .table_name(&self.config.kel_table)
            .set_item(Some(item))
            .condition_expression(condition_expr)
            .send()
            .await
            .map_err(|e| {
                let err_str = e.to_string();
                if err_str.contains("ConditionalCheckFailed") {
                    DbError::Duplicate(format!("Event already exists for {} at sn {}", prefix, sn))
                } else {
                    DbError::DynamoDb(err_str)
                }
            })?;

        Ok(())
    }

    async fn get_event(&self, prefix: &str, sn: u64) -> DbResult<Option<SignedEvent>> {
        let sk = sn_to_sk(sn);

        let result = self
            .client
            .get_item()
            .table_name(&self.config.kel_table)
            .key("aid", AttributeValue::S(prefix.to_string()))
            .key("sn", AttributeValue::S(sk))
            .send()
            .await
            .map_err(|e| DbError::DynamoDb(e.to_string()))?;

        match result.item {
            Some(item) => {
                let event_json = item
                    .get("event")
                    .and_then(|v| v.as_s().ok())
                    .ok_or_else(|| DbError::Other("Missing event field".to_string()))?;

                let event: SignedEvent = serde_json::from_str(event_json)?;
                Ok(Some(event))
            }
            None => Ok(None),
        }
    }

    async fn get_events(
        &self,
        prefix: &str,
        start_sn: u64,
        end_sn: Option<u64>,
    ) -> DbResult<Vec<SignedEvent>> {
        let start_sk = sn_to_sk(start_sn);

        let mut query = self
            .client
            .query()
            .table_name(&self.config.kel_table)
            .key_condition_expression("aid = :aid AND sn >= :start_sn")
            .expression_attribute_values(":aid", AttributeValue::S(prefix.to_string()))
            .expression_attribute_values(":start_sn", AttributeValue::S(start_sk));

        if let Some(end) = end_sn {
            let end_sk = sn_to_sk(end);
            query = query
                .key_condition_expression("aid = :aid AND sn BETWEEN :start_sn AND :end_sn")
                .expression_attribute_values(":end_sn", AttributeValue::S(end_sk));
        }

        let result = query
            .send()
            .await
            .map_err(|e| DbError::DynamoDb(e.to_string()))?;

        let mut events = Vec::new();
        if let Some(items) = result.items {
            for item in items {
                if let Some(event_json) = item.get("event").and_then(|v| v.as_s().ok()) {
                    let event: SignedEvent = serde_json::from_str(event_json)?;
                    events.push(event);
                }
            }
        }

        // Sort by sn
        events.sort_by_key(|e| e.event.sn);

        Ok(events)
    }

    async fn get_latest(&self, prefix: &str) -> DbResult<Option<SignedEvent>> {
        let result = self
            .client
            .query()
            .table_name(&self.config.kel_table)
            .key_condition_expression("aid = :aid")
            .expression_attribute_values(":aid", AttributeValue::S(prefix.to_string()))
            .scan_index_forward(false) // Descending order
            .limit(1)
            .send()
            .await
            .map_err(|e| DbError::DynamoDb(e.to_string()))?;

        if let Some(items) = result.items {
            if let Some(item) = items.into_iter().next() {
                if let Some(event_json) = item.get("event").and_then(|v| v.as_s().ok()) {
                    let event: SignedEvent = serde_json::from_str(event_json)?;
                    return Ok(Some(event));
                }
            }
        }

        Ok(None)
    }

    async fn get_event_by_digest(
        &self,
        prefix: &str,
        digest: &str,
    ) -> DbResult<Option<SignedEvent>> {
        // Query by prefix and filter by digest
        // In production, consider a GSI on digest
        let result = self
            .client
            .query()
            .table_name(&self.config.kel_table)
            .key_condition_expression("aid = :aid")
            .filter_expression("digest = :digest")
            .expression_attribute_values(":aid", AttributeValue::S(prefix.to_string()))
            .expression_attribute_values(":digest", AttributeValue::S(digest.to_string()))
            .send()
            .await
            .map_err(|e| DbError::DynamoDb(e.to_string()))?;

        if let Some(items) = result.items {
            if let Some(item) = items.into_iter().next() {
                if let Some(event_json) = item.get("event").and_then(|v| v.as_s().ok()) {
                    let event: SignedEvent = serde_json::from_str(event_json)?;
                    return Ok(Some(event));
                }
            }
        }

        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sn_to_sk() {
        assert_eq!(sn_to_sk(0), "0000000000000000");
        assert_eq!(sn_to_sk(1), "0000000000000001");
        assert_eq!(sn_to_sk(255), "00000000000000ff");
        assert_eq!(sn_to_sk(u64::MAX), "ffffffffffffffff");
    }

    #[test]
    fn test_sk_to_sn() {
        assert_eq!(sk_to_sn("0000000000000000"), Some(0));
        assert_eq!(sk_to_sn("0000000000000001"), Some(1));
        assert_eq!(sk_to_sn("00000000000000ff"), Some(255));
        assert_eq!(sk_to_sn("ffffffffffffffff"), Some(u64::MAX));
        assert_eq!(sk_to_sn("invalid"), None);
    }
}
