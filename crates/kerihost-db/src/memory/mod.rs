//! In-memory database implementation for testing
//!
//! This implementation stores all data in memory using HashMaps.
//! It's useful for unit tests and local development.

use crate::error::{DbError, DbResult};
use crate::traits::{
    EscrowReason, EscrowStore, EscrowedEvent, KelStore, ReceiptStore, StateStore,
};
use async_trait::async_trait;
use kerihost_core::{KeyState, NontransferableReceipt, SignedEvent};
use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;
use tokio::sync::RwLock;

/// In-memory database for testing
pub struct InMemoryDatabase {
    /// KEL storage: prefix -> (sn -> event)
    kel: Arc<RwLock<HashMap<String, BTreeMap<u64, SignedEvent>>>>,
    /// State storage: prefix -> state
    states: Arc<RwLock<HashMap<String, KeyState>>>,
    /// Receipt storage: event_digest -> (witness_prefix -> receipt)
    receipts: Arc<RwLock<HashMap<String, HashMap<String, NontransferableReceipt>>>>,
    /// Escrow storage: digest -> escrowed_event
    escrows: Arc<RwLock<HashMap<String, EscrowedEvent>>>,
}

impl InMemoryDatabase {
    /// Create new empty in-memory database
    pub fn new() -> Self {
        InMemoryDatabase {
            kel: Arc::new(RwLock::new(HashMap::new())),
            states: Arc::new(RwLock::new(HashMap::new())),
            receipts: Arc::new(RwLock::new(HashMap::new())),
            escrows: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Clear all data (for testing)
    pub async fn clear(&self) {
        self.kel.write().await.clear();
        self.states.write().await.clear();
        self.receipts.write().await.clear();
        self.escrows.write().await.clear();
    }

    /// Get count of events for a prefix (for testing)
    pub async fn event_count(&self, prefix: &str) -> usize {
        let kel = self.kel.read().await;
        kel.get(prefix).map(|m| m.len()).unwrap_or(0)
    }
}

impl Default for InMemoryDatabase {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for InMemoryDatabase {
    fn clone(&self) -> Self {
        InMemoryDatabase {
            kel: Arc::clone(&self.kel),
            states: Arc::clone(&self.states),
            receipts: Arc::clone(&self.receipts),
            escrows: Arc::clone(&self.escrows),
        }
    }
}

#[async_trait]
impl KelStore for InMemoryDatabase {
    async fn append_event(&self, event: &SignedEvent) -> DbResult<()> {
        let mut kel = self.kel.write().await;
        let prefix = &event.event.prefix;
        let sn = event.event.sn;

        let prefix_kel = kel.entry(prefix.clone()).or_insert_with(BTreeMap::new);

        // For inception (sn=0), check that no events exist
        if sn == 0 {
            if !prefix_kel.is_empty() {
                return Err(DbError::Duplicate(format!(
                    "Inception already exists for {}",
                    prefix
                )));
            }
        } else {
            // For non-inception, verify prior digest
            let prior_sn = sn - 1;
            if let Some(prior_event) = prefix_kel.get(&prior_sn) {
                if let Some(ref event_prior) = event.event.prior_digest {
                    if event_prior != &prior_event.event.digest {
                        return Err(DbError::PriorDigestMismatch {
                            expected: prior_event.event.digest.clone(),
                            actual: event_prior.clone(),
                        });
                    }
                } else {
                    return Err(DbError::Other(
                        "Non-inception event missing prior digest".to_string(),
                    ));
                }
            } else {
                return Err(DbError::NotFound(format!(
                    "Prior event at sn {} not found for {}",
                    prior_sn, prefix
                )));
            }
        }

        // Check for duplicate
        if prefix_kel.contains_key(&sn) {
            return Err(DbError::Duplicate(format!(
                "Event at sn {} already exists for {}",
                sn, prefix
            )));
        }

        prefix_kel.insert(sn, event.clone());
        Ok(())
    }

    async fn get_event(&self, prefix: &str, sn: u64) -> DbResult<Option<SignedEvent>> {
        let kel = self.kel.read().await;
        Ok(kel.get(prefix).and_then(|m| m.get(&sn).cloned()))
    }

    async fn get_events(
        &self,
        prefix: &str,
        start_sn: u64,
        end_sn: Option<u64>,
    ) -> DbResult<Vec<SignedEvent>> {
        let kel = self.kel.read().await;

        let Some(prefix_kel) = kel.get(prefix) else {
            return Ok(vec![]);
        };

        let events: Vec<SignedEvent> = prefix_kel
            .range(start_sn..)
            .filter(|(sn, _)| end_sn.map(|e| **sn <= e).unwrap_or(true))
            .map(|(_, e)| e.clone())
            .collect();

        Ok(events)
    }

    async fn get_latest(&self, prefix: &str) -> DbResult<Option<SignedEvent>> {
        let kel = self.kel.read().await;
        Ok(kel
            .get(prefix)
            .and_then(|m| m.iter().last().map(|(_, e)| e.clone())))
    }

    async fn get_event_by_digest(&self, prefix: &str, digest: &str) -> DbResult<Option<SignedEvent>> {
        let kel = self.kel.read().await;
        Ok(kel.get(prefix).and_then(|m| {
            m.values()
                .find(|e| e.event.digest == digest)
                .cloned()
        }))
    }
}

#[async_trait]
impl StateStore for InMemoryDatabase {
    async fn get_state(&self, prefix: &str) -> DbResult<Option<KeyState>> {
        let states = self.states.read().await;
        Ok(states.get(prefix).cloned())
    }

    async fn put_state(&self, state: &KeyState) -> DbResult<()> {
        let mut states = self.states.write().await;
        states.insert(state.prefix.clone(), state.clone());
        Ok(())
    }

    async fn delete_state(&self, prefix: &str) -> DbResult<()> {
        let mut states = self.states.write().await;
        states.remove(prefix);
        Ok(())
    }
}

#[async_trait]
impl ReceiptStore for InMemoryDatabase {
    async fn add_receipt(&self, receipt: &NontransferableReceipt) -> DbResult<()> {
        let mut receipts = self.receipts.write().await;
        let event_receipts = receipts
            .entry(receipt.event_digest.clone())
            .or_insert_with(HashMap::new);

        event_receipts.insert(receipt.witness_prefix.clone(), receipt.clone());
        Ok(())
    }

    async fn get_receipts(&self, event_digest: &str) -> DbResult<Vec<NontransferableReceipt>> {
        let receipts = self.receipts.read().await;
        Ok(receipts
            .get(event_digest)
            .map(|m| m.values().cloned().collect())
            .unwrap_or_default())
    }

    async fn get_receipt(
        &self,
        event_digest: &str,
        witness_prefix: &str,
    ) -> DbResult<Option<NontransferableReceipt>> {
        let receipts = self.receipts.read().await;
        Ok(receipts
            .get(event_digest)
            .and_then(|m| m.get(witness_prefix).cloned()))
    }

    async fn count_receipts(&self, event_digest: &str) -> DbResult<usize> {
        let receipts = self.receipts.read().await;
        Ok(receipts.get(event_digest).map(|m| m.len()).unwrap_or(0))
    }
}

#[async_trait]
impl EscrowStore for InMemoryDatabase {
    async fn escrow_event(&self, event: &SignedEvent, reason: EscrowReason) -> DbResult<()> {
        let mut escrows = self.escrows.write().await;
        let escrowed = EscrowedEvent::new(event.clone(), reason, 3600); // 1 hour TTL
        escrows.insert(event.event.digest.clone(), escrowed);
        Ok(())
    }

    async fn get_escrowed(&self, prefix: &str) -> DbResult<Vec<EscrowedEvent>> {
        let escrows = self.escrows.read().await;
        Ok(escrows
            .values()
            .filter(|e| e.event.event.prefix == prefix)
            .cloned()
            .collect())
    }

    async fn get_all_escrowed(&self) -> DbResult<Vec<EscrowedEvent>> {
        let escrows = self.escrows.read().await;
        Ok(escrows.values().cloned().collect())
    }

    async fn promote_escrowed(&self, event_digest: &str) -> DbResult<Option<SignedEvent>> {
        let mut escrows = self.escrows.write().await;
        Ok(escrows.remove(event_digest).map(|e| e.event))
    }

    async fn remove_escrowed(&self, event_digest: &str) -> DbResult<()> {
        let mut escrows = self.escrows.write().await;
        escrows.remove(event_digest);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kerihost_core::{EventType, IndexedSignature, KeyEvent, Threshold};

    fn create_test_event(prefix: &str, sn: u64, prior_digest: Option<String>) -> SignedEvent {
        let event = KeyEvent {
            prefix: prefix.to_string(),
            sn,
            event_type: if sn == 0 {
                EventType::Icp
            } else {
                EventType::Ixn
            },
            prior_digest,
            signing_keys: vec!["DKey1234567890123456789012345678901234567890123".to_string()],
            signing_threshold: Threshold::simple(1),
            next_key_digest: Some("ENext123456789012345678901234567890123456789012".to_string()),
            witness_threshold: Threshold::simple(1),
            witnesses: vec![],
            anchors: vec![],
            delegator: None,
            raw: b"test".to_vec(),
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

    fn create_test_state(prefix: &str, sn: u64) -> KeyState {
        KeyState {
            prefix: prefix.to_string(),
            sn,
            latest_digest: format!("EDigest{}_{}", prefix, sn),
            signing_keys: vec!["DKey1".to_string()],
            signing_threshold: Threshold::simple(1),
            next_key_digest: Some("ENext123".to_string()),
            witnesses: vec![],
            witness_threshold: Threshold::simple(0),
            delegator: None,
            config: vec![],
            transferable: true,
            metadata: kerihost_core::HonestMetadata::local_only(0),
        }
    }

    fn create_test_receipt(event_digest: &str, witness: &str) -> NontransferableReceipt {
        NontransferableReceipt {
            event_digest: event_digest.to_string(),
            event_sn: 0,
            event_prefix: "DTest123".to_string(),
            witness_prefix: witness.to_string(),
            signature: "0BSig123".to_string(),
        }
    }

    // KEL Store Tests

    #[tokio::test]
    async fn test_kel_append_inception() {
        let db = InMemoryDatabase::new();
        let event = create_test_event("DTest123", 0, None);

        db.append_event(&event).await.unwrap();

        let retrieved = db.get_event("DTest123", 0).await.unwrap();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().event.digest, event.event.digest);
    }

    #[tokio::test]
    async fn test_kel_append_subsequent() {
        let db = InMemoryDatabase::new();
        let icp = create_test_event("DTest123", 0, None);
        db.append_event(&icp).await.unwrap();

        let ixn = create_test_event("DTest123", 1, Some(icp.event.digest.clone()));
        db.append_event(&ixn).await.unwrap();

        assert_eq!(db.event_count("DTest123").await, 2);
    }

    #[tokio::test]
    async fn test_kel_append_wrong_prior_digest() {
        let db = InMemoryDatabase::new();
        let icp = create_test_event("DTest123", 0, None);
        db.append_event(&icp).await.unwrap();

        let ixn = create_test_event("DTest123", 1, Some("EWrongDigest".to_string()));
        let result = db.append_event(&ixn).await;

        assert!(matches!(result, Err(DbError::PriorDigestMismatch { .. })));
    }

    #[tokio::test]
    async fn test_kel_append_duplicate_inception() {
        let db = InMemoryDatabase::new();
        let icp = create_test_event("DTest123", 0, None);

        db.append_event(&icp).await.unwrap();
        let result = db.append_event(&icp).await;

        assert!(matches!(result, Err(DbError::Duplicate(_))));
    }

    #[tokio::test]
    async fn test_kel_append_missing_prior() {
        let db = InMemoryDatabase::new();
        let event = create_test_event("DTest123", 5, Some("EPrior".to_string()));

        let result = db.append_event(&event).await;
        assert!(matches!(result, Err(DbError::NotFound(_))));
    }

    #[tokio::test]
    async fn test_kel_get_events_range() {
        let db = InMemoryDatabase::new();

        // Create chain of events
        let icp = create_test_event("DTest123", 0, None);
        db.append_event(&icp).await.unwrap();

        let ixn1 = create_test_event("DTest123", 1, Some(icp.event.digest.clone()));
        db.append_event(&ixn1).await.unwrap();

        let ixn2 = create_test_event("DTest123", 2, Some(ixn1.event.digest.clone()));
        db.append_event(&ixn2).await.unwrap();

        // Get all events
        let all = db.get_events("DTest123", 0, None).await.unwrap();
        assert_eq!(all.len(), 3);

        // Get range
        let range = db.get_events("DTest123", 1, Some(2)).await.unwrap();
        assert_eq!(range.len(), 2);
    }

    #[tokio::test]
    async fn test_kel_get_latest() {
        let db = InMemoryDatabase::new();

        let icp = create_test_event("DTest123", 0, None);
        db.append_event(&icp).await.unwrap();

        let ixn = create_test_event("DTest123", 1, Some(icp.event.digest.clone()));
        db.append_event(&ixn).await.unwrap();

        let latest = db.get_latest("DTest123").await.unwrap();
        assert!(latest.is_some());
        assert_eq!(latest.unwrap().event.sn, 1);
    }

    #[tokio::test]
    async fn test_kel_get_by_digest() {
        let db = InMemoryDatabase::new();

        let icp = create_test_event("DTest123", 0, None);
        db.append_event(&icp).await.unwrap();

        let found = db
            .get_event_by_digest("DTest123", &icp.event.digest)
            .await
            .unwrap();
        assert!(found.is_some());

        let not_found = db
            .get_event_by_digest("DTest123", "ENotExist")
            .await
            .unwrap();
        assert!(not_found.is_none());
    }

    // State Store Tests

    #[tokio::test]
    async fn test_state_put_get() {
        let db = InMemoryDatabase::new();
        let state = create_test_state("DTest123", 0);

        db.put_state(&state).await.unwrap();

        let retrieved = db.get_state("DTest123").await.unwrap();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().sn, 0);
    }

    #[tokio::test]
    async fn test_state_update() {
        let db = InMemoryDatabase::new();

        let state1 = create_test_state("DTest123", 0);
        db.put_state(&state1).await.unwrap();

        let state2 = create_test_state("DTest123", 1);
        db.put_state(&state2).await.unwrap();

        let retrieved = db.get_state("DTest123").await.unwrap();
        assert_eq!(retrieved.unwrap().sn, 1);
    }

    #[tokio::test]
    async fn test_state_delete() {
        let db = InMemoryDatabase::new();
        let state = create_test_state("DTest123", 0);

        db.put_state(&state).await.unwrap();
        db.delete_state("DTest123").await.unwrap();

        let retrieved = db.get_state("DTest123").await.unwrap();
        assert!(retrieved.is_none());
    }

    #[tokio::test]
    async fn test_state_not_found() {
        let db = InMemoryDatabase::new();
        let retrieved = db.get_state("DNotExist").await.unwrap();
        assert!(retrieved.is_none());
    }

    // Receipt Store Tests

    #[tokio::test]
    async fn test_receipt_add_get() {
        let db = InMemoryDatabase::new();
        let receipt = create_test_receipt("EDigest123", "BWitness1");

        db.add_receipt(&receipt).await.unwrap();

        let receipts = db.get_receipts("EDigest123").await.unwrap();
        assert_eq!(receipts.len(), 1);
    }

    #[tokio::test]
    async fn test_receipt_multiple_witnesses() {
        let db = InMemoryDatabase::new();

        db.add_receipt(&create_test_receipt("EDigest123", "BWitness1"))
            .await
            .unwrap();
        db.add_receipt(&create_test_receipt("EDigest123", "BWitness2"))
            .await
            .unwrap();

        let count = db.count_receipts("EDigest123").await.unwrap();
        assert_eq!(count, 2);
    }

    #[tokio::test]
    async fn test_receipt_get_specific() {
        let db = InMemoryDatabase::new();
        db.add_receipt(&create_test_receipt("EDigest123", "BWitness1"))
            .await
            .unwrap();

        let receipt = db.get_receipt("EDigest123", "BWitness1").await.unwrap();
        assert!(receipt.is_some());

        let not_found = db.get_receipt("EDigest123", "BWitness2").await.unwrap();
        assert!(not_found.is_none());
    }

    #[tokio::test]
    async fn test_receipt_duplicate_witness() {
        let db = InMemoryDatabase::new();

        db.add_receipt(&create_test_receipt("EDigest123", "BWitness1"))
            .await
            .unwrap();
        db.add_receipt(&create_test_receipt("EDigest123", "BWitness1"))
            .await
            .unwrap();

        // Should only have one (deduped by witness)
        let count = db.count_receipts("EDigest123").await.unwrap();
        assert_eq!(count, 1);
    }

    // Escrow Store Tests

    #[tokio::test]
    async fn test_escrow_add_get() {
        let db = InMemoryDatabase::new();
        let event = create_test_event("DTest123", 5, Some("EPrior".to_string()));

        db.escrow_event(&event, EscrowReason::OutOfOrder)
            .await
            .unwrap();

        let escrowed = db.get_escrowed("DTest123").await.unwrap();
        assert_eq!(escrowed.len(), 1);
        assert_eq!(escrowed[0].reason, EscrowReason::OutOfOrder);
    }

    #[tokio::test]
    async fn test_escrow_get_all() {
        let db = InMemoryDatabase::new();

        let event1 = create_test_event("DTest1", 5, Some("EP1".to_string()));
        let event2 = create_test_event("DTest2", 3, Some("EP2".to_string()));

        db.escrow_event(&event1, EscrowReason::OutOfOrder)
            .await
            .unwrap();
        db.escrow_event(&event2, EscrowReason::PartiallySigned)
            .await
            .unwrap();

        let all = db.get_all_escrowed().await.unwrap();
        assert_eq!(all.len(), 2);
    }

    #[tokio::test]
    async fn test_escrow_promote() {
        let db = InMemoryDatabase::new();
        let event = create_test_event("DTest123", 5, Some("EPrior".to_string()));

        db.escrow_event(&event, EscrowReason::OutOfOrder)
            .await
            .unwrap();

        let promoted = db.promote_escrowed(&event.event.digest).await.unwrap();
        assert!(promoted.is_some());

        let remaining = db.get_escrowed("DTest123").await.unwrap();
        assert!(remaining.is_empty());
    }

    #[tokio::test]
    async fn test_escrow_remove() {
        let db = InMemoryDatabase::new();
        let event = create_test_event("DTest123", 5, Some("EPrior".to_string()));

        db.escrow_event(&event, EscrowReason::OutOfOrder)
            .await
            .unwrap();
        db.remove_escrowed(&event.event.digest).await.unwrap();

        let remaining = db.get_escrowed("DTest123").await.unwrap();
        assert!(remaining.is_empty());
    }

    // Database Clear Test

    #[tokio::test]
    async fn test_clear() {
        let db = InMemoryDatabase::new();

        let event = create_test_event("DTest123", 0, None);
        db.append_event(&event).await.unwrap();

        let state = create_test_state("DTest123", 0);
        db.put_state(&state).await.unwrap();

        db.clear().await;

        assert!(db.get_event("DTest123", 0).await.unwrap().is_none());
        assert!(db.get_state("DTest123").await.unwrap().is_none());
    }

    // Clone Test

    #[tokio::test]
    async fn test_clone_shares_data() {
        let db1 = InMemoryDatabase::new();
        let db2 = db1.clone();

        let event = create_test_event("DTest123", 0, None);
        db1.append_event(&event).await.unwrap();

        // db2 should see the same data
        let retrieved = db2.get_event("DTest123", 0).await.unwrap();
        assert!(retrieved.is_some());
    }
}
