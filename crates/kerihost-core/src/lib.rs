//! kerihost-core: Core KERI types, validation, and state calculation
//!
//! This crate provides the foundation for KERI event processing:
//! - Event types (inception, rotation, interaction, delegation)
//! - Key state computation
//! - Event validation
//! - Receipt types
//!
//! # KERI-Honest Design
//!
//! This implementation follows KERI-honest principles:
//! - Never claim "FINAL" - use confidence levels instead
//! - Escrow is state, not error
//! - Cryptographic eventual finality
//! - Explicit confidence qualifiers

pub mod error;
pub mod event;
pub mod receipt;
pub mod state;
pub mod validation;

pub use error::*;
pub use event::*;
pub use receipt::*;
pub use state::*;
pub use validation::*;

/// KERI-honest confidence levels for state/responses
///
/// # Design Principle
/// There is no global clock, no global state, no global finality in KERI.
/// We NEVER claim "FINAL" - that would be a lie.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ConfidenceLevel {
    /// Event stored locally, no witness receipts yet
    LocalOnly,
    /// Witness receipt threshold met for this event
    ReceiptThresholdMet,
    // NOTE: Never "Final" - that's a lie in KERI
}

impl std::fmt::Display for ConfidenceLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfidenceLevel::LocalOnly => write!(f, "LOCAL_ONLY"),
            ConfidenceLevel::ReceiptThresholdMet => write!(f, "RECEIPT_THRESHOLD_MET"),
        }
    }
}

/// KERI-honest response metadata
///
/// Every API response that returns state MUST include these fields.
/// This is honest. Anything less is lying.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HonestMetadata {
    /// Current confidence level
    pub confidence: ConfidenceLevel,
    /// Number of witness receipts seen
    pub witnesses_seen: u32,
    /// Number of witness receipts required (threshold)
    pub witnesses_required: u32,
    /// ISO 8601 timestamp of this view
    pub as_of: String,
}

impl HonestMetadata {
    /// Create new metadata with current timestamp
    pub fn new(confidence: ConfidenceLevel, witnesses_seen: u32, witnesses_required: u32) -> Self {
        Self {
            confidence,
            witnesses_seen,
            witnesses_required,
            as_of: chrono::Utc::now().to_rfc3339(),
        }
    }

    /// Create LOCAL_ONLY metadata (just accepted, no receipts)
    pub fn local_only(witnesses_required: u32) -> Self {
        Self::new(ConfidenceLevel::LocalOnly, 1, witnesses_required)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_confidence_level_serialization() {
        let local = ConfidenceLevel::LocalOnly;
        let json = serde_json::to_string(&local).unwrap();
        assert_eq!(json, "\"LOCAL_ONLY\"");

        let threshold = ConfidenceLevel::ReceiptThresholdMet;
        let json = serde_json::to_string(&threshold).unwrap();
        assert_eq!(json, "\"RECEIPT_THRESHOLD_MET\"");
    }

    #[test]
    fn test_honest_metadata_serialization() {
        let meta = HonestMetadata::new(ConfidenceLevel::LocalOnly, 1, 2);
        let json = serde_json::to_string(&meta).unwrap();
        assert!(json.contains("\"confidence\":\"LOCAL_ONLY\""));
        assert!(json.contains("\"witnessesSeen\":1"));
        assert!(json.contains("\"witnessesRequired\":2"));
        assert!(json.contains("\"asOf\":"));
    }

    #[test]
    fn test_confidence_level_display() {
        assert_eq!(ConfidenceLevel::LocalOnly.to_string(), "LOCAL_ONLY");
        assert_eq!(
            ConfidenceLevel::ReceiptThresholdMet.to_string(),
            "RECEIPT_THRESHOLD_MET"
        );
    }
}
