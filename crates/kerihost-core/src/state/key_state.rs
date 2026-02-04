//! Key state representation
//!
//! KeyState represents the current cryptographic state of an identifier,
//! derived from processing the Key Event Log (KEL).

use crate::error::{CoreError, CoreResult};
use crate::event::{EventType, KeyEvent, Threshold};
use crate::{ConfidenceLevel, HonestMetadata};
use serde::{Deserialize, Serialize};

/// Current key state of an identifier
///
/// This is derived from processing all events in the KEL up to a certain point.
/// The state includes all information needed to validate future events.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyState {
    /// Identifier prefix
    pub prefix: String,

    /// Latest sequence number
    pub sn: u64,

    /// Digest of latest event
    pub latest_digest: String,

    /// Current signing keys (qb64 strings)
    pub signing_keys: Vec<String>,

    /// Current signing threshold
    pub signing_threshold: Threshold,

    /// Next key commitment (digest)
    pub next_key_digest: Option<String>,

    /// Current witnesses
    pub witnesses: Vec<String>,

    /// Witness threshold
    pub witness_threshold: Threshold,

    /// Delegator prefix (for delegated identifiers)
    pub delegator: Option<String>,

    /// Configuration traits
    pub config: Vec<String>,

    /// Whether this identifier is transferable
    pub transferable: bool,

    /// KERI-honest metadata
    #[serde(flatten)]
    pub metadata: HonestMetadata,
}

impl KeyState {
    /// Create initial state from inception event
    pub fn from_inception(event: &KeyEvent) -> CoreResult<Self> {
        if event.event_type != EventType::Icp && event.event_type != EventType::Dip {
            return Err(CoreError::InvalidEvent(
                "Expected inception event".to_string(),
            ));
        }

        if event.sn != 0 {
            return Err(CoreError::SequenceMismatch {
                expected: 0,
                actual: event.sn,
            });
        }

        let witness_threshold = match &event.witness_threshold {
            Threshold::Simple(n) => *n,
            Threshold::Weighted(_) => 1, // Conservative
        };

        Ok(KeyState {
            prefix: event.prefix.clone(),
            sn: event.sn,
            latest_digest: event.digest.clone(),
            signing_keys: event.signing_keys.clone(),
            signing_threshold: event.signing_threshold.clone(),
            next_key_digest: event.next_key_digest.clone(),
            witnesses: event.witnesses.clone(),
            witness_threshold: event.witness_threshold.clone(),
            delegator: event.delegator.clone(),
            config: vec![],
            transferable: event.next_key_digest.is_some(),
            metadata: HonestMetadata::local_only(witness_threshold),
        })
    }

    /// Apply an event to produce new state
    ///
    /// This validates that the event can be applied and produces the new state.
    /// Returns an error if the event cannot be applied to this state.
    pub fn apply(&self, event: &KeyEvent) -> CoreResult<KeyState> {
        // Verify prefix matches
        if event.prefix != self.prefix {
            return Err(CoreError::InvalidEvent(format!(
                "Prefix mismatch: expected {}, got {}",
                self.prefix, event.prefix
            )));
        }

        // Verify sequence number
        let expected_sn = self.sn + 1;
        if event.sn != expected_sn {
            return Err(CoreError::SequenceMismatch {
                expected: expected_sn,
                actual: event.sn,
            });
        }

        // Verify prior digest
        if let Some(ref prior) = event.prior_digest {
            if prior != &self.latest_digest {
                return Err(CoreError::PriorDigestMismatch {
                    expected: self.latest_digest.clone(),
                    actual: prior.clone(),
                });
            }
        } else {
            return Err(CoreError::InvalidEvent(
                "Non-inception event missing prior digest".to_string(),
            ));
        }

        // Apply based on event type
        match event.event_type {
            EventType::Rot | EventType::Drt => self.apply_rotation(event),
            EventType::Ixn => self.apply_interaction(event),
            EventType::Icp | EventType::Dip => Err(CoreError::InvalidEvent(
                "Cannot apply inception to existing state".to_string(),
            )),
        }
    }

    /// Apply rotation event
    fn apply_rotation(&self, event: &KeyEvent) -> CoreResult<KeyState> {
        let witness_threshold = match &event.witness_threshold {
            Threshold::Simple(n) => *n,
            Threshold::Weighted(_) => 1,
        };

        Ok(KeyState {
            prefix: self.prefix.clone(),
            sn: event.sn,
            latest_digest: event.digest.clone(),
            signing_keys: event.signing_keys.clone(),
            signing_threshold: event.signing_threshold.clone(),
            next_key_digest: event.next_key_digest.clone(),
            witnesses: self.apply_witness_changes(event),
            witness_threshold: event.witness_threshold.clone(),
            delegator: self.delegator.clone(),
            config: self.config.clone(),
            transferable: event.next_key_digest.is_some(),
            metadata: HonestMetadata::local_only(witness_threshold),
        })
    }

    /// Apply interaction event (only updates sn and digest)
    fn apply_interaction(&self, event: &KeyEvent) -> CoreResult<KeyState> {
        let witness_threshold = match &self.witness_threshold {
            Threshold::Simple(n) => *n,
            Threshold::Weighted(_) => 1,
        };

        Ok(KeyState {
            prefix: self.prefix.clone(),
            sn: event.sn,
            latest_digest: event.digest.clone(),
            signing_keys: self.signing_keys.clone(),
            signing_threshold: self.signing_threshold.clone(),
            next_key_digest: self.next_key_digest.clone(),
            witnesses: self.witnesses.clone(),
            witness_threshold: self.witness_threshold.clone(),
            delegator: self.delegator.clone(),
            config: self.config.clone(),
            transferable: self.transferable,
            metadata: HonestMetadata::local_only(witness_threshold),
        })
    }

    /// Apply witness changes from rotation event
    fn apply_witness_changes(&self, event: &KeyEvent) -> Vec<String> {
        // For simplicity, rotation events carry the full witness list
        // In full KERI, we'd need to parse br (remove) and ba (add)
        event.witnesses.clone()
    }

    /// Update metadata with receipt information
    pub fn with_receipts(mut self, witnesses_seen: u32) -> Self {
        let required = match &self.witness_threshold {
            Threshold::Simple(n) => *n,
            Threshold::Weighted(_) => 1,
        };

        self.metadata.witnesses_seen = witnesses_seen;
        self.metadata.witnesses_required = required;

        if witnesses_seen >= required {
            self.metadata.confidence = ConfidenceLevel::ReceiptThresholdMet;
        }

        self
    }

    /// Check if state has met witness threshold
    pub fn has_threshold_receipts(&self) -> bool {
        self.metadata.confidence == ConfidenceLevel::ReceiptThresholdMet
    }

    /// Get minimum signatures needed
    pub fn min_signatures(&self) -> usize {
        self.signing_threshold.min_signatures()
    }

    /// Check if identifier is delegated
    pub fn is_delegated(&self) -> bool {
        self.delegator.is_some()
    }
}

/// State along with its provenance (for tracking)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StateWithProvenance {
    /// The key state
    #[serde(flatten)]
    pub state: KeyState,

    /// Events processed to derive this state
    pub event_digests: Vec<String>,
}

impl StateWithProvenance {
    /// Create from state and event chain
    pub fn new(state: KeyState, event_digests: Vec<String>) -> Self {
        StateWithProvenance {
            state,
            event_digests,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::{InceptionParams, KeyEvent};

    fn create_test_inception_event() -> KeyEvent {
        KeyEvent {
            prefix: "DTest123456789012345678901234567890123456789012".to_string(),
            sn: 0,
            event_type: EventType::Icp,
            prior_digest: None,
            signing_keys: vec!["DKey1234567890123456789012345678901234567890123".to_string()],
            signing_threshold: Threshold::simple(1),
            next_key_digest: Some("ENext123456789012345678901234567890123456789012".to_string()),
            witness_threshold: Threshold::simple(1),
            witnesses: vec!["BWit1234567890123456789012345678901234567890123".to_string()],
            anchors: vec![],
            delegator: None,
            raw: vec![],
            digest: "EDigest12345678901234567890123456789012345678901".to_string(),
        }
    }

    fn create_test_rotation_event(prior_digest: &str) -> KeyEvent {
        KeyEvent {
            prefix: "DTest123456789012345678901234567890123456789012".to_string(),
            sn: 1,
            event_type: EventType::Rot,
            prior_digest: Some(prior_digest.to_string()),
            signing_keys: vec!["DKey2234567890123456789012345678901234567890123".to_string()],
            signing_threshold: Threshold::simple(1),
            next_key_digest: Some("ENext223456789012345678901234567890123456789012".to_string()),
            witness_threshold: Threshold::simple(1),
            witnesses: vec!["BWit1234567890123456789012345678901234567890123".to_string()],
            anchors: vec![],
            delegator: None,
            raw: vec![],
            digest: "EDigest22345678901234567890123456789012345678901".to_string(),
        }
    }

    fn create_test_interaction_event(sn: u64, prior_digest: &str) -> KeyEvent {
        KeyEvent {
            prefix: "DTest123456789012345678901234567890123456789012".to_string(),
            sn,
            event_type: EventType::Ixn,
            prior_digest: Some(prior_digest.to_string()),
            signing_keys: vec![],
            signing_threshold: Threshold::simple(1),
            next_key_digest: None,
            witness_threshold: Threshold::simple(0),
            witnesses: vec![],
            anchors: vec![],
            delegator: None,
            raw: vec![],
            digest: format!("EDigest{}2345678901234567890123456789012345678901", sn),
        }
    }

    #[test]
    fn test_state_from_inception() {
        let event = create_test_inception_event();
        let state = KeyState::from_inception(&event).unwrap();

        assert_eq!(state.prefix, event.prefix);
        assert_eq!(state.sn, 0);
        assert_eq!(state.latest_digest, event.digest);
        assert_eq!(state.signing_keys.len(), 1);
        assert!(state.transferable);
        assert_eq!(state.metadata.confidence, ConfidenceLevel::LocalOnly);
    }

    #[test]
    fn test_state_from_inception_wrong_type() {
        let mut event = create_test_inception_event();
        event.event_type = EventType::Rot;

        let result = KeyState::from_inception(&event);
        assert!(result.is_err());
    }

    #[test]
    fn test_state_from_inception_wrong_sn() {
        let mut event = create_test_inception_event();
        event.sn = 1;

        let result = KeyState::from_inception(&event);
        assert!(matches!(result, Err(CoreError::SequenceMismatch { .. })));
    }

    #[test]
    fn test_state_apply_rotation() {
        let icp = create_test_inception_event();
        let state = KeyState::from_inception(&icp).unwrap();

        let rot = create_test_rotation_event(&icp.digest);
        let new_state = state.apply(&rot).unwrap();

        assert_eq!(new_state.sn, 1);
        assert_eq!(new_state.latest_digest, rot.digest);
        assert_ne!(new_state.signing_keys, state.signing_keys);
    }

    #[test]
    fn test_state_apply_interaction() {
        let icp = create_test_inception_event();
        let state = KeyState::from_inception(&icp).unwrap();

        let ixn = create_test_interaction_event(1, &icp.digest);
        let new_state = state.apply(&ixn).unwrap();

        assert_eq!(new_state.sn, 1);
        assert_eq!(new_state.latest_digest, ixn.digest);
        // Keys unchanged by interaction
        assert_eq!(new_state.signing_keys, state.signing_keys);
    }

    #[test]
    fn test_state_apply_wrong_sequence() {
        let icp = create_test_inception_event();
        let state = KeyState::from_inception(&icp).unwrap();

        let mut ixn = create_test_interaction_event(5, &icp.digest);
        ixn.sn = 5; // Wrong sequence

        let result = state.apply(&ixn);
        assert!(matches!(result, Err(CoreError::SequenceMismatch { .. })));
    }

    #[test]
    fn test_state_apply_wrong_prior_digest() {
        let icp = create_test_inception_event();
        let state = KeyState::from_inception(&icp).unwrap();

        let ixn = create_test_interaction_event(1, "EWrongDigest123456789012345678901234567890123");
        let result = state.apply(&ixn);
        assert!(matches!(result, Err(CoreError::PriorDigestMismatch { .. })));
    }

    #[test]
    fn test_state_apply_wrong_prefix() {
        let icp = create_test_inception_event();
        let state = KeyState::from_inception(&icp).unwrap();

        let mut ixn = create_test_interaction_event(1, &icp.digest);
        ixn.prefix = "DWrong12345678901234567890123456789012345678901".to_string();

        let result = state.apply(&ixn);
        assert!(result.is_err());
    }

    #[test]
    fn test_state_with_receipts() {
        let icp = create_test_inception_event();
        let state = KeyState::from_inception(&icp).unwrap();

        assert_eq!(state.metadata.confidence, ConfidenceLevel::LocalOnly);

        let state_with_receipts = state.with_receipts(2);
        assert_eq!(
            state_with_receipts.metadata.confidence,
            ConfidenceLevel::ReceiptThresholdMet
        );
        assert_eq!(state_with_receipts.metadata.witnesses_seen, 2);
    }

    #[test]
    fn test_state_has_threshold_receipts() {
        let icp = create_test_inception_event();
        let state = KeyState::from_inception(&icp).unwrap();

        assert!(!state.has_threshold_receipts());

        let state_with_receipts = state.with_receipts(1);
        assert!(state_with_receipts.has_threshold_receipts());
    }

    #[test]
    fn test_state_serialization() {
        let icp = create_test_inception_event();
        let state = KeyState::from_inception(&icp).unwrap();

        let json = serde_json::to_string(&state).unwrap();
        assert!(json.contains("\"prefix\":"));
        assert!(json.contains("\"sn\":"));
        assert!(json.contains("\"confidence\":\"LOCAL_ONLY\""));

        let parsed: KeyState = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.prefix, state.prefix);
        assert_eq!(parsed.sn, state.sn);
    }

    #[test]
    fn test_state_with_provenance() {
        let icp = create_test_inception_event();
        let state = KeyState::from_inception(&icp).unwrap();

        let provenance =
            StateWithProvenance::new(state.clone(), vec![icp.digest.clone()]);

        assert_eq!(provenance.event_digests.len(), 1);
        assert_eq!(provenance.state.prefix, state.prefix);
    }

    #[test]
    fn test_delegated_state() {
        let mut icp = create_test_inception_event();
        icp.event_type = EventType::Dip;
        icp.delegator = Some("DDelegator123456789012345678901234567890123456".to_string());

        let state = KeyState::from_inception(&icp).unwrap();
        assert!(state.is_delegated());
        assert_eq!(
            state.delegator,
            Some("DDelegator123456789012345678901234567890123456".to_string())
        );
    }

    #[test]
    fn test_non_transferable_state() {
        let mut icp = create_test_inception_event();
        icp.next_key_digest = None;

        let state = KeyState::from_inception(&icp).unwrap();
        assert!(!state.transferable);
    }
}
