//! Core Witness implementation

use crate::config::WitnessConfig;
use crate::error::{WitnessError, WitnessResult};
use crate::processor::{EventProcessor, ProcessResult};
use cesride::{Matter, Signer};
use kerihost_core::{KeyState, NontransferableReceipt, SignedEvent};
use kerihost_db::{EscrowedEvent, WitnessDatabase};
use std::sync::Arc;

/// KERI Witness
///
/// Processes key events, validates them, stores them, and issues receipts.
pub struct Witness<D: WitnessDatabase> {
    /// Witness identifier prefix
    pub prefix: String,
    /// Witness signing key
    signer: Option<Signer>,
    /// Database
    db: Arc<D>,
    /// Configuration
    config: WitnessConfig,
    /// Event processor
    processor: EventProcessor<D>,
}

impl<D: WitnessDatabase> Witness<D> {
    /// Create new witness
    pub fn new(signer: Option<Signer>, db: Arc<D>, config: WitnessConfig) -> Self {
        let prefix = if let Some(ref s) = signer {
            s.verfer().qb64().unwrap_or_default()
        } else {
            config.prefix.clone()
        };

        let processor = EventProcessor::new_with_prefix(
            Arc::clone(&db),
            config.strict_validation,
            prefix.clone(),
        );

        Witness {
            prefix,
            signer,
            db,
            config,
            processor,
        }
    }

    /// Create witness from seed
    pub fn from_seed(seed: &[u8], db: Arc<D>, config: WitnessConfig) -> WitnessResult<Self> {
        let signer = Signer::new_with_raw(seed, Some(false), None)
            .map_err(|e| WitnessError::Cesr(e.to_string()))?;

        Ok(Self::new(Some(signer), db, config))
    }

    /// Process incoming event notice
    ///
    /// Returns a ProcessResult indicating whether the event was accepted,
    /// escrowed, or was a duplicate.
    pub async fn process_notice(&self, raw: &[u8]) -> WitnessResult<ProcessResult> {
        self.processor.process(raw).await
    }

    /// Generate receipt for a signed event
    pub fn generate_receipt(&self, event: &SignedEvent) -> WitnessResult<NontransferableReceipt> {
        let signer = self.signer.as_ref().ok_or(WitnessError::MissingSigner)?;

        // Sign the event raw bytes
        let cigar = signer
            .sign_unindexed(&event.event.raw)
            .map_err(|e| WitnessError::Cesr(e.to_string()))?;

        let signature = cigar
            .qb64()
            .map_err(|e| WitnessError::Cesr(e.to_string()))?;

        Ok(NontransferableReceipt {
            event_digest: event.event.digest.clone(),
            event_sn: event.event.sn,
            event_prefix: event.event.prefix.clone(),
            witness_prefix: self.prefix.clone(),
            signature,
        })
    }

    /// Get key state for an identifier
    pub async fn get_state(&self, prefix: &str) -> WitnessResult<Option<KeyState>> {
        let state = self.db.get_state(prefix).await?;

        // If we have state, enrich it with receipt count
        if let Some(mut state) = state {
            let receipt_count = self.db.count_receipts(&state.latest_digest).await?;
            state = state.with_receipts(receipt_count as u32);
            Ok(Some(state))
        } else {
            Ok(None)
        }
    }

    /// Get KEL events for an identifier
    pub async fn get_kel(
        &self,
        prefix: &str,
        start: u64,
        end: Option<u64>,
    ) -> WitnessResult<Vec<SignedEvent>> {
        Ok(self.db.get_events(prefix, start, end).await?)
    }

    /// Get receipts for an event
    pub async fn get_receipts(
        &self,
        event_digest: &str,
    ) -> WitnessResult<Vec<NontransferableReceipt>> {
        Ok(self.db.get_receipts(event_digest).await?)
    }

    /// Get OOBI URL for this witness
    pub fn oobi_url(&self) -> String {
        format!("{}/oobi/{}", self.config.public_url, self.prefix)
    }

    /// Get introduction URL
    pub fn introduce_url(&self) -> String {
        format!("{}/introduce", self.config.public_url)
    }

    /// Check if witness is authorized for an identifier
    pub async fn is_authorized(&self, prefix: &str) -> WitnessResult<bool> {
        // Get the state for the identifier
        let state = self.db.get_state(prefix).await?;

        match state {
            Some(s) => {
                // Check if this witness is in the witness list
                Ok(s.witnesses.contains(&self.prefix))
            }
            None => {
                // No state yet - any witness can process inception
                Ok(true)
            }
        }
    }

    /// Get all escrowed events (for scheduled processing)
    pub async fn get_all_escrowed(&self) -> WitnessResult<Vec<EscrowedEvent>> {
        Ok(self.db.get_all_escrowed().await?)
    }

    /// Check if an escrowed event can be promoted
    pub async fn can_promote(&self, escrowed: &EscrowedEvent) -> WitnessResult<bool> {
        use kerihost_db::EscrowReason;

        match escrowed.reason {
            EscrowReason::OutOfOrder => {
                // Check if prior event now exists
                if escrowed.event.event.sn == 0 {
                    return Ok(true); // Inception never out of order
                }

                let prior_sn = escrowed.event.event.sn - 1;
                let prior = self
                    .db
                    .get_event(&escrowed.event.event.prefix, prior_sn)
                    .await?;
                Ok(prior.is_some())
            }
            EscrowReason::PartiallySigned => {
                // Would need to check if more signatures arrived
                // For now, can't promote
                Ok(false)
            }
            EscrowReason::MissingDelegator => {
                // Would need to check delegator KEL
                // For now, can't promote
                Ok(false)
            }
            EscrowReason::MissingReceipts => {
                // Check receipt count
                let count = self
                    .db
                    .count_receipts(&escrowed.event.event.digest)
                    .await?;
                // Assuming threshold of 1 for now
                Ok(count >= 1)
            }
        }
    }

    /// Promote an escrowed event
    pub async fn promote_escrowed(&self, escrowed: &EscrowedEvent) -> WitnessResult<ProcessResult> {
        // Remove from escrow
        let event = self
            .db
            .promote_escrowed(&escrowed.event.event.digest)
            .await?;

        if let Some(event) = event {
            // Re-process the event
            self.processor.process_signed_event(event).await
        } else {
            Ok(ProcessResult::Duplicate)
        }
    }

    /// Remove an escrowed event
    pub async fn remove_escrowed(&self, event_digest: &str) -> WitnessResult<()> {
        Ok(self.db.remove_escrowed(event_digest).await?)
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

    fn create_test_config() -> WitnessConfig {
        WitnessConfig::new("BTest123".to_string(), "https://test.keri.host".to_string())
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
    async fn test_witness_new() {
        let db = create_test_db();
        let config = create_test_config();
        let witness: Witness<InMemoryDatabase> = Witness::new(None, db, config);

        assert_eq!(witness.prefix, "BTest123");
    }

    #[tokio::test]
    async fn test_witness_oobi_url() {
        let db = create_test_db();
        let config = WitnessConfig::new("BTest123".to_string(), "https://witness.example.com".to_string());
        let witness: Witness<InMemoryDatabase> = Witness::new(None, db, config);

        assert_eq!(
            witness.oobi_url(),
            "https://witness.example.com/oobi/BTest123"
        );
    }

    #[tokio::test]
    async fn test_witness_introduce_url() {
        let db = create_test_db();
        let config = WitnessConfig::new("BTest123".to_string(), "https://witness.example.com".to_string());
        let witness: Witness<InMemoryDatabase> = Witness::new(None, db, config);

        assert_eq!(
            witness.introduce_url(),
            "https://witness.example.com/introduce"
        );
    }

    #[tokio::test]
    async fn test_witness_get_state_not_found() {
        let db = create_test_db();
        let config = create_test_config();
        let witness: Witness<InMemoryDatabase> = Witness::new(None, db, config);

        let state = witness.get_state("DNotExist").await.unwrap();
        assert!(state.is_none());
    }

    #[tokio::test]
    async fn test_witness_is_authorized_no_state() {
        let db = create_test_db();
        let config = create_test_config();
        let witness: Witness<InMemoryDatabase> = Witness::new(None, db, config);

        // No state yet - should be authorized for any prefix
        let authorized = witness.is_authorized("DNewPrefix").await.unwrap();
        assert!(authorized);
    }

    #[tokio::test]
    async fn test_witness_get_kel_empty() {
        let db = create_test_db();
        let config = create_test_config();
        let witness: Witness<InMemoryDatabase> = Witness::new(None, db, config);

        let kel = witness.get_kel("DNotExist", 0, None).await.unwrap();
        assert!(kel.is_empty());
    }

    #[tokio::test]
    async fn test_witness_get_receipts_empty() {
        let db = create_test_db();
        let config = create_test_config();
        let witness: Witness<InMemoryDatabase> = Witness::new(None, db, config);

        let receipts = witness.get_receipts("ENotExist").await.unwrap();
        assert!(receipts.is_empty());
    }

    #[tokio::test]
    async fn test_witness_generate_receipt_no_signer() {
        let db = create_test_db();
        let config = create_test_config();
        let witness: Witness<InMemoryDatabase> = Witness::new(None, db, config);

        let event = create_test_event("DTest", 0, None);
        let result = witness.generate_receipt(&event);

        assert!(matches!(result, Err(WitnessError::MissingSigner)));
    }
}
