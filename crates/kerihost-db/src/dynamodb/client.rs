//! DynamoDB client wrapper

use super::TableConfig;
use aws_sdk_dynamodb::Client;

/// DynamoDB database implementation
#[derive(Clone)]
pub struct DynamoDbDatabase {
    pub(crate) client: Client,
    pub(crate) config: TableConfig,
}

impl DynamoDbDatabase {
    /// Create new DynamoDB database with config
    pub fn new(client: Client, config: TableConfig) -> Self {
        DynamoDbDatabase { client, config }
    }

    /// Create from environment
    pub async fn from_env() -> Self {
        let aws_config = aws_config::defaults(aws_config::BehaviorVersion::latest())
            .load()
            .await;
        let client = Client::new(&aws_config);
        let config = TableConfig::from_env();
        DynamoDbDatabase { client, config }
    }

    /// Create with custom endpoint (for local testing)
    pub async fn with_endpoint(endpoint: &str, config: TableConfig) -> Self {
        let aws_config = aws_config::defaults(aws_config::BehaviorVersion::latest())
            .endpoint_url(endpoint)
            .load()
            .await;
        let client = Client::new(&aws_config);
        DynamoDbDatabase { client, config }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_table_config_default() {
        let config = TableConfig::default();
        assert!(!config.kel_table.is_empty());
        assert!(!config.states_table.is_empty());
    }

    #[test]
    fn test_table_config_custom() {
        let config = TableConfig::new("my-kel", "my-states", "my-receipts", "my-escrows");
        assert_eq!(config.kel_table, "my-kel");
        assert_eq!(config.states_table, "my-states");
    }
}
