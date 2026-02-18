//! Receipt generation utilities

use crate::error::{WitnessError, WitnessResult};
use cesride::{Matter, Signer};
use kerihost_core::{NontransferableReceipt, SignedEvent};

/// Generate a non-transferable receipt for an event
pub fn generate_receipt(
    signer: &Signer,
    event: &SignedEvent,
) -> WitnessResult<NontransferableReceipt> {
    // Get witness prefix from signer's verfer
    let witness_prefix = signer
        .verfer()
        .qb64()
        .map_err(|e| WitnessError::Cesr(e.to_string()))?;

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
        witness_prefix,
        signature,
    })
}

/// Batch generate receipts for multiple events
pub fn generate_receipts(
    signer: &Signer,
    events: &[SignedEvent],
) -> WitnessResult<Vec<NontransferableReceipt>> {
    events
        .iter()
        .map(|e| generate_receipt(signer, e))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use cesride::Salter;
    use kerihost_core::{EventType, IndexedSignature, KeyEvent, Threshold};

    fn create_test_signer() -> Signer {
        let salter = Salter::new_with_defaults(None).unwrap();
        salter.signer(None, Some(false), None, None, None).unwrap()
    }

    fn create_test_event() -> SignedEvent {
        let event = KeyEvent {
            prefix: "DTest123456789012345678901234567890123456789012".to_string(),
            sn: 0,
            event_type: EventType::Icp,
            prior_digest: None,
            signing_keys: vec!["DKey1".to_string()],
            signing_threshold: Threshold::simple(1),
            next_key_digest: Some("ENext123".to_string()),
            witness_threshold: Threshold::simple(1),
            witnesses: vec![],
            anchors: vec![],
            witnesses_remove: vec![],
            witnesses_add: vec![],
            delegator: None,
            raw: b"test event data for signing".to_vec(),
            digest: "EDigest12345678901234567890123456789012345678901".to_string(),
        };

        SignedEvent {
            event,
            signatures: vec![IndexedSignature {
                index: 0,
                signature: "AASig".to_string(),
            }],
        }
    }

    #[test]
    fn test_generate_receipt() {
        let signer = create_test_signer();
        let event = create_test_event();

        let receipt = generate_receipt(&signer, &event).unwrap();

        assert_eq!(receipt.event_digest, event.event.digest);
        assert_eq!(receipt.event_sn, event.event.sn);
        assert_eq!(receipt.event_prefix, event.event.prefix);
        assert!(!receipt.signature.is_empty());
        assert!(!receipt.witness_prefix.is_empty());
    }

    #[test]
    fn test_generate_receipts_batch() {
        let signer = create_test_signer();
        let events = vec![create_test_event(), create_test_event()];

        let receipts = generate_receipts(&signer, &events).unwrap();

        assert_eq!(receipts.len(), 2);
    }

    #[test]
    fn test_receipt_witness_prefix_from_signer() {
        let signer = create_test_signer();
        let expected_prefix = signer.verfer().qb64().unwrap();

        let event = create_test_event();
        let receipt = generate_receipt(&signer, &event).unwrap();

        assert_eq!(receipt.witness_prefix, expected_prefix);
    }
}
