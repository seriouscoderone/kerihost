//! KERI event types and structures
//!
//! This module defines the core event types used in KERI:
//! - Inception (icp) - Creates a new identifier
//! - Rotation (rot) - Rotates keys for an identifier
//! - Interaction (ixn) - Non-establishment event for anchoring data
//! - Delegated Inception (dip) - Creates a delegated identifier
//! - Delegated Rotation (drt) - Rotates keys for a delegated identifier

mod inception;
mod interaction;
mod rotation;

pub use inception::*;
pub use interaction::*;
pub use rotation::*;

use crate::error::{CoreError, CoreResult};
use cesride::{Diger, Indexer, Prefixer, Siger, Verfer};
use serde::{Deserialize, Serialize};

/// KERI event types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EventType {
    /// Inception event - creates identifier
    Icp,
    /// Rotation event - rotates keys
    Rot,
    /// Interaction event - anchors data
    Ixn,
    /// Delegated inception
    Dip,
    /// Delegated rotation
    Drt,
}

impl EventType {
    /// Parse event type from string
    pub fn from_str(s: &str) -> CoreResult<Self> {
        match s {
            "icp" => Ok(EventType::Icp),
            "rot" => Ok(EventType::Rot),
            "ixn" => Ok(EventType::Ixn),
            "dip" => Ok(EventType::Dip),
            "drt" => Ok(EventType::Drt),
            _ => Err(CoreError::UnknownEventType(s.to_string())),
        }
    }

    /// Check if this is an establishment event (changes key state)
    pub fn is_establishment(&self) -> bool {
        matches!(
            self,
            EventType::Icp | EventType::Rot | EventType::Dip | EventType::Drt
        )
    }

    /// Check if this is a delegated event
    pub fn is_delegated(&self) -> bool {
        matches!(self, EventType::Dip | EventType::Drt)
    }
}

impl std::fmt::Display for EventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EventType::Icp => write!(f, "icp"),
            EventType::Rot => write!(f, "rot"),
            EventType::Ixn => write!(f, "ixn"),
            EventType::Dip => write!(f, "dip"),
            EventType::Drt => write!(f, "drt"),
        }
    }
}

/// Threshold specification for multi-sig
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Threshold {
    /// Simple numeric threshold (e.g., "2" means 2-of-n)
    Simple(u32),
    /// Weighted threshold (e.g., [["1/2", "1/2", "1/2"], ["1"]])
    Weighted(Vec<Vec<String>>),
}

impl Threshold {
    /// Create a simple threshold
    pub fn simple(n: u32) -> Self {
        Threshold::Simple(n)
    }

    /// Check if signatures meet threshold
    pub fn is_satisfied(&self, sig_count: usize, _key_count: usize) -> bool {
        match self {
            Threshold::Simple(n) => sig_count >= *n as usize,
            Threshold::Weighted(_weights) => {
                // For weighted thresholds, need more complex logic
                // For now, treat as simple majority
                // TODO: Implement proper weighted threshold checking
                sig_count > 0
            }
        }
    }

    /// Get the minimum number of signatures needed
    pub fn min_signatures(&self) -> usize {
        match self {
            Threshold::Simple(n) => *n as usize,
            Threshold::Weighted(_) => 1, // Conservative minimum
        }
    }
}

impl Default for Threshold {
    fn default() -> Self {
        Threshold::Simple(1)
    }
}

/// Seal/anchor in an event
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Anchor {
    /// Identifier prefix (for event seals)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i: Option<String>,
    /// Sequence number (for event seals)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s: Option<String>,
    /// Digest
    pub d: String,
}

impl Anchor {
    /// Create a digest seal
    pub fn digest(d: &str) -> Self {
        Anchor {
            i: None,
            s: None,
            d: d.to_string(),
        }
    }

    /// Create an event seal
    pub fn event(i: &str, s: &str, d: &str) -> Self {
        Anchor {
            i: Some(i.to_string()),
            s: Some(s.to_string()),
            d: d.to_string(),
        }
    }
}

/// Parsed and validated KERI key event
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyEvent {
    /// Identifier prefix (AID)
    pub prefix: String,
    /// Sequence number
    pub sn: u64,
    /// Event type
    pub event_type: EventType,
    /// Previous event digest (None for inception)
    pub prior_digest: Option<String>,
    /// Current signing keys (as qb64 strings)
    pub signing_keys: Vec<String>,
    /// Signing threshold
    pub signing_threshold: Threshold,
    /// Next key commitment (digest of next keys)
    pub next_key_digest: Option<String>,
    /// Witness threshold
    pub witness_threshold: Threshold,
    /// Witnesses (as qb64 prefix strings)
    pub witnesses: Vec<String>,
    /// Anchors/seals
    pub anchors: Vec<Anchor>,
    /// Delegator prefix (for delegated events)
    pub delegator: Option<String>,
    /// Original serialized bytes (for signing/verification)
    #[serde(skip)]
    pub raw: Vec<u8>,
    /// Event digest (SAID)
    pub digest: String,
}

impl KeyEvent {
    /// Parse event from raw JSON bytes
    pub fn from_cesr(raw: &[u8]) -> CoreResult<Self> {
        // Parse as JSON
        let ked: serde_json::Value = serde_json::from_slice(raw)
            .map_err(|e| CoreError::CesrParse(format!("JSON parse error: {}", e)))?;

        // Extract event type
        let event_type_str = ked["t"]
            .as_str()
            .ok_or_else(|| CoreError::InvalidEvent("missing event type".to_string()))?;
        let event_type = EventType::from_str(event_type_str)?;

        // Extract prefix
        let prefix = ked["i"]
            .as_str()
            .ok_or_else(|| CoreError::InvalidEvent("missing prefix".to_string()))?
            .to_string();

        // Extract sequence number
        let sn_str = ked["s"]
            .as_str()
            .ok_or_else(|| CoreError::InvalidEvent("missing sequence number".to_string()))?;
        let sn = u64::from_str_radix(sn_str, 16)
            .map_err(|_| CoreError::InvalidEvent("invalid sequence number".to_string()))?;

        // Extract prior digest (not present for inception)
        let prior_digest = ked["p"].as_str().map(|s| s.to_string());

        // Extract signing keys
        let signing_keys: Vec<String> = ked["k"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|k| k.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        // Extract signing threshold
        let signing_threshold = if let Some(kt) = ked.get("kt") {
            parse_threshold(kt)?
        } else {
            Threshold::simple(1)
        };

        // Extract next key digest
        let next_key_digest = if let Some(n) = ked.get("n") {
            if let Some(arr) = n.as_array() {
                arr.first().and_then(|v| v.as_str().map(|s| s.to_string()))
            } else {
                n.as_str().map(|s| s.to_string())
            }
        } else {
            None
        };

        // Extract witness threshold
        let witness_threshold = if let Some(wt) = ked.get("bt") {
            parse_threshold(wt)?
        } else {
            Threshold::simple(0)
        };

        // Extract witnesses
        let witnesses: Vec<String> = ked["b"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|w| w.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        // Extract anchors
        let anchors: Vec<Anchor> = ked["a"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|a| serde_json::from_value(a.clone()).ok())
                    .collect()
            })
            .unwrap_or_default();

        // Extract delegator (for dip/drt)
        let delegator = ked["di"].as_str().map(|s| s.to_string());

        // Get event digest (SAID - from d field)
        let digest = ked["d"]
            .as_str()
            .ok_or_else(|| CoreError::InvalidEvent("missing digest".to_string()))?
            .to_string();

        Ok(KeyEvent {
            prefix,
            sn,
            event_type,
            prior_digest,
            signing_keys,
            signing_threshold,
            next_key_digest,
            witness_threshold,
            witnesses,
            anchors,
            delegator,
            raw: raw.to_vec(),
            digest,
        })
    }

    /// Check if this is an establishment event
    pub fn is_establishment(&self) -> bool {
        self.event_type.is_establishment()
    }

    /// Check if this is a delegated event
    pub fn is_delegated(&self) -> bool {
        self.event_type.is_delegated()
    }

    /// Get prefix as Prefixer
    pub fn prefixer(&self) -> CoreResult<Prefixer> {
        Prefixer::new_with_qb64(&self.prefix).map_err(|e| CoreError::CesrParse(e.to_string()))
    }

    /// Get digest as Diger
    pub fn diger(&self) -> CoreResult<Diger> {
        Diger::new_with_qb64(&self.digest).map_err(|e| CoreError::CesrParse(e.to_string()))
    }

    /// Get signing keys as Verfers
    pub fn verfers(&self) -> CoreResult<Vec<Verfer>> {
        self.signing_keys
            .iter()
            .map(|k| Verfer::new_with_qb64(k).map_err(|e| CoreError::CesrParse(e.to_string())))
            .collect()
    }
}

/// Signed event with attached signatures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedEvent {
    /// The underlying event
    pub event: KeyEvent,
    /// Controller signatures
    pub signatures: Vec<IndexedSignature>,
}

/// Indexed signature with key index
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexedSignature {
    /// Key index in the signing keys array
    pub index: u32,
    /// The signature (as qb64)
    pub signature: String,
}

impl IndexedSignature {
    /// Create from cesride Siger
    pub fn from_siger(siger: &Siger) -> CoreResult<Self> {
        Ok(IndexedSignature {
            index: Indexer::index(siger),
            signature: Indexer::qb64(siger).map_err(|e| CoreError::CesrParse(e.to_string()))?,
        })
    }

    /// Convert to cesride Siger (requires Verfer)
    pub fn to_siger(&self, verfer: &Verfer) -> CoreResult<Siger> {
        Siger::new_with_qb64(&self.signature, Some(verfer))
            .map_err(|e| CoreError::CesrParse(e.to_string()))
    }
}

impl SignedEvent {
    /// Create signed event from event and signatures
    pub fn new(event: KeyEvent, signatures: Vec<IndexedSignature>) -> Self {
        SignedEvent { event, signatures }
    }

    /// Parse signed event from CESR stream
    pub fn from_cesr(raw: &[u8]) -> CoreResult<Self> {
        // First parse the event
        let (event_end, event) = parse_event_from_stream(raw)?;

        // Then parse signatures from remainder
        let signatures = parse_signatures(&raw[event_end..])?;

        Ok(SignedEvent { event, signatures })
    }

    /// Get signature count
    pub fn signature_count(&self) -> usize {
        self.signatures.len()
    }
}

// Helper functions

fn parse_threshold(value: &serde_json::Value) -> CoreResult<Threshold> {
    if let Some(s) = value.as_str() {
        let n: u32 = s
            .parse()
            .map_err(|_| CoreError::InvalidThreshold(s.to_string()))?;
        Ok(Threshold::Simple(n))
    } else if let Some(n) = value.as_u64() {
        Ok(Threshold::Simple(n as u32))
    } else if let Some(arr) = value.as_array() {
        // Weighted threshold
        let weights: Vec<Vec<String>> = arr
            .iter()
            .filter_map(|inner| {
                inner.as_array().map(|a| {
                    a.iter()
                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                        .collect()
                })
            })
            .collect();
        Ok(Threshold::Weighted(weights))
    } else {
        Err(CoreError::InvalidThreshold(format!("{:?}", value)))
    }
}

fn parse_event_from_stream(raw: &[u8]) -> CoreResult<(usize, KeyEvent)> {
    // Find the end of the JSON event (before attachments)
    // KERI events are JSON followed by CESR attachments
    let mut depth = 0;
    let mut in_string = false;
    let mut escape = false;
    let mut end = 0;

    for (i, &b) in raw.iter().enumerate() {
        if escape {
            escape = false;
            continue;
        }

        match b {
            b'\\' if in_string => escape = true,
            b'"' => in_string = !in_string,
            b'{' if !in_string => depth += 1,
            b'}' if !in_string => {
                depth -= 1;
                if depth == 0 {
                    end = i + 1;
                    break;
                }
            }
            _ => {}
        }
    }

    if end == 0 {
        return Err(CoreError::CesrParse("Could not find event end".to_string()));
    }

    let event = KeyEvent::from_cesr(&raw[..end])?;
    Ok((end, event))
}

fn parse_signatures(raw: &[u8]) -> CoreResult<Vec<IndexedSignature>> {
    if raw.is_empty() {
        return Ok(vec![]);
    }

    let mut signatures = Vec::new();

    // Parse CESR counter and signatures
    // Counter format: -A## where ## is base64 count
    if raw.len() >= 4 && raw[0] == b'-' {
        let counter_code = std::str::from_utf8(&raw[..2])
            .map_err(|e| CoreError::CesrParse(e.to_string()))?;

        if counter_code == "-A" {
            // Controller indexed signatures
            let count_b64 = std::str::from_utf8(&raw[2..4])
                .map_err(|e| CoreError::CesrParse(e.to_string()))?;
            let count = decode_b64_count(count_b64)?;

            let mut offset = 4;
            for _ in 0..count {
                if offset + 88 > raw.len() {
                    break;
                }

                let sig_qb64 = std::str::from_utf8(&raw[offset..offset + 88])
                    .map_err(|e| CoreError::CesrParse(e.to_string()))?;

                // Parse index from signature code
                let siger = Siger::new_with_qb64(sig_qb64, None)
                    .map_err(|e| CoreError::CesrParse(e.to_string()))?;

                signatures.push(IndexedSignature::from_siger(&siger)?);
                offset += 88;
            }
        }
    }

    Ok(signatures)
}

fn decode_b64_count(s: &str) -> CoreResult<usize> {
    const B64_CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";

    let bytes = s.as_bytes();
    if bytes.len() != 2 {
        return Err(CoreError::CesrParse("Invalid count length".to_string()));
    }

    let high = B64_CHARS
        .iter()
        .position(|&c| c == bytes[0])
        .ok_or_else(|| CoreError::CesrParse("Invalid base64 char".to_string()))?;
    let low = B64_CHARS
        .iter()
        .position(|&c| c == bytes[1])
        .ok_or_else(|| CoreError::CesrParse("Invalid base64 char".to_string()))?;

    Ok(high * 64 + low)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_type_from_str() {
        assert_eq!(EventType::from_str("icp").unwrap(), EventType::Icp);
        assert_eq!(EventType::from_str("rot").unwrap(), EventType::Rot);
        assert_eq!(EventType::from_str("ixn").unwrap(), EventType::Ixn);
        assert_eq!(EventType::from_str("dip").unwrap(), EventType::Dip);
        assert_eq!(EventType::from_str("drt").unwrap(), EventType::Drt);
        assert!(EventType::from_str("invalid").is_err());
    }

    #[test]
    fn test_event_type_is_establishment() {
        assert!(EventType::Icp.is_establishment());
        assert!(EventType::Rot.is_establishment());
        assert!(!EventType::Ixn.is_establishment());
        assert!(EventType::Dip.is_establishment());
        assert!(EventType::Drt.is_establishment());
    }

    #[test]
    fn test_event_type_is_delegated() {
        assert!(!EventType::Icp.is_delegated());
        assert!(!EventType::Rot.is_delegated());
        assert!(!EventType::Ixn.is_delegated());
        assert!(EventType::Dip.is_delegated());
        assert!(EventType::Drt.is_delegated());
    }

    #[test]
    fn test_threshold_simple() {
        let t = Threshold::simple(2);
        assert!(t.is_satisfied(2, 3));
        assert!(t.is_satisfied(3, 3));
        assert!(!t.is_satisfied(1, 3));
        assert_eq!(t.min_signatures(), 2);
    }

    #[test]
    fn test_threshold_parse() {
        let val = serde_json::json!("2");
        let t = parse_threshold(&val).unwrap();
        assert_eq!(t, Threshold::Simple(2));

        let val = serde_json::json!(3);
        let t = parse_threshold(&val).unwrap();
        assert_eq!(t, Threshold::Simple(3));
    }

    #[test]
    fn test_decode_b64_count() {
        assert_eq!(decode_b64_count("AA").unwrap(), 0);
        assert_eq!(decode_b64_count("AB").unwrap(), 1);
        assert_eq!(decode_b64_count("AC").unwrap(), 2);
        assert_eq!(decode_b64_count("BA").unwrap(), 64);
    }

    #[test]
    fn test_anchor_digest() {
        let anchor = Anchor::digest("Eabc123");
        assert!(anchor.i.is_none());
        assert!(anchor.s.is_none());
        assert_eq!(anchor.d, "Eabc123");
    }

    #[test]
    fn test_anchor_event() {
        let anchor = Anchor::event("Did123", "0", "Eabc123");
        assert_eq!(anchor.i, Some("Did123".to_string()));
        assert_eq!(anchor.s, Some("0".to_string()));
        assert_eq!(anchor.d, "Eabc123");
    }

    #[test]
    fn test_key_event_from_json() {
        let json = r#"{
            "v": "KERI10JSON0000ed_",
            "t": "icp",
            "d": "EBq7-hJfUj_R3yaMhXVDLrP7STSCC7ckVfEDiPq2fhJI",
            "i": "EBq7-hJfUj_R3yaMhXVDLrP7STSCC7ckVfEDiPq2fhJI",
            "s": "0",
            "kt": "1",
            "k": ["DJD91FzIX4DH6VZ9fICaNM6KrOJ4CcXTX2mH4lPAMjpI"],
            "nt": "1",
            "n": ["ENpxKUo7y4UKcTI0F7T4rH5mwgmfblhB4kGUGLCDJZDs"],
            "bt": "0",
            "b": [],
            "c": [],
            "a": []
        }"#;

        let event = KeyEvent::from_cesr(json.as_bytes()).unwrap();
        assert_eq!(event.event_type, EventType::Icp);
        assert_eq!(event.sn, 0);
        assert_eq!(event.signing_keys.len(), 1);
    }

    #[test]
    fn test_signed_event_serialization() {
        let event = KeyEvent {
            prefix: "DTest123".to_string(),
            sn: 0,
            event_type: EventType::Icp,
            prior_digest: None,
            signing_keys: vec!["DKey1".to_string()],
            signing_threshold: Threshold::simple(1),
            next_key_digest: Some("ENext123".to_string()),
            witness_threshold: Threshold::simple(0),
            witnesses: vec![],
            anchors: vec![],
            delegator: None,
            raw: vec![],
            digest: "EDigest123".to_string(),
        };

        let signed = SignedEvent {
            event,
            signatures: vec![IndexedSignature {
                index: 0,
                signature: "AASig".to_string(),
            }],
        };

        let json = serde_json::to_string(&signed).unwrap();
        assert!(json.contains("\"prefix\":\"DTest123\""));
    }
}
