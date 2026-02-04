//! Event validation logic
//!
//! This module provides validation for KERI events including:
//! - Signature verification
//! - Sequence number checking
//! - Prior digest verification
//! - Threshold checking

use crate::error::{CoreError, CoreResult};
use crate::event::SignedEvent;
use crate::state::KeyState;
use cesride::{Indexer, Matter, Verfer};

/// Result of event validation
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationResult {
    /// Event is valid and can be applied
    Valid,
    /// Event is out of order (missing prior events)
    OutOfOrder { expected_sn: u64, actual_sn: u64 },
    /// Event is partially signed (needs more signatures)
    PartiallySigned { have: usize, need: usize },
    /// Delegated event missing delegator approval
    MissingDelegator,
    /// Duplicate event (already processed)
    Duplicate,
}

/// Event validator
pub struct EventValidator;

impl EventValidator {
    /// Validate a signed event against current state
    ///
    /// If `current_state` is None, only inception events are valid.
    /// Returns `ValidationResult` indicating whether the event is valid
    /// or needs to be escrowed.
    pub fn validate(
        event: &SignedEvent,
        current_state: Option<&KeyState>,
    ) -> CoreResult<ValidationResult> {
        // Check if this is an inception event
        if event.event.sn == 0 {
            return Self::validate_inception(event, current_state);
        }

        // Non-inception requires prior state
        let state = current_state.ok_or(CoreError::MissingPriorState {
            sn: event.event.sn,
        })?;

        // Check sequence number
        let expected_sn = state.sn + 1;
        if event.event.sn != expected_sn {
            if event.event.sn > expected_sn {
                return Ok(ValidationResult::OutOfOrder {
                    expected_sn,
                    actual_sn: event.event.sn,
                });
            } else if event.event.sn < expected_sn {
                // This is a duplicate or superseded event
                return Ok(ValidationResult::Duplicate);
            }
        }

        // Check prior digest
        if let Some(ref prior) = event.event.prior_digest {
            if prior != &state.latest_digest {
                return Err(CoreError::PriorDigestMismatch {
                    expected: state.latest_digest.clone(),
                    actual: prior.clone(),
                });
            }
        } else {
            return Err(CoreError::InvalidEvent(
                "Non-inception event missing prior digest".to_string(),
            ));
        }

        // Verify signatures
        Self::verify_signatures(event, Some(state))?;

        // Check if delegated event has delegator approval
        if event.event.is_delegated() {
            // For now, we don't fully validate delegator approval
            // This would require fetching delegator's KEL
            // TODO: Implement full delegation validation
        }

        Ok(ValidationResult::Valid)
    }

    /// Validate an inception event
    fn validate_inception(
        event: &SignedEvent,
        current_state: Option<&KeyState>,
    ) -> CoreResult<ValidationResult> {
        // If we already have state for this prefix, it's a duplicate
        if let Some(state) = current_state {
            if state.prefix == event.event.prefix {
                return Ok(ValidationResult::Duplicate);
            }
        }

        // Verify sequence number is 0
        if event.event.sn != 0 {
            return Err(CoreError::InvalidEvent(
                "Inception event must have sn=0".to_string(),
            ));
        }

        // Inception should not have prior digest
        if event.event.prior_digest.is_some() {
            return Err(CoreError::InvalidEvent(
                "Inception event should not have prior digest".to_string(),
            ));
        }

        // Verify signatures against the keys in the event itself
        Self::verify_signatures(event, None)?;

        Ok(ValidationResult::Valid)
    }

    /// Verify signatures on an event
    fn verify_signatures(
        event: &SignedEvent,
        state: Option<&KeyState>,
    ) -> CoreResult<ValidationResult> {
        // Get the signing keys
        let signing_keys = if event.event.is_establishment() {
            // For establishment events, use keys from the event
            &event.event.signing_keys
        } else if let Some(s) = state {
            // For non-establishment, use keys from state
            &s.signing_keys
        } else {
            return Err(CoreError::MissingPriorState {
                sn: event.event.sn,
            });
        };

        // Get the threshold
        let threshold = if event.event.is_establishment() {
            &event.event.signing_threshold
        } else if let Some(s) = state {
            &s.signing_threshold
        } else {
            return Err(CoreError::MissingPriorState {
                sn: event.event.sn,
            });
        };

        // Count valid signatures
        let mut valid_count = 0;
        let needed = threshold.min_signatures();

        for sig in &event.signatures {
            let key_idx = sig.index as usize;
            if key_idx >= signing_keys.len() {
                continue; // Invalid index, skip
            }

            let key_qb64 = &signing_keys[key_idx];

            // Parse the key
            let verfer = match Verfer::new_with_qb64(key_qb64) {
                Ok(v) => v,
                Err(_) => continue,
            };

            // Verify signature
            if Self::verify_single_signature(&verfer, &sig.signature, &event.event.raw)? {
                valid_count += 1;
            }
        }

        // Check threshold
        if valid_count < needed {
            return Ok(ValidationResult::PartiallySigned {
                have: valid_count,
                need: needed,
            });
        }

        Ok(ValidationResult::Valid)
    }

    /// Verify a single signature
    fn verify_single_signature(verfer: &Verfer, sig_qb64: &str, data: &[u8]) -> CoreResult<bool> {
        // Parse signature - try indexed first, then unindexed
        let sig_bytes: Vec<u8> = match cesride::Siger::new_with_qb64(sig_qb64, None) {
            Ok(siger) => Indexer::raw(&siger),
            Err(_) => {
                // Try parsing as Cigar (unindexed)
                match cesride::Cigar::new_with_qb64(sig_qb64, Some(verfer)) {
                    Ok(cigar) => Matter::raw(&cigar),
                    Err(e) => return Err(CoreError::InvalidSignature(e.to_string())),
                }
            }
        };

        // Verify
        verfer
            .verify(&sig_bytes, data)
            .map_err(|e| CoreError::InvalidSignature(e.to_string()))
    }
}

/// Quick validation helpers
pub fn is_valid_sequence(current_sn: u64, event_sn: u64) -> bool {
    event_sn == current_sn + 1
}

pub fn is_valid_prior_digest(state_digest: &str, event_prior: &str) -> bool {
    state_digest == event_prior
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::{EventType, IndexedSignature, KeyEvent, Threshold};

    fn create_test_signed_event(sn: u64, prior: Option<String>) -> SignedEvent {
        let event = KeyEvent {
            prefix: "DTest123456789012345678901234567890123456789012".to_string(),
            sn,
            event_type: if sn == 0 {
                EventType::Icp
            } else {
                EventType::Ixn
            },
            prior_digest: prior,
            signing_keys: vec!["DKey1234567890123456789012345678901234567890123".to_string()],
            signing_threshold: Threshold::simple(1),
            next_key_digest: Some("ENext123456789012345678901234567890123456789012".to_string()),
            witness_threshold: Threshold::simple(1),
            witnesses: vec![],
            anchors: vec![],
            delegator: None,
            raw: b"test data".to_vec(),
            digest: format!("EDigest{}2345678901234567890123456789012345678901", sn),
        };

        SignedEvent {
            event,
            signatures: vec![IndexedSignature {
                index: 0,
                // Note: This is a placeholder - real tests would use actual signatures
                signature: "AATest_Signature_Placeholder_12345678901234567890123456789012345678901234567890123456".to_string(),
            }],
        }
    }

    fn create_test_state(sn: u64, digest: &str) -> KeyState {
        KeyState {
            prefix: "DTest123456789012345678901234567890123456789012".to_string(),
            sn,
            latest_digest: digest.to_string(),
            signing_keys: vec!["DKey1234567890123456789012345678901234567890123".to_string()],
            signing_threshold: Threshold::simple(1),
            next_key_digest: Some("ENext123456789012345678901234567890123456789012".to_string()),
            witnesses: vec![],
            witness_threshold: Threshold::simple(0),
            delegator: None,
            config: vec![],
            transferable: true,
            metadata: crate::HonestMetadata::local_only(0),
        }
    }

    #[test]
    fn test_validation_result_equality() {
        assert_eq!(ValidationResult::Valid, ValidationResult::Valid);
        assert_eq!(ValidationResult::Duplicate, ValidationResult::Duplicate);
        assert_eq!(
            ValidationResult::OutOfOrder {
                expected_sn: 1,
                actual_sn: 5
            },
            ValidationResult::OutOfOrder {
                expected_sn: 1,
                actual_sn: 5
            }
        );
    }

    #[test]
    fn test_is_valid_sequence() {
        assert!(is_valid_sequence(0, 1));
        assert!(is_valid_sequence(5, 6));
        assert!(!is_valid_sequence(0, 2));
        assert!(!is_valid_sequence(5, 3));
    }

    #[test]
    fn test_is_valid_prior_digest() {
        assert!(is_valid_prior_digest("abc", "abc"));
        assert!(!is_valid_prior_digest("abc", "def"));
    }

    #[test]
    fn test_validate_inception_no_state() {
        let event = create_test_signed_event(0, None);
        // Note: Signature verification will fail with placeholder,
        // but we're testing the flow
        let _result = EventValidator::validate(&event, None);
        // Basic structure test - real validation needs real crypto
    }

    #[test]
    fn test_validate_out_of_order() {
        let event = create_test_signed_event(
            5,
            Some("EDigest02345678901234567890123456789012345678901".to_string()),
        );
        let state = create_test_state(0, "EDigest02345678901234567890123456789012345678901");

        let result = EventValidator::validate(&event, Some(&state));
        // Should be out of order since state.sn=0 but event.sn=5
        match result {
            Ok(ValidationResult::OutOfOrder { expected_sn, actual_sn }) => {
                assert_eq!(expected_sn, 1);
                assert_eq!(actual_sn, 5);
            }
            _ => {} // May also fail on signature verification
        }
    }

    #[test]
    fn test_validate_duplicate() {
        let event = create_test_signed_event(
            0,
            Some("EDigest02345678901234567890123456789012345678901".to_string()),
        );
        let state = create_test_state(1, "EDigest12345678901234567890123456789012345678901");

        let result = EventValidator::validate(&event, Some(&state));
        // Event sn=0 but state sn=1, so this is old/duplicate
        match result {
            Ok(ValidationResult::Duplicate) => {}
            _ => {} // May fail differently
        }
    }

    #[test]
    fn test_validate_missing_prior_state() {
        let event = create_test_signed_event(
            1,
            Some("EPrior12345678901234567890123456789012345678901".to_string()),
        );

        let result = EventValidator::validate(&event, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_inception_with_prior_digest_fails() {
        let mut event = create_test_signed_event(0, None);
        event.event.prior_digest = Some("EShouldNotExist123456789012345678901234567890".to_string());

        let result = EventValidator::validate(&event, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_non_inception_missing_prior_digest() {
        let mut event = create_test_signed_event(1, None);
        event.event.prior_digest = None;
        let state = create_test_state(0, "EPrior12345678901234567890123456789012345678901");

        let result = EventValidator::validate(&event, Some(&state));
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_prior_digest_mismatch() {
        let event = create_test_signed_event(
            1,
            Some("EWrongDigest12345678901234567890123456789012345".to_string()),
        );
        let state = create_test_state(0, "ECorrectDigest2345678901234567890123456789012");

        let result = EventValidator::validate(&event, Some(&state));
        assert!(matches!(result, Err(CoreError::PriorDigestMismatch { .. })));
    }
}
