//! Rotation event (rot) creation and handling

use crate::error::{CoreError, CoreResult};
use crate::event::Threshold;
use serde_json::json;

/// Parameters for creating a rotation event
#[derive(Debug, Clone)]
pub struct RotationParams {
    /// Identifier prefix
    pub prefix: String,
    /// Sequence number (must be > 0)
    pub sn: u64,
    /// Prior event digest
    pub prior_digest: String,
    /// New signing keys
    pub keys: Vec<String>,
    /// Signing threshold
    pub threshold: Threshold,
    /// Next keys digest (commitment to next rotation)
    pub next_keys_digest: Option<String>,
    /// Witnesses to add
    pub witnesses_add: Vec<String>,
    /// Witnesses to remove
    pub witnesses_remove: Vec<String>,
    /// Witness threshold
    pub witness_threshold: Threshold,
    /// Anchors/seals
    pub anchors: Vec<serde_json::Value>,
}

impl RotationParams {
    /// Create new rotation parameters
    pub fn new(prefix: String, sn: u64, prior_digest: String, keys: Vec<String>) -> Self {
        RotationParams {
            prefix,
            sn,
            prior_digest,
            keys,
            threshold: Threshold::simple(1),
            next_keys_digest: None,
            witnesses_add: vec![],
            witnesses_remove: vec![],
            witness_threshold: Threshold::simple(0),
            anchors: vec![],
        }
    }

    /// Set signing threshold
    pub fn with_threshold(mut self, threshold: Threshold) -> Self {
        self.threshold = threshold;
        self
    }

    /// Set next keys digest
    pub fn with_next_keys(mut self, digest: String) -> Self {
        self.next_keys_digest = Some(digest);
        self
    }

    /// Set witness changes
    pub fn with_witness_changes(
        mut self,
        add: Vec<String>,
        remove: Vec<String>,
        threshold: Threshold,
    ) -> Self {
        self.witnesses_add = add;
        self.witnesses_remove = remove;
        self.witness_threshold = threshold;
        self
    }

    /// Add an anchor
    pub fn with_anchor(mut self, anchor: serde_json::Value) -> Self {
        self.anchors.push(anchor);
        self
    }

    /// Convert to Key Event Dictionary (KED)
    pub fn to_ked(&self) -> CoreResult<serde_json::Value> {
        let threshold_str = match &self.threshold {
            Threshold::Simple(n) => json!(n.to_string()),
            Threshold::Weighted(w) => json!(w),
        };

        let witness_threshold_str = match &self.witness_threshold {
            Threshold::Simple(n) => json!(n.to_string()),
            Threshold::Weighted(w) => json!(w),
        };

        let next_keys = if let Some(ref nkd) = self.next_keys_digest {
            json!([nkd])
        } else {
            json!([])
        };

        let ked = json!({
            "v": "KERI10JSON000000_",
            "t": "rot",
            "d": "",
            "i": self.prefix,
            "s": format!("{:x}", self.sn),
            "p": self.prior_digest,
            "kt": threshold_str,
            "k": self.keys,
            "nt": "1",
            "n": next_keys,
            "bt": witness_threshold_str,
            "br": self.witnesses_remove,
            "ba": self.witnesses_add,
            "a": self.anchors
        });

        Ok(ked)
    }
}

/// Builder for rotation events
pub struct RotationBuilder {
    prefix: String,
    sn: u64,
    prior_digest: String,
    keys: Vec<String>,
    threshold: Threshold,
    next_keys_digest: Option<String>,
    witnesses_add: Vec<String>,
    witnesses_remove: Vec<String>,
    witness_threshold: Threshold,
    anchors: Vec<serde_json::Value>,
}

impl RotationBuilder {
    /// Create new builder
    pub fn new(prefix: String, sn: u64, prior_digest: String, keys: Vec<String>) -> Self {
        RotationBuilder {
            prefix,
            sn,
            prior_digest,
            keys,
            threshold: Threshold::simple(1),
            next_keys_digest: None,
            witnesses_add: vec![],
            witnesses_remove: vec![],
            witness_threshold: Threshold::simple(0),
            anchors: vec![],
        }
    }

    /// Set signing threshold
    pub fn threshold(mut self, t: Threshold) -> Self {
        self.threshold = t;
        self
    }

    /// Set next key commitment
    pub fn next_keys(mut self, digest: String) -> Self {
        self.next_keys_digest = Some(digest);
        self
    }

    /// Add witnesses
    pub fn add_witnesses(mut self, witnesses: Vec<String>) -> Self {
        self.witnesses_add = witnesses;
        self
    }

    /// Remove witnesses
    pub fn remove_witnesses(mut self, witnesses: Vec<String>) -> Self {
        self.witnesses_remove = witnesses;
        self
    }

    /// Set witness threshold
    pub fn witness_threshold(mut self, t: Threshold) -> Self {
        self.witness_threshold = t;
        self
    }

    /// Add anchor
    pub fn anchor(mut self, anchor: serde_json::Value) -> Self {
        self.anchors.push(anchor);
        self
    }

    /// Build rotation parameters
    pub fn build(self) -> CoreResult<RotationParams> {
        if self.keys.is_empty() {
            return Err(CoreError::InvalidEvent(
                "At least one signing key required".to_string(),
            ));
        }

        if self.sn == 0 {
            return Err(CoreError::InvalidEvent(
                "Rotation sequence number must be > 0".to_string(),
            ));
        }

        Ok(RotationParams {
            prefix: self.prefix,
            sn: self.sn,
            prior_digest: self.prior_digest,
            keys: self.keys,
            threshold: self.threshold,
            next_keys_digest: self.next_keys_digest,
            witnesses_add: self.witnesses_add,
            witnesses_remove: self.witnesses_remove,
            witness_threshold: self.witness_threshold,
            anchors: self.anchors,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotation_params_new() {
        let params = RotationParams::new(
            "DTest123".to_string(),
            1,
            "EDigest123".to_string(),
            vec!["DKey2".to_string()],
        );

        assert_eq!(params.prefix, "DTest123");
        assert_eq!(params.sn, 1);
        assert_eq!(params.prior_digest, "EDigest123");
        assert_eq!(params.keys.len(), 1);
    }

    #[test]
    fn test_rotation_params_to_ked() {
        let params = RotationParams::new(
            "DTest123".to_string(),
            1,
            "EDigest123".to_string(),
            vec!["DKey2".to_string()],
        );
        let ked = params.to_ked().unwrap();

        assert_eq!(ked["t"], "rot");
        assert_eq!(ked["i"], "DTest123");
        assert_eq!(ked["s"], "1");
        assert_eq!(ked["p"], "EDigest123");
    }

    #[test]
    fn test_rotation_builder() {
        let params = RotationBuilder::new(
            "DTest123".to_string(),
            1,
            "EDigest123".to_string(),
            vec!["DKey2".to_string()],
        )
        .threshold(Threshold::simple(1))
        .next_keys("ENext456".to_string())
        .add_witnesses(vec!["BWit2".to_string()])
        .remove_witnesses(vec!["BWit1".to_string()])
        .witness_threshold(Threshold::simple(1))
        .build()
        .unwrap();

        assert_eq!(params.next_keys_digest, Some("ENext456".to_string()));
        assert_eq!(params.witnesses_add.len(), 1);
        assert_eq!(params.witnesses_remove.len(), 1);
    }

    #[test]
    fn test_rotation_builder_sn_zero_fails() {
        let result = RotationBuilder::new(
            "DTest123".to_string(),
            0,
            "EDigest123".to_string(),
            vec!["DKey2".to_string()],
        )
        .build();

        assert!(result.is_err());
    }

    #[test]
    fn test_rotation_builder_no_keys_fails() {
        let result = RotationBuilder::new(
            "DTest123".to_string(),
            1,
            "EDigest123".to_string(),
            vec![],
        )
        .build();

        assert!(result.is_err());
    }

    #[test]
    fn test_rotation_with_anchor() {
        let anchor = json!({
            "i": "DAnchored123",
            "s": "0",
            "d": "EAnchorDigest"
        });

        let params = RotationParams::new(
            "DTest123".to_string(),
            1,
            "EDigest123".to_string(),
            vec!["DKey2".to_string()],
        )
        .with_anchor(anchor);

        assert_eq!(params.anchors.len(), 1);
    }
}
