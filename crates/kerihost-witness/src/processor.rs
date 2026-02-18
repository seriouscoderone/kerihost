//! Event processing pipeline

use crate::error::{WitnessError, WitnessResult};
use kerihost_core::{
    ConfidenceLevel, CoreError, EventValidator, HonestMetadata, KeyState, NontransferableReceipt,
    SignedEvent, ValidationResult,
};
use kerihost_db::{EscrowReason, WitnessDatabase};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Result of processing an event
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum ProcessResult {
    /// Event was accepted and stored
    Accepted {
        /// Receipt for the event (if witness has signer)
        receipt: Option<NontransferableReceipt>,
        /// Current key state
        state: KeyState,
    },
    /// Event was escrowed
    Escrowed {
        /// Reason for escrow
        reason: EscrowReason,
    },
    /// Event is a duplicate
    Duplicate,
}

impl ProcessResult {
    /// Get KERI-honest metadata for this result
    pub fn metadata(&self) -> HonestMetadata {
        match self {
            ProcessResult::Accepted { state, .. } => state.metadata.clone(),
            ProcessResult::Escrowed { .. } => HonestMetadata::new(ConfidenceLevel::LocalOnly, 0, 0),
            ProcessResult::Duplicate => HonestMetadata::new(ConfidenceLevel::LocalOnly, 0, 0),
        }
    }
}

/// Event processor
///
/// Handles the validation and storage of KERI events.
pub struct EventProcessor<D: WitnessDatabase> {
    db: Arc<D>,
    strict_validation: bool,
    /// Witness prefix for authorization checks
    witness_prefix: Option<String>,
}

impl<D: WitnessDatabase> EventProcessor<D> {
    /// Create new event processor
    pub fn new(db: Arc<D>, strict_validation: bool) -> Self {
        EventProcessor {
            db,
            strict_validation,
            witness_prefix: None,
        }
    }

    /// Create new event processor with witness prefix for authorization checks
    pub fn new_with_prefix(db: Arc<D>, strict_validation: bool, witness_prefix: String) -> Self {
        EventProcessor {
            db,
            strict_validation,
            witness_prefix: Some(witness_prefix),
        }
    }

    /// Process raw CESR bytes
    pub async fn process(&self, raw: &[u8]) -> WitnessResult<ProcessResult> {
        // Parse the event
        let signed_event = SignedEvent::from_cesr(raw).map_err(|e| {
            WitnessError::Validation(format!("Failed to parse event: {}", e))
        })?;

        self.process_signed_event(signed_event).await
    }

    /// Process a parsed signed event
    pub async fn process_signed_event(&self, event: SignedEvent) -> WitnessResult<ProcessResult> {
        let prefix = &event.event.prefix;
        let sn = event.event.sn;

        // Get current state
        let current_state = self.db.get_state(prefix).await?;

        // Validate the event
        let validation_result = if self.strict_validation {
            EventValidator::validate(&event, current_state.as_ref())
        } else {
            // Lenient validation - skip signature verification
            self.lenient_validate(&event, current_state.as_ref())
        };

        match validation_result {
            Ok(ValidationResult::Valid) => {
                // Check witness authorization for non-inception events
                if let Some(ref witness_prefix) = self.witness_prefix {
                    if sn > 0 {
                        // For non-inception: witness must be in the identifier's witness list
                        let authorized = if let Some(ref state) = current_state {
                            state.witnesses.contains(witness_prefix)
                        } else {
                            false
                        };
                        if !authorized {
                            return Err(WitnessError::Validation(format!(
                                "Witness {} is not authorized for identifier {}",
                                witness_prefix, prefix
                            )));
                        }
                    } else {
                        // For inception: witness must be in the event's witness list
                        if !event.event.witnesses.contains(witness_prefix) {
                            return Err(WitnessError::Validation(format!(
                                "Witness {} is not in inception witness list for {}",
                                witness_prefix, prefix
                            )));
                        }
                    }
                }

                // Store the event
                self.db.append_event(&event).await?;

                // Compute new state
                let new_state = if sn == 0 {
                    KeyState::from_inception(&event.event)?
                } else {
                    let current = current_state
                        .ok_or_else(|| WitnessError::Validation("Missing prior state".to_string()))?;
                    current.apply(&event.event)?
                };

                // Store the new state
                self.db.put_state(&new_state).await?;

                Ok(ProcessResult::Accepted {
                    receipt: None, // Witness generates receipt separately
                    state: new_state,
                })
            }
            Ok(ValidationResult::OutOfOrder { .. }) => {
                // Escrow the event
                self.db
                    .escrow_event(&event, EscrowReason::OutOfOrder)
                    .await?;

                Ok(ProcessResult::Escrowed {
                    reason: EscrowReason::OutOfOrder,
                })
            }
            Ok(ValidationResult::PartiallySigned { .. }) => {
                // Escrow the event
                self.db
                    .escrow_event(&event, EscrowReason::PartiallySigned)
                    .await?;

                Ok(ProcessResult::Escrowed {
                    reason: EscrowReason::PartiallySigned,
                })
            }
            Ok(ValidationResult::MissingDelegator) => {
                // Escrow the event
                self.db
                    .escrow_event(&event, EscrowReason::MissingDelegator)
                    .await?;

                Ok(ProcessResult::Escrowed {
                    reason: EscrowReason::MissingDelegator,
                })
            }
            Ok(ValidationResult::Duplicate) => Ok(ProcessResult::Duplicate),
            Err(e) => Err(WitnessError::Validation(e.to_string())),
        }
    }

    /// Lenient validation (skip signature verification)
    fn lenient_validate(
        &self,
        event: &SignedEvent,
        current_state: Option<&KeyState>,
    ) -> Result<ValidationResult, CoreError> {
        // Check if this is an inception event
        if event.event.sn == 0 {
            if current_state.is_some() {
                return Ok(ValidationResult::Duplicate);
            }

            // Basic inception validation
            if event.event.prior_digest.is_some() {
                return Err(CoreError::InvalidEvent(
                    "Inception event should not have prior digest".to_string(),
                ));
            }

            return Ok(ValidationResult::Valid);
        }

        // Non-inception requires prior state
        let state = match current_state {
            Some(s) => s,
            None => {
                return Ok(ValidationResult::OutOfOrder {
                    expected_sn: 0,
                    actual_sn: event.event.sn,
                })
            }
        };

        // Check sequence number
        let expected_sn = state.sn + 1;
        if event.event.sn != expected_sn {
            if event.event.sn > expected_sn {
                return Ok(ValidationResult::OutOfOrder {
                    expected_sn,
                    actual_sn: event.event.sn,
                });
            } else {
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

        Ok(ValidationResult::Valid)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kerihost_core::{EventType, IndexedSignature, KeyEvent, Threshold};
    use kerihost_db::InMemoryDatabase;

    fn create_test_db() -> Arc<InMemoryDatabase> {
        Arc::new(InMemoryDatabase::new())
    }

    fn create_test_event(prefix: &str, sn: u64, prior: Option<String>) -> SignedEvent {
        let event = KeyEvent {
            prefix: prefix.to_string(),
            sn,
            event_type: if sn == 0 {
                EventType::Icp
            } else {
                EventType::Ixn
            },
            prior_digest: prior,
            signing_keys: vec!["DKey1234567890123456789012345678901234567890123".to_string()],
            signing_threshold: Threshold::simple(1),
            next_key_digest: Some("ENext123".to_string()),
            witness_threshold: Threshold::simple(1),
            witnesses: vec!["BTest123".to_string()],
            anchors: vec![],
            witnesses_remove: vec![],
            witnesses_add: vec![],
            delegator: None,
            raw: b"test event data".to_vec(),
            digest: format!("EDigest{}_{}", prefix, sn),
        };

        SignedEvent {
            event,
            signatures: vec![IndexedSignature {
                index: 0,
                signature: "AASig".to_string(),
            }],
        }
    }

    #[tokio::test]
    async fn test_processor_inception() {
        let db = create_test_db();
        let processor = EventProcessor::new(db.clone(), false); // Lenient mode

        let event = create_test_event("DTest123", 0, None);
        let result = processor.process_signed_event(event).await.unwrap();

        match result {
            ProcessResult::Accepted { state, .. } => {
                assert_eq!(state.prefix, "DTest123");
                assert_eq!(state.sn, 0);
            }
            _ => panic!("Expected Accepted"),
        }
    }

    #[tokio::test]
    async fn test_processor_subsequent_event() {
        let db = create_test_db();
        let processor = EventProcessor::new(db.clone(), false);

        // First, process inception
        let icp = create_test_event("DTest123", 0, None);
        processor.process_signed_event(icp.clone()).await.unwrap();

        // Then process interaction
        let ixn = create_test_event("DTest123", 1, Some(icp.event.digest.clone()));
        let result = processor.process_signed_event(ixn).await.unwrap();

        match result {
            ProcessResult::Accepted { state, .. } => {
                assert_eq!(state.sn, 1);
            }
            _ => panic!("Expected Accepted"),
        }
    }

    #[tokio::test]
    async fn test_processor_out_of_order() {
        let db = create_test_db();
        let processor = EventProcessor::new(db.clone(), false);

        // Process event at sn=5 without prior events
        let event = create_test_event("DTest123", 5, Some("EPrior".to_string()));
        let result = processor.process_signed_event(event).await.unwrap();

        match result {
            ProcessResult::Escrowed { reason } => {
                assert_eq!(reason, EscrowReason::OutOfOrder);
            }
            _ => panic!("Expected Escrowed"),
        }
    }

    #[tokio::test]
    async fn test_processor_duplicate_inception() {
        let db = create_test_db();
        let processor = EventProcessor::new(db.clone(), false);

        let icp = create_test_event("DTest123", 0, None);

        // First should succeed
        let result1 = processor.process_signed_event(icp.clone()).await.unwrap();
        assert!(matches!(result1, ProcessResult::Accepted { .. }));

        // Second should be duplicate
        let result2 = processor.process_signed_event(icp).await.unwrap();
        assert!(matches!(result2, ProcessResult::Duplicate));
    }

    #[tokio::test]
    async fn test_processor_wrong_prior_digest() {
        let db = create_test_db();
        let processor = EventProcessor::new(db.clone(), false);

        // Process inception
        let icp = create_test_event("DTest123", 0, None);
        processor.process_signed_event(icp).await.unwrap();

        // Try to process with wrong prior digest
        let ixn = create_test_event("DTest123", 1, Some("EWrongDigest".to_string()));
        let result = processor.process_signed_event(ixn).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_processor_old_event_duplicate() {
        let db = create_test_db();
        let processor = EventProcessor::new(db.clone(), false);

        // Process inception and one interaction
        let icp = create_test_event("DTest123", 0, None);
        processor.process_signed_event(icp.clone()).await.unwrap();

        let ixn = create_test_event("DTest123", 1, Some(icp.event.digest.clone()));
        processor.process_signed_event(ixn.clone()).await.unwrap();

        // Try to resubmit old inception
        let result = processor.process_signed_event(icp).await.unwrap();
        assert!(matches!(result, ProcessResult::Duplicate));
    }

    #[tokio::test]
    async fn test_process_result_metadata() {
        let state = KeyState {
            prefix: "DTest123".to_string(),
            sn: 0,
            latest_digest: "EDigest".to_string(),
            signing_keys: vec![],
            signing_threshold: Threshold::simple(1),
            next_key_digest: None,
            witnesses: vec![],
            witness_threshold: Threshold::simple(0),
            delegator: None,
            config: vec![],
            transferable: true,
            metadata: HonestMetadata::local_only(0),
        };

        let result = ProcessResult::Accepted {
            receipt: None,
            state: state.clone(),
        };

        let meta = result.metadata();
        assert_eq!(meta.confidence, ConfidenceLevel::LocalOnly);

        let escrowed = ProcessResult::Escrowed {
            reason: EscrowReason::OutOfOrder,
        };
        let meta = escrowed.metadata();
        assert_eq!(meta.confidence, ConfidenceLevel::LocalOnly);
    }
}
