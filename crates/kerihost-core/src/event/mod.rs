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
use cesride::{Diger, Indexer, Matter, Prefixer, Siger, Verfer};
use parside::{CesrGroup, Message};
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
    ///
    /// For weighted thresholds, `satisfied_indices` should be the indices of keys
    /// with valid signatures. For simple thresholds, only `sig_count` matters.
    pub fn is_satisfied(&self, sig_count: usize, _key_count: usize) -> bool {
        match self {
            Threshold::Simple(n) => sig_count >= *n as usize,
            Threshold::Weighted(_) => {
                // Any clause satisfied means threshold is met
                // This simple check assumes signatures are for indices 0..sig_count
                // For proper checking, use is_satisfied_by_indices
                self.is_satisfied_by_indices(&(0..sig_count).collect::<Vec<_>>())
            }
        }
    }

    /// Check if weighted threshold is satisfied by specific key indices
    pub fn is_satisfied_by_indices(&self, indices: &[usize]) -> bool {
        match self {
            Threshold::Simple(n) => indices.len() >= *n as usize,
            Threshold::Weighted(clauses) => {
                // Each inner vec is a clause; satisfaction requires meeting ANY clause
                // Within a clause, sum weights of keys with valid signatures
                // Clause is satisfied if sum >= 1
                clauses.iter().any(|clause| {
                    let mut sum_num: u64 = 0;
                    let mut sum_den: u64 = 1; // LCM denominator
                    for (key_idx, weight_str) in clause.iter().enumerate() {
                        if indices.contains(&key_idx) {
                            if let Some((num, den)) = parse_fraction(weight_str) {
                                // Add num/den to sum: sum = sum_num/sum_den + num/den
                                sum_num = sum_num * den + num * sum_den;
                                sum_den *= den;
                            }
                        }
                    }
                    // Satisfied if sum >= 1, i.e., sum_num >= sum_den
                    sum_num >= sum_den
                })
            }
        }
    }

    /// Get the minimum number of signatures needed
    pub fn min_signatures(&self) -> usize {
        match self {
            Threshold::Simple(n) => *n as usize,
            Threshold::Weighted(clauses) => {
                // Find the clause that requires the fewest signatures
                clauses
                    .iter()
                    .map(|clause| min_sigs_for_clause(clause))
                    .min()
                    .unwrap_or(1)
            }
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
    /// Witnesses to remove (rotation only, from "br" field)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub witnesses_remove: Vec<String>,
    /// Witnesses to add (rotation only, from "ba" field)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub witnesses_add: Vec<String>,
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

        // Validate version string if present
        if let Some(v) = ked["v"].as_str() {
            parse_version_string(v)?;
        }

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

        // Extract witness changes (rotation only)
        let witnesses_remove: Vec<String> = ked["br"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|w| w.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let witnesses_add: Vec<String> = ked["ba"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|w| w.as_str().map(|s| s.to_string()))
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

        // SAID verification: verify the digest field matches the event content
        verify_said(raw, &digest)?;

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
            witnesses_remove,
            witnesses_add,
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
        // Parse the JSON event — from_stream_bytes returns remaining bytes
        let (after_event, first_msg) = Message::from_stream_bytes(raw)
            .map_err(|e| CoreError::CesrParse(format!("parside: {}", e)))?;

        // Verify it's a JSON payload
        match &first_msg {
            Message::Custom { .. } => {}
            _ => {
                return Err(CoreError::CesrParse(
                    "Expected JSON event as first message".into(),
                ))
            }
        }

        // Compute event size from remaining bytes — preserves original raw bytes for SAID
        let event_size = raw.len() - after_event.len();
        let event = KeyEvent::from_cesr(&raw[..event_size])?;

        // Parse attachment groups from remaining bytes
        let mut signatures = Vec::new();
        let mut rest = after_event;
        while !rest.is_empty() {
            let (remaining, msg) = Message::from_stream_bytes(rest)
                .map_err(|e| CoreError::CesrParse(format!("parside attachment: {}", e)))?;

            if let Message::Group { value } = msg {
                if let CesrGroup::ControllerIdxSigsVariant { value: sigs } = value {
                    for sig in &sigs.value {
                        signatures.push(IndexedSignature::from_siger(&sig.siger)?);
                    }
                }
                // Other group types (witness sigs, receipts, etc.) silently ignored for now
            }

            if remaining.len() == rest.len() {
                break; // No progress, avoid infinite loop
            }
            rest = remaining;
        }

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

/// Verify SAID (Self-Addressing IDentifier) of an event
///
/// Takes the original raw bytes, replaces the "d" field value with placeholder
/// chars of the correct length, computes digest, and compares.
fn verify_said(raw: &[u8], expected_digest: &str) -> CoreResult<()> {
    // Determine the digest algorithm from the expected digest code
    let diger = Diger::new_with_qb64(expected_digest)
        .map_err(|e| CoreError::CesrParse(format!("Invalid SAID: {}", e)))?;
    let placeholder = "#".repeat(expected_digest.len());

    // Find and replace the "d" field value in the raw bytes
    // Look for "d":"<value>" pattern
    let raw_str = std::str::from_utf8(raw)
        .map_err(|e| CoreError::CesrParse(format!("Invalid UTF-8 in event: {}", e)))?;

    // Find the "d" field and replace its value with placeholder
    let replaced = replace_json_field_value(raw_str, "d", &placeholder)
        .ok_or_else(|| CoreError::InvalidEvent("Could not find 'd' field for SAID verification".to_string()))?;

    // Compute digest using the same algorithm
    let code = Matter::code(&diger);
    let computed = Diger::new_with_ser(replaced.as_bytes(), Some(&code))
        .map_err(|e| CoreError::CesrParse(format!("Failed to compute SAID: {}", e)))?;

    let computed_qb64 = computed
        .qb64()
        .map_err(|e| CoreError::CesrParse(format!("Failed to encode computed SAID: {}", e)))?;

    if computed_qb64 != expected_digest {
        return Err(CoreError::InvalidEvent(format!(
            "SAID mismatch: expected {}, computed {}",
            expected_digest, computed_qb64
        )));
    }

    Ok(())
}

/// Replace a JSON string field's value in raw text, preserving all formatting
fn replace_json_field_value(json: &str, field: &str, replacement: &str) -> Option<String> {
    // Look for "field":"value" pattern (field must be a string value)
    let pattern = format!("\"{}\":\"", field);
    // Also check with space after colon
    let pattern_spaced = format!("\"{}\": \"", field);

    let (start, prefix_len) = if let Some(pos) = json.find(&pattern) {
        (pos, pattern.len())
    } else if let Some(pos) = json.find(&pattern_spaced) {
        (pos, pattern_spaced.len())
    } else {
        return None;
    };

    let value_start = start + prefix_len;
    // Find the closing quote of the value (handle escaped quotes)
    let rest = &json[value_start..];
    let mut end = 0;
    let mut escape = false;
    for (i, c) in rest.chars().enumerate() {
        if escape {
            escape = false;
            continue;
        }
        if c == '\\' {
            escape = true;
            continue;
        }
        if c == '"' {
            end = i;
            break;
        }
    }

    let mut result = String::with_capacity(json.len());
    result.push_str(&json[..value_start]);
    result.push_str(replacement);
    result.push_str(&json[value_start + end..]);
    Some(result)
}

/// Parse a fractional weight string like "1/2" into (numerator, denominator)
fn parse_fraction(s: &str) -> Option<(u64, u64)> {
    let parts: Vec<&str> = s.split('/').collect();
    match parts.len() {
        1 => {
            let n: u64 = parts[0].parse().ok()?;
            Some((n, 1))
        }
        2 => {
            let num: u64 = parts[0].parse().ok()?;
            let den: u64 = parts[1].parse().ok()?;
            if den == 0 {
                return None;
            }
            Some((num, den))
        }
        _ => None,
    }
}

/// Calculate minimum signatures needed to satisfy a single weighted clause
fn min_sigs_for_clause(clause: &[String]) -> usize {
    // Sort weights descending, greedily pick until sum >= 1
    let mut weights: Vec<(u64, u64)> = clause
        .iter()
        .filter_map(|s| parse_fraction(s))
        .collect();
    // Sort descending by value (num/den)
    weights.sort_by(|a, b| (b.0 * a.1).cmp(&(a.0 * b.1)));

    let mut sum_num: u64 = 0;
    let mut sum_den: u64 = 1;
    let mut count = 0;
    for (num, den) in &weights {
        sum_num = sum_num * den + num * sum_den;
        sum_den *= den;
        count += 1;
        if sum_num >= sum_den {
            return count;
        }
    }
    count.max(1) // Need all signatures if none suffice individually
}

/// Parsed KERI version string
#[derive(Debug, Clone)]
pub struct VersionString {
    /// Protocol (e.g., "KERI")
    pub protocol: String,
    /// Major version
    pub major: u8,
    /// Minor version
    pub minor: u8,
    /// Serialization kind (JSON, CBOR, MGPK)
    pub kind: String,
    /// Event size in bytes (from hex field)
    pub size: usize,
}

/// Parse a KERI version string like "KERI10JSON0000ed_"
fn parse_version_string(v: &str) -> CoreResult<VersionString> {
    // Format: PPPPvvKKKKssssss_ (17 chars)
    // PPPP = protocol (4 chars)
    // vv = version (2 chars: major, minor as hex digits)
    // KKKK = serialization kind (4 chars: JSON, CBOR, MGPK)
    // ssssss = size in hex (6 chars)
    // _ = terminator
    if v.len() < 17 || !v.ends_with('_') {
        return Err(CoreError::InvalidEvent(format!(
            "Invalid version string format: {}",
            v
        )));
    }

    let protocol = &v[0..4];
    if protocol != "KERI" {
        return Err(CoreError::InvalidEvent(format!(
            "Unknown protocol: {}",
            protocol
        )));
    }

    let major = u8::from_str_radix(&v[4..5], 16)
        .map_err(|_| CoreError::InvalidEvent("Invalid version major".to_string()))?;
    let minor = u8::from_str_radix(&v[5..6], 16)
        .map_err(|_| CoreError::InvalidEvent("Invalid version minor".to_string()))?;

    // Accept version 1.0
    if major != 1 || minor != 0 {
        return Err(CoreError::InvalidEvent(format!(
            "Unsupported KERI version: {}.{}",
            major, minor
        )));
    }

    let kind = &v[6..10];
    if kind != "JSON" {
        return Err(CoreError::InvalidEvent(format!(
            "Unsupported serialization kind: {} (only JSON supported)",
            kind
        )));
    }

    let size = usize::from_str_radix(&v[10..16], 16)
        .map_err(|_| CoreError::InvalidEvent("Invalid size in version string".to_string()))?;

    Ok(VersionString {
        protocol: protocol.to_string(),
        major,
        minor,
        kind: kind.to_string(),
        size,
    })
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
        // Build a valid event with correct SAID using the proper derivation procedure:
        // 1. Create JSON with placeholder d and placeholder size (same length as final)
        // 2. The size field is 6 hex chars, placeholder is also 6 hex chars, so size is stable
        // 3. Compute size first, then compute SAID, then substitute
        let placeholder = "#".repeat(44);
        let prefix = "DJD91FzIX4DH6VZ9fICaNM6KrOJ4CcXTX2mH4lPAMjpI";
        // Use a temp size that has the right number of digits
        let temp = format!(
            r#"{{"v":"KERI10JSON000000_","t":"icp","d":"{}","i":"{}","s":"0","kt":"1","k":["{}"],"nt":"1","n":["ENpxKUo7y4UKcTI0F7T4rH5mwgmfblhB4kGUGLCDJZDs"],"bt":"0","b":[],"c":[],"a":[]}}"#,
            placeholder, prefix, prefix,
        );
        // JSON length is stable (placeholder and SAID are both 44 chars)
        let size_hex = format!("{:06x}", temp.len());
        let template = temp.replace("000000", &size_hex);

        // Compute SAID over the template (d is placeholder, size is correct)
        let diger = Diger::new_with_ser(template.as_bytes(), Some("E")).unwrap();
        let said = diger.qb64().unwrap();

        // Substitute SAID into d field only (first occurrence of placeholder)
        let json = template.replacen(&placeholder, &said, 1);

        let event = KeyEvent::from_cesr(json.as_bytes()).unwrap();
        assert_eq!(event.event_type, EventType::Icp);
        assert_eq!(event.sn, 0);
        assert_eq!(event.signing_keys.len(), 1);
        assert_eq!(event.digest, said);
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
            witnesses_remove: vec![],
            witnesses_add: vec![],
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

    // --- SAID verification tests ---

    #[test]
    fn test_said_tampered_event_rejected() {
        let placeholder = "#".repeat(44);
        let prefix = "DJD91FzIX4DH6VZ9fICaNM6KrOJ4CcXTX2mH4lPAMjpI";
        let temp = format!(
            r#"{{"v":"KERI10JSON000000_","t":"icp","d":"{}","i":"{}","s":"0","kt":"1","k":["{}"],"nt":"1","n":["ENpxKUo7y4UKcTI0F7T4rH5mwgmfblhB4kGUGLCDJZDs"],"bt":"0","b":[],"c":[],"a":[]}}"#,
            placeholder, prefix, prefix,
        );
        let size_hex = format!("{:06x}", temp.len());
        let template = temp.replace("000000", &size_hex);
        let diger = Diger::new_with_ser(template.as_bytes(), Some("E")).unwrap();
        let said = diger.qb64().unwrap();
        let mut json = template.replacen(&placeholder, &said, 1);

        // Tamper with the event: change sequence number
        json = json.replace(r#""s":"0""#, r#""s":"1""#);

        let result = KeyEvent::from_cesr(json.as_bytes());
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("SAID mismatch"));
    }

    // --- Weighted threshold tests ---

    #[test]
    fn test_weighted_threshold_satisfied() {
        // 2-of-3 with equal weights: each key has weight 1/2, need sum >= 1
        let t = Threshold::Weighted(vec![vec![
            "1/2".to_string(),
            "1/2".to_string(),
            "1/2".to_string(),
        ]]);
        assert!(t.is_satisfied_by_indices(&[0, 1]));    // 1/2 + 1/2 = 1 >= 1
        assert!(t.is_satisfied_by_indices(&[0, 2]));    // 1/2 + 1/2 = 1 >= 1
        assert!(!t.is_satisfied_by_indices(&[0]));       // 1/2 < 1
        assert!(!t.is_satisfied_by_indices(&[]));
    }

    #[test]
    fn test_weighted_threshold_multi_clause() {
        // Two clauses: either [1/2, 1/2] or [1]
        let t = Threshold::Weighted(vec![
            vec!["1/2".to_string(), "1/2".to_string()],
            vec!["1".to_string()],
        ]);
        assert!(t.is_satisfied_by_indices(&[0, 1]));    // First clause: 1/2+1/2=1
        assert!(t.is_satisfied_by_indices(&[0]));        // Second clause: key 0 has weight 1
        assert!(!t.is_satisfied_by_indices(&[]));
    }

    #[test]
    fn test_weighted_threshold_min_signatures() {
        let t = Threshold::Weighted(vec![vec![
            "1/2".to_string(),
            "1/2".to_string(),
            "1/2".to_string(),
        ]]);
        assert_eq!(t.min_signatures(), 2);

        let t = Threshold::Weighted(vec![
            vec!["1/3".to_string(), "1/3".to_string(), "1/3".to_string()],
            vec!["1".to_string()],
        ]);
        assert_eq!(t.min_signatures(), 1); // Second clause needs only 1
    }

    // --- Version string tests ---

    #[test]
    fn test_version_string_parsing() {
        let vs = parse_version_string("KERI10JSON0000ed_").unwrap();
        assert_eq!(vs.protocol, "KERI");
        assert_eq!(vs.major, 1);
        assert_eq!(vs.minor, 0);
        assert_eq!(vs.kind, "JSON");
        assert_eq!(vs.size, 0xed);
    }

    #[test]
    fn test_version_string_unsupported_protocol() {
        let result = parse_version_string("ACDC10JSON0000ed_");
        assert!(result.is_err());
    }

    #[test]
    fn test_version_string_unsupported_kind() {
        let result = parse_version_string("KERI10CBOR0000ed_");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("only JSON"));
    }

    #[test]
    fn test_version_string_unsupported_version() {
        let result = parse_version_string("KERI20JSON0000ed_");
        assert!(result.is_err());
    }

    // --- Fraction parsing tests ---

    #[test]
    fn test_parse_fraction() {
        assert_eq!(parse_fraction("1/2"), Some((1, 2)));
        assert_eq!(parse_fraction("1/3"), Some((1, 3)));
        assert_eq!(parse_fraction("1"), Some((1, 1)));
        assert_eq!(parse_fraction("0/1"), Some((0, 1)));
        assert_eq!(parse_fraction("1/0"), None); // division by zero
    }
}
