//! Receipt types for witness receipts
//!
//! Witnesses issue receipts for events they have validated and stored.
//! These receipts provide evidence of witnessing that contributes to
//! the duplicity detection mechanism.

mod nontrans;

pub use nontrans::*;

use crate::error::CoreResult;
use serde::{Deserialize, Serialize};

/// Receipt types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReceiptType {
    /// Non-transferable witness receipt (most common)
    NonTransferable,
    /// Transferable witness receipt (rare, for transferable witnesses)
    Transferable,
}

/// Generic receipt trait
pub trait Receipt {
    /// Get the digest of the receipted event
    fn event_digest(&self) -> &str;

    /// Get the sequence number of the receipted event
    fn event_sn(&self) -> u64;

    /// Get the prefix of the receipted event
    fn event_prefix(&self) -> &str;

    /// Get the witness prefix
    fn witness_prefix(&self) -> &str;

    /// Verify the receipt signature against event data
    fn verify(&self, event_data: &[u8]) -> CoreResult<bool>;

    /// Serialize to CESR format
    fn to_cesr(&self) -> CoreResult<Vec<u8>>;
}

/// Collection of receipts for an event
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReceiptSet {
    /// The event digest these receipts are for
    pub event_digest: String,

    /// Non-transferable receipts
    pub receipts: Vec<NontransferableReceipt>,
}

impl ReceiptSet {
    /// Create new empty receipt set
    pub fn new(event_digest: String) -> Self {
        ReceiptSet {
            event_digest,
            receipts: vec![],
        }
    }

    /// Add a receipt
    pub fn add(&mut self, receipt: NontransferableReceipt) {
        // Don't add duplicates
        if !self
            .receipts
            .iter()
            .any(|r| r.witness_prefix == receipt.witness_prefix)
        {
            self.receipts.push(receipt);
        }
    }

    /// Get number of unique witnesses
    pub fn witness_count(&self) -> usize {
        self.receipts.len()
    }

    /// Check if a specific witness has receipted
    pub fn has_witness(&self, witness_prefix: &str) -> bool {
        self.receipts
            .iter()
            .any(|r| r.witness_prefix == witness_prefix)
    }

    /// Get all witness prefixes
    pub fn witnesses(&self) -> Vec<&str> {
        self.receipts
            .iter()
            .map(|r| r.witness_prefix.as_str())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_receipt(witness: &str) -> NontransferableReceipt {
        NontransferableReceipt {
            event_digest: "EDigest12345678901234567890123456789012345678901".to_string(),
            event_sn: 0,
            event_prefix: "DPrefix12345678901234567890123456789012345678901".to_string(),
            witness_prefix: witness.to_string(),
            signature: "0BSig123456789012345678901234567890123456789012345678901234567890123456789012345678901234".to_string(),
        }
    }

    #[test]
    fn test_receipt_set_new() {
        let set = ReceiptSet::new("EDigest123".to_string());
        assert_eq!(set.event_digest, "EDigest123");
        assert_eq!(set.witness_count(), 0);
    }

    #[test]
    fn test_receipt_set_add() {
        let mut set = ReceiptSet::new("EDigest123".to_string());
        let receipt = create_test_receipt("BWitness1");

        set.add(receipt);
        assert_eq!(set.witness_count(), 1);
    }

    #[test]
    fn test_receipt_set_no_duplicates() {
        let mut set = ReceiptSet::new("EDigest123".to_string());
        let receipt1 = create_test_receipt("BWitness1");
        let receipt2 = create_test_receipt("BWitness1");

        set.add(receipt1);
        set.add(receipt2);
        assert_eq!(set.witness_count(), 1);
    }

    #[test]
    fn test_receipt_set_has_witness() {
        let mut set = ReceiptSet::new("EDigest123".to_string());
        set.add(create_test_receipt("BWitness1"));

        assert!(set.has_witness("BWitness1"));
        assert!(!set.has_witness("BWitness2"));
    }

    #[test]
    fn test_receipt_set_witnesses() {
        let mut set = ReceiptSet::new("EDigest123".to_string());
        set.add(create_test_receipt("BWitness1"));
        set.add(create_test_receipt("BWitness2"));

        let witnesses = set.witnesses();
        assert_eq!(witnesses.len(), 2);
        assert!(witnesses.contains(&"BWitness1"));
        assert!(witnesses.contains(&"BWitness2"));
    }

    #[test]
    fn test_receipt_type_serialization() {
        let rt = ReceiptType::NonTransferable;
        let json = serde_json::to_string(&rt).unwrap();
        assert_eq!(json, "\"NonTransferable\"");
    }
}
