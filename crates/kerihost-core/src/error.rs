//! Error types for kerihost-core

use thiserror::Error;

/// Core errors for KERI event processing
#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum CoreError {
    /// Failed to parse CESR-encoded data
    #[error("CESR parsing error: {0}")]
    CesrParse(String),

    /// Invalid event structure
    #[error("Invalid event structure: {0}")]
    InvalidEvent(String),

    /// Invalid signature
    #[error("Invalid signature: {0}")]
    InvalidSignature(String),

    /// Sequence number mismatch
    #[error("Sequence number mismatch: expected {expected}, got {actual}")]
    SequenceMismatch { expected: u64, actual: u64 },

    /// Prior digest mismatch
    #[error("Prior digest mismatch: expected {expected}, got {actual}")]
    PriorDigestMismatch { expected: String, actual: String },

    /// Missing prior state for non-inception event
    #[error("Missing prior state for event at sn {sn}")]
    MissingPriorState { sn: u64 },

    /// Threshold not met
    #[error("Signature threshold not met: have {have}, need {need}")]
    ThresholdNotMet { have: usize, need: usize },

    /// Unknown event type
    #[error("Unknown event type: {0}")]
    UnknownEventType(String),

    /// Invalid threshold specification
    #[error("Invalid threshold: {0}")]
    InvalidThreshold(String),

    /// Duplicate event
    #[error("Duplicate event with digest {digest}")]
    DuplicateEvent { digest: String },

    /// Missing delegator approval
    #[error("Missing delegator approval for delegated event")]
    MissingDelegatorApproval,

    /// Invalid witness configuration
    #[error("Invalid witness configuration: {0}")]
    InvalidWitnessConfig(String),

    /// Key not found
    #[error("Key not found at index {index}")]
    KeyNotFound { index: usize },

    /// Serialization error
    #[error("Serialization error: {0}")]
    Serialization(String),
}

impl From<anyhow::Error> for CoreError {
    fn from(err: anyhow::Error) -> Self {
        CoreError::CesrParse(err.to_string())
    }
}

impl From<serde_json::Error> for CoreError {
    fn from(err: serde_json::Error) -> Self {
        CoreError::Serialization(err.to_string())
    }
}

/// Result type alias for core operations
pub type CoreResult<T> = Result<T, CoreError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = CoreError::SequenceMismatch {
            expected: 1,
            actual: 5,
        };
        assert_eq!(
            err.to_string(),
            "Sequence number mismatch: expected 1, got 5"
        );
    }

    #[test]
    fn test_threshold_not_met_error() {
        let err = CoreError::ThresholdNotMet { have: 1, need: 2 };
        assert_eq!(
            err.to_string(),
            "Signature threshold not met: have 1, need 2"
        );
    }

    #[test]
    fn test_error_equality() {
        let err1 = CoreError::DuplicateEvent {
            digest: "abc".to_string(),
        };
        let err2 = CoreError::DuplicateEvent {
            digest: "abc".to_string(),
        };
        assert_eq!(err1, err2);
    }
}
