//! Interaction event (ixn) creation and handling
//!
//! Interaction events are non-establishment events that anchor data
//! without changing the key state. They only increment the sequence
//! number and chain to the prior event.

use crate::error::{CoreError, CoreResult};
use serde_json::json;

/// Parameters for creating an interaction event
#[derive(Debug, Clone)]
pub struct InteractionParams {
    /// Identifier prefix
    pub prefix: String,
    /// Sequence number
    pub sn: u64,
    /// Prior event digest
    pub prior_digest: String,
    /// Anchors/seals to include
    pub anchors: Vec<serde_json::Value>,
}

impl InteractionParams {
    /// Create new interaction parameters
    pub fn new(prefix: String, sn: u64, prior_digest: String) -> Self {
        InteractionParams {
            prefix,
            sn,
            prior_digest,
            anchors: vec![],
        }
    }

    /// Add an anchor
    pub fn with_anchor(mut self, anchor: serde_json::Value) -> Self {
        self.anchors.push(anchor);
        self
    }

    /// Add multiple anchors
    pub fn with_anchors(mut self, anchors: Vec<serde_json::Value>) -> Self {
        self.anchors.extend(anchors);
        self
    }

    /// Convert to Key Event Dictionary (KED)
    pub fn to_ked(&self) -> CoreResult<serde_json::Value> {
        let ked = json!({
            "v": "KERI10JSON000000_",
            "t": "ixn",
            "d": "",
            "i": self.prefix,
            "s": format!("{:x}", self.sn),
            "p": self.prior_digest,
            "a": self.anchors
        });

        Ok(ked)
    }
}

/// Builder for interaction events
pub struct InteractionBuilder {
    prefix: String,
    sn: u64,
    prior_digest: String,
    anchors: Vec<serde_json::Value>,
}

impl InteractionBuilder {
    /// Create new builder
    pub fn new(prefix: String, sn: u64, prior_digest: String) -> Self {
        InteractionBuilder {
            prefix,
            sn,
            prior_digest,
            anchors: vec![],
        }
    }

    /// Add a digest seal
    pub fn digest_seal(mut self, digest: &str) -> Self {
        self.anchors.push(json!({ "d": digest }));
        self
    }

    /// Add an event seal
    pub fn event_seal(mut self, prefix: &str, sn: &str, digest: &str) -> Self {
        self.anchors.push(json!({
            "i": prefix,
            "s": sn,
            "d": digest
        }));
        self
    }

    /// Add raw anchor
    pub fn anchor(mut self, anchor: serde_json::Value) -> Self {
        self.anchors.push(anchor);
        self
    }

    /// Build interaction parameters
    pub fn build(self) -> CoreResult<InteractionParams> {
        if self.sn == 0 {
            return Err(CoreError::InvalidEvent(
                "Interaction sequence number must be > 0".to_string(),
            ));
        }

        Ok(InteractionParams {
            prefix: self.prefix,
            sn: self.sn,
            prior_digest: self.prior_digest,
            anchors: self.anchors,
        })
    }
}

/// Create a digest seal anchor
pub fn digest_seal(digest: &str) -> serde_json::Value {
    json!({ "d": digest })
}

/// Create an event seal anchor
pub fn event_seal(prefix: &str, sn: u64, digest: &str) -> serde_json::Value {
    json!({
        "i": prefix,
        "s": format!("{:x}", sn),
        "d": digest
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interaction_params_new() {
        let params = InteractionParams::new(
            "DTest123".to_string(),
            1,
            "EDigest123".to_string(),
        );

        assert_eq!(params.prefix, "DTest123");
        assert_eq!(params.sn, 1);
        assert_eq!(params.prior_digest, "EDigest123");
        assert!(params.anchors.is_empty());
    }

    #[test]
    fn test_interaction_params_with_anchor() {
        let anchor = json!({ "d": "EAnchorDigest" });
        let params = InteractionParams::new(
            "DTest123".to_string(),
            1,
            "EDigest123".to_string(),
        )
        .with_anchor(anchor);

        assert_eq!(params.anchors.len(), 1);
    }

    #[test]
    fn test_interaction_params_to_ked() {
        let params = InteractionParams::new(
            "DTest123".to_string(),
            1,
            "EDigest123".to_string(),
        );
        let ked = params.to_ked().unwrap();

        assert_eq!(ked["t"], "ixn");
        assert_eq!(ked["i"], "DTest123");
        assert_eq!(ked["s"], "1");
        assert_eq!(ked["p"], "EDigest123");
    }

    #[test]
    fn test_interaction_builder() {
        let params = InteractionBuilder::new(
            "DTest123".to_string(),
            1,
            "EDigest123".to_string(),
        )
        .digest_seal("EAnchor1")
        .event_seal("DAnotherAid", "0", "EAnotherDigest")
        .build()
        .unwrap();

        assert_eq!(params.anchors.len(), 2);
    }

    #[test]
    fn test_interaction_builder_sn_zero_fails() {
        let result = InteractionBuilder::new(
            "DTest123".to_string(),
            0,
            "EDigest123".to_string(),
        )
        .build();

        assert!(result.is_err());
    }

    #[test]
    fn test_digest_seal_helper() {
        let seal = digest_seal("EDigest123");
        assert_eq!(seal["d"], "EDigest123");
    }

    #[test]
    fn test_event_seal_helper() {
        let seal = event_seal("DAid123", 0, "EDigest123");
        assert_eq!(seal["i"], "DAid123");
        assert_eq!(seal["s"], "0");
        assert_eq!(seal["d"], "EDigest123");
    }

    #[test]
    fn test_interaction_with_multiple_anchors() {
        let anchors = vec![
            digest_seal("E1"),
            digest_seal("E2"),
            event_seal("D1", 0, "E3"),
        ];

        let params = InteractionParams::new("DTest".to_string(), 1, "EPrior".to_string())
            .with_anchors(anchors);

        assert_eq!(params.anchors.len(), 3);
    }
}
