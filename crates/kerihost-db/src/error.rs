//! Database error types

use thiserror::Error;

/// Database errors
#[derive(Error, Debug)]
pub enum DbError {
    /// Failed to connect to database
    #[error("Connection error: {0}")]
    Connection(String),

    /// Item not found
    #[error("Not found: {0}")]
    NotFound(String),

    /// Prior digest mismatch (conditional write failed)
    #[error("Prior digest mismatch: expected {expected}, actual {actual}")]
    PriorDigestMismatch { expected: String, actual: String },

    /// Duplicate item
    #[error("Duplicate: {0}")]
    Duplicate(String),

    /// Serialization error
    #[error("Serialization error: {0}")]
    Serialization(String),

    /// DynamoDB error
    #[error("DynamoDB error: {0}")]
    DynamoDb(String),

    /// Generic error
    #[error("Database error: {0}")]
    Other(String),
}

impl From<serde_json::Error> for DbError {
    fn from(err: serde_json::Error) -> Self {
        DbError::Serialization(err.to_string())
    }
}

/// Result type for database operations
pub type DbResult<T> = Result<T, DbError>;
