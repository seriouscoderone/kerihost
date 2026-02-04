//! Witness error types

use kerihost_core::CoreError;
use kerihost_db::DbError;
use thiserror::Error;

/// Witness errors
#[derive(Error, Debug)]
pub enum WitnessError {
    /// Core KERI error
    #[error("Core error: {0}")]
    Core(#[from] CoreError),

    /// Database error
    #[error("Database error: {0}")]
    Database(#[from] DbError),

    /// Event validation failed
    #[error("Validation error: {0}")]
    Validation(String),

    /// Witness not authorized for this identifier
    #[error("Witness not authorized for {prefix}")]
    NotAuthorized { prefix: String },

    /// Missing witness signer
    #[error("Witness signer not configured")]
    MissingSigner,

    /// CESR encoding error
    #[error("CESR error: {0}")]
    Cesr(String),

    /// Configuration error
    #[error("Configuration error: {0}")]
    Config(String),
}

/// Result type for witness operations
pub type WitnessResult<T> = Result<T, WitnessError>;
