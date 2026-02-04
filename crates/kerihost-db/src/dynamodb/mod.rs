//! DynamoDB database implementation
//!
//! This implementation uses AWS DynamoDB for persistent storage.
//! It follows the KERI-honest design principles with conditional writes
//! to enforce event ordering.

mod client;
mod escrows;
mod kel;
mod receipts;
mod states;

pub use client::DynamoDbDatabase;

/// Table names configuration
#[derive(Debug, Clone)]
pub struct TableConfig {
    /// KEL table name
    pub kel_table: String,
    /// States table name
    pub states_table: String,
    /// Receipts table name
    pub receipts_table: String,
    /// Escrows table name
    pub escrows_table: String,
}

impl TableConfig {
    /// Create config from environment variables
    pub fn from_env() -> Self {
        TableConfig {
            kel_table: std::env::var("KEL_TABLE").unwrap_or_else(|_| "kerihost-kel".to_string()),
            states_table: std::env::var("STATES_TABLE")
                .unwrap_or_else(|_| "kerihost-states".to_string()),
            receipts_table: std::env::var("RECEIPTS_TABLE")
                .unwrap_or_else(|_| "kerihost-receipts".to_string()),
            escrows_table: std::env::var("ESCROWS_TABLE")
                .unwrap_or_else(|_| "kerihost-escrows".to_string()),
        }
    }

    /// Create with custom table names
    pub fn new(kel: &str, states: &str, receipts: &str, escrows: &str) -> Self {
        TableConfig {
            kel_table: kel.to_string(),
            states_table: states.to_string(),
            receipts_table: receipts.to_string(),
            escrows_table: escrows.to_string(),
        }
    }
}

impl Default for TableConfig {
    fn default() -> Self {
        Self::from_env()
    }
}
