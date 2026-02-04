//! Receipt storage implementation for DynamoDB

use super::DynamoDbDatabase;
use crate::error::{DbError, DbResult};
use crate::traits::ReceiptStore;
use async_trait::async_trait;
use aws_sdk_dynamodb::types::AttributeValue;
use kerihost_core::NontransferableReceipt;
use std::collections::HashMap;

#[async_trait]
impl ReceiptStore for DynamoDbDatabase {
    async fn add_receipt(&self, receipt: &NontransferableReceipt) -> DbResult<()> {
        let receipt_json =
            serde_json::to_string(receipt).map_err(|e| DbError::Serialization(e.to_string()))?;

        let mut item = HashMap::new();
        item.insert(
            "event_digest".to_string(),
            AttributeValue::S(receipt.event_digest.clone()),
        );
        item.insert(
            "witness_aid".to_string(),
            AttributeValue::S(receipt.witness_prefix.clone()),
        );
        item.insert("receipt".to_string(), AttributeValue::S(receipt_json));
        item.insert(
            "signature".to_string(),
            AttributeValue::S(receipt.signature.clone()),
        );
        item.insert(
            "event_sn".to_string(),
            AttributeValue::N(receipt.event_sn.to_string()),
        );
        item.insert(
            "event_aid".to_string(),
            AttributeValue::S(receipt.event_prefix.clone()),
        );

        self.client
            .put_item()
            .table_name(&self.config.receipts_table)
            .set_item(Some(item))
            .send()
            .await
            .map_err(|e| DbError::DynamoDb(e.to_string()))?;

        Ok(())
    }

    async fn get_receipts(&self, event_digest: &str) -> DbResult<Vec<NontransferableReceipt>> {
        let result = self
            .client
            .query()
            .table_name(&self.config.receipts_table)
            .key_condition_expression("event_digest = :digest")
            .expression_attribute_values(
                ":digest",
                AttributeValue::S(event_digest.to_string()),
            )
            .send()
            .await
            .map_err(|e| DbError::DynamoDb(e.to_string()))?;

        let mut receipts = Vec::new();
        if let Some(items) = result.items {
            for item in items {
                if let Some(receipt_json) = item.get("receipt").and_then(|v| v.as_s().ok()) {
                    let receipt: NontransferableReceipt = serde_json::from_str(receipt_json)?;
                    receipts.push(receipt);
                }
            }
        }

        Ok(receipts)
    }

    async fn get_receipt(
        &self,
        event_digest: &str,
        witness_prefix: &str,
    ) -> DbResult<Option<NontransferableReceipt>> {
        let result = self
            .client
            .get_item()
            .table_name(&self.config.receipts_table)
            .key("event_digest", AttributeValue::S(event_digest.to_string()))
            .key("witness_aid", AttributeValue::S(witness_prefix.to_string()))
            .send()
            .await
            .map_err(|e| DbError::DynamoDb(e.to_string()))?;

        match result.item {
            Some(item) => {
                let receipt_json = item
                    .get("receipt")
                    .and_then(|v| v.as_s().ok())
                    .ok_or_else(|| DbError::Other("Missing receipt field".to_string()))?;

                let receipt: NontransferableReceipt = serde_json::from_str(receipt_json)?;
                Ok(Some(receipt))
            }
            None => Ok(None),
        }
    }

    async fn count_receipts(&self, event_digest: &str) -> DbResult<usize> {
        let result = self
            .client
            .query()
            .table_name(&self.config.receipts_table)
            .key_condition_expression("event_digest = :digest")
            .expression_attribute_values(
                ":digest",
                AttributeValue::S(event_digest.to_string()),
            )
            .select(aws_sdk_dynamodb::types::Select::Count)
            .send()
            .await
            .map_err(|e| DbError::DynamoDb(e.to_string()))?;

        Ok(result.count as usize)
    }
}
