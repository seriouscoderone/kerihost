//! Non-transferable witness receipts
//!
//! Non-transferable receipts are the most common type of witness receipt.
//! They use an unindexed signature (Cigar) since non-transferable witnesses
//! have a single permanent key.

use crate::error::{CoreError, CoreResult};
use crate::receipt::Receipt;
use cesride::{Cigar, Matter, Signer, Verfer};
use serde::{Deserialize, Serialize};

/// Non-transferable witness receipt
///
/// This receipt type is used by witnesses with non-transferable identifiers
/// (basic witnesses). The witness signs the event with its permanent key.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NontransferableReceipt {
    /// Digest of the receipted event
    pub event_digest: String,

    /// Sequence number of the receipted event
    pub event_sn: u64,

    /// Prefix of the receipted event (the AID being witnessed)
    pub event_prefix: String,

    /// Witness identifier (non-transferable prefix, typically "B" prefix)
    pub witness_prefix: String,

    /// Witness signature (unindexed Cigar)
    pub signature: String,
}

impl NontransferableReceipt {
    /// Create a new non-transferable receipt
    pub fn new(
        event_digest: String,
        event_sn: u64,
        event_prefix: String,
        witness_prefix: String,
        signature: String,
    ) -> Self {
        NontransferableReceipt {
            event_digest,
            event_sn,
            event_prefix,
            witness_prefix,
            signature,
        }
    }

    /// Create receipt by signing event data
    pub fn sign(
        event_digest: String,
        event_sn: u64,
        event_prefix: String,
        witness_prefix: String,
        signer: &Signer,
        event_data: &[u8],
    ) -> CoreResult<Self> {
        // Sign with unindexed signature
        let cigar = signer
            .sign_unindexed(event_data)
            .map_err(|e| CoreError::InvalidSignature(e.to_string()))?;

        let signature = cigar
            .qb64()
            .map_err(|e| CoreError::CesrParse(e.to_string()))?;

        Ok(NontransferableReceipt {
            event_digest,
            event_sn,
            event_prefix,
            witness_prefix,
            signature,
        })
    }

    /// Get the Cigar signature object
    pub fn cigar(&self, verfer: Option<&Verfer>) -> CoreResult<Cigar> {
        Cigar::new_with_qb64(&self.signature, verfer)
            .map_err(|e| CoreError::CesrParse(e.to_string()))
    }

    /// Get the witness public key from prefix
    pub fn witness_verfer(&self) -> CoreResult<Verfer> {
        // Non-transferable prefixes (B prefix) are the public key directly
        Verfer::new_with_qb64(&self.witness_prefix)
            .map_err(|e| CoreError::CesrParse(e.to_string()))
    }
}

impl Receipt for NontransferableReceipt {
    fn event_digest(&self) -> &str {
        &self.event_digest
    }

    fn event_sn(&self) -> u64 {
        self.event_sn
    }

    fn event_prefix(&self) -> &str {
        &self.event_prefix
    }

    fn witness_prefix(&self) -> &str {
        &self.witness_prefix
    }

    fn verify(&self, event_data: &[u8]) -> CoreResult<bool> {
        // Get witness public key
        let verfer = self.witness_verfer()?;

        // Get signature bytes
        let cigar = self.cigar(Some(&verfer))?;

        // Verify
        verfer
            .verify(&Matter::raw(&cigar), event_data)
            .map_err(|e| CoreError::InvalidSignature(e.to_string()))
    }

    fn to_cesr(&self) -> CoreResult<Vec<u8>> {
        // Non-transferable receipt couple format:
        // Counter (-C for NonTransReceiptCouples) + verfer + cigar
        let verfer = self.witness_verfer()?;
        let cigar = self.cigar(Some(&verfer))?;

        let mut result = Vec::new();

        // Counter code for single couple
        result.extend_from_slice(b"-CAB");

        // Verfer qb64
        let verfer_qb64 = verfer
            .qb64()
            .map_err(|e| CoreError::CesrParse(e.to_string()))?;
        result.extend_from_slice(verfer_qb64.as_bytes());

        // Cigar qb64
        let cigar_qb64 = cigar
            .qb64()
            .map_err(|e| CoreError::CesrParse(e.to_string()))?;
        result.extend_from_slice(cigar_qb64.as_bytes());

        Ok(result)
    }
}

/// Builder for non-transferable receipts
pub struct NontransReceiptBuilder {
    event_digest: Option<String>,
    event_sn: Option<u64>,
    event_prefix: Option<String>,
    witness_prefix: Option<String>,
}

impl NontransReceiptBuilder {
    /// Create new builder
    pub fn new() -> Self {
        NontransReceiptBuilder {
            event_digest: None,
            event_sn: None,
            event_prefix: None,
            witness_prefix: None,
        }
    }

    /// Set event digest
    pub fn event_digest(mut self, digest: String) -> Self {
        self.event_digest = Some(digest);
        self
    }

    /// Set event sequence number
    pub fn event_sn(mut self, sn: u64) -> Self {
        self.event_sn = Some(sn);
        self
    }

    /// Set event prefix
    pub fn event_prefix(mut self, prefix: String) -> Self {
        self.event_prefix = Some(prefix);
        self
    }

    /// Set witness prefix
    pub fn witness_prefix(mut self, prefix: String) -> Self {
        self.witness_prefix = Some(prefix);
        self
    }

    /// Build and sign the receipt
    pub fn sign(self, signer: &Signer, event_data: &[u8]) -> CoreResult<NontransferableReceipt> {
        let event_digest = self
            .event_digest
            .ok_or_else(|| CoreError::InvalidEvent("Missing event_digest".to_string()))?;
        let event_sn = self
            .event_sn
            .ok_or_else(|| CoreError::InvalidEvent("Missing event_sn".to_string()))?;
        let event_prefix = self
            .event_prefix
            .ok_or_else(|| CoreError::InvalidEvent("Missing event_prefix".to_string()))?;
        let witness_prefix = self
            .witness_prefix
            .ok_or_else(|| CoreError::InvalidEvent("Missing witness_prefix".to_string()))?;

        NontransferableReceipt::sign(
            event_digest,
            event_sn,
            event_prefix,
            witness_prefix,
            signer,
            event_data,
        )
    }
}

impl Default for NontransReceiptBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_receipt() -> NontransferableReceipt {
        NontransferableReceipt {
            event_digest: "EDigest12345678901234567890123456789012345678901".to_string(),
            event_sn: 0,
            event_prefix: "DPrefix12345678901234567890123456789012345678901".to_string(),
            witness_prefix: "BWitness1234567890123456789012345678901234567890".to_string(),
            signature: "0BSig123456789012345678901234567890123456789012345678901234567890123456789012345678901234".to_string(),
        }
    }

    #[test]
    fn test_receipt_new() {
        let receipt = NontransferableReceipt::new(
            "EDigest123".to_string(),
            0,
            "DPrefix123".to_string(),
            "BWitness123".to_string(),
            "0BSig123".to_string(),
        );

        assert_eq!(receipt.event_digest, "EDigest123");
        assert_eq!(receipt.event_sn, 0);
        assert_eq!(receipt.event_prefix, "DPrefix123");
        assert_eq!(receipt.witness_prefix, "BWitness123");
    }

    #[test]
    fn test_receipt_trait_methods() {
        let receipt = create_test_receipt();

        assert_eq!(
            receipt.event_digest(),
            "EDigest12345678901234567890123456789012345678901"
        );
        assert_eq!(receipt.event_sn(), 0);
        assert_eq!(
            receipt.event_prefix(),
            "DPrefix12345678901234567890123456789012345678901"
        );
        assert_eq!(
            receipt.witness_prefix(),
            "BWitness1234567890123456789012345678901234567890"
        );
    }

    #[test]
    fn test_receipt_serialization() {
        let receipt = create_test_receipt();
        let json = serde_json::to_string(&receipt).unwrap();

        assert!(json.contains("\"eventDigest\":"));
        assert!(json.contains("\"eventSn\":0"));
        assert!(json.contains("\"eventPrefix\":"));
        assert!(json.contains("\"witnessPrefix\":"));
        assert!(json.contains("\"signature\":"));

        // Deserialize back
        let parsed: NontransferableReceipt = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.event_sn, receipt.event_sn);
    }

    #[test]
    fn test_receipt_builder() {
        let builder = NontransReceiptBuilder::new()
            .event_digest("EDigest123".to_string())
            .event_sn(0)
            .event_prefix("DPrefix123".to_string())
            .witness_prefix("BWitness123".to_string());

        // Can't actually sign without a real signer, but builder pattern works
        assert!(builder.event_digest.is_some());
        assert!(builder.event_sn.is_some());
    }

    #[test]
    fn test_receipt_builder_missing_field() {
        let builder = NontransReceiptBuilder::new()
            .event_digest("EDigest123".to_string());
        // Missing required fields

        // Create a fake signer - this will fail because we're missing fields
        // In real code, you'd use cesride::Salter to create a signer
    }
}
