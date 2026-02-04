//! State storage implementation for DynamoDB

use super::DynamoDbDatabase;
use crate::error::{DbError, DbResult};
use crate::traits::StateStore;
use async_trait::async_trait;
use aws_sdk_dynamodb::types::AttributeValue;
use kerihost_core::KeyState;
use std::collections::HashMap;

#[async_trait]
impl StateStore for DynamoDbDatabase {
    async fn get_state(&self, prefix: &str) -> DbResult<Option<KeyState>> {
        let result = self
            .client
            .get_item()
            .table_name(&self.config.states_table)
            .key("aid", AttributeValue::S(prefix.to_string()))
            .send()
            .await
            .map_err(|e| DbError::DynamoDb(e.to_string()))?;

        match result.item {
            Some(item) => {
                let state_json = item
                    .get("state")
                    .and_then(|v| v.as_s().ok())
                    .ok_or_else(|| DbError::Other("Missing state field".to_string()))?;

                let state: KeyState = serde_json::from_str(state_json)?;
                Ok(Some(state))
            }
            None => Ok(None),
        }
    }

    async fn put_state(&self, state: &KeyState) -> DbResult<()> {
        let state_json =
            serde_json::to_string(state).map_err(|e| DbError::Serialization(e.to_string()))?;

        let mut item = HashMap::new();
        item.insert("aid".to_string(), AttributeValue::S(state.prefix.clone()));
        item.insert("state".to_string(), AttributeValue::S(state_json));
        item.insert("sn".to_string(), AttributeValue::N(state.sn.to_string()));
        item.insert(
            "digest".to_string(),
            AttributeValue::S(state.latest_digest.clone()),
        );

        self.client
            .put_item()
            .table_name(&self.config.states_table)
            .set_item(Some(item))
            .send()
            .await
            .map_err(|e| DbError::DynamoDb(e.to_string()))?;

        Ok(())
    }

    async fn delete_state(&self, prefix: &str) -> DbResult<()> {
        self.client
            .delete_item()
            .table_name(&self.config.states_table)
            .key("aid", AttributeValue::S(prefix.to_string()))
            .send()
            .await
            .map_err(|e| DbError::DynamoDb(e.to_string()))?;

        Ok(())
    }
}
