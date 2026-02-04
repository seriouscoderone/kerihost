//! Inception event (icp) creation and handling

use crate::error::{CoreError, CoreResult};
use crate::event::Threshold;
use serde_json::json;

/// Parameters for creating an inception event
#[derive(Debug, Clone)]
pub struct InceptionParams {
    /// Identifier prefix (derived from keys or provided)
    pub prefix: String,
    /// Signing keys (qb64 strings)
    pub keys: Vec<String>,
    /// Signing threshold
    pub threshold: Threshold,
    /// Next keys digest (commitment to rotation keys)
    pub next_keys_digest: Option<String>,
    /// Witness prefixes
    pub witnesses: Vec<String>,
    /// Witness threshold
    pub witness_threshold: Threshold,
    /// Configuration traits
    pub config: Vec<String>,
}

impl InceptionParams {
    /// Create new inception parameters with defaults
    pub fn new(prefix: String, keys: Vec<String>) -> Self {
        InceptionParams {
            prefix,
            keys,
            threshold: Threshold::simple(1),
            next_keys_digest: None,
            witnesses: vec![],
            witness_threshold: Threshold::simple(0),
            config: vec![],
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

    /// Set witnesses
    pub fn with_witnesses(mut self, witnesses: Vec<String>, threshold: Threshold) -> Self {
        self.witnesses = witnesses;
        self.witness_threshold = threshold;
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
            "t": "icp",
            "d": "",
            "i": self.prefix,
            "s": "0",
            "kt": threshold_str,
            "k": self.keys,
            "nt": "1",
            "n": next_keys,
            "bt": witness_threshold_str,
            "b": self.witnesses,
            "c": self.config,
            "a": []
        });

        Ok(ked)
    }
}

/// Builder for inception events
pub struct InceptionBuilder {
    keys: Vec<String>,
    threshold: Threshold,
    next_keys_digest: Option<String>,
    witnesses: Vec<String>,
    witness_threshold: Threshold,
    config: Vec<String>,
}

impl InceptionBuilder {
    /// Create new builder with required keys
    pub fn new(keys: Vec<String>) -> Self {
        InceptionBuilder {
            keys,
            threshold: Threshold::simple(1),
            next_keys_digest: None,
            witnesses: vec![],
            witness_threshold: Threshold::simple(0),
            config: vec![],
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

    /// Set witnesses
    pub fn witnesses(mut self, witnesses: Vec<String>) -> Self {
        self.witnesses = witnesses;
        self
    }

    /// Set witness threshold
    pub fn witness_threshold(mut self, t: Threshold) -> Self {
        self.witness_threshold = t;
        self
    }

    /// Add configuration trait
    pub fn config_trait(mut self, trait_name: &str) -> Self {
        self.config.push(trait_name.to_string());
        self
    }

    /// Build inception parameters (prefix derived from first key)
    pub fn build(self) -> CoreResult<InceptionParams> {
        if self.keys.is_empty() {
            return Err(CoreError::InvalidEvent(
                "At least one signing key required".to_string(),
            ));
        }

        // For basic derivation, prefix is the first signing key
        // In practice, this should be computed properly with cesride
        let prefix = self.keys[0].clone();

        Ok(InceptionParams {
            prefix,
            keys: self.keys,
            threshold: self.threshold,
            next_keys_digest: self.next_keys_digest,
            witnesses: self.witnesses,
            witness_threshold: self.witness_threshold,
            config: self.config,
        })
    }

    /// Build with explicit prefix
    pub fn build_with_prefix(self, prefix: String) -> CoreResult<InceptionParams> {
        if self.keys.is_empty() {
            return Err(CoreError::InvalidEvent(
                "At least one signing key required".to_string(),
            ));
        }

        Ok(InceptionParams {
            prefix,
            keys: self.keys,
            threshold: self.threshold,
            next_keys_digest: self.next_keys_digest,
            witnesses: self.witnesses,
            witness_threshold: self.witness_threshold,
            config: self.config,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inception_params_new() {
        let params = InceptionParams::new(
            "DTest123".to_string(),
            vec!["DKey1".to_string()],
        );
        assert_eq!(params.prefix, "DTest123");
        assert_eq!(params.keys.len(), 1);
        assert_eq!(params.threshold, Threshold::Simple(1));
    }

    #[test]
    fn test_inception_params_builder_chain() {
        let params = InceptionParams::new("DTest123".to_string(), vec!["DKey1".to_string()])
            .with_threshold(Threshold::simple(2))
            .with_next_keys("ENext123".to_string())
            .with_witnesses(vec!["BWit1".to_string()], Threshold::simple(1));

        assert_eq!(params.threshold, Threshold::Simple(2));
        assert_eq!(params.next_keys_digest, Some("ENext123".to_string()));
        assert_eq!(params.witnesses.len(), 1);
        assert_eq!(params.witness_threshold, Threshold::Simple(1));
    }

    #[test]
    fn test_inception_params_to_ked() {
        let params = InceptionParams::new(
            "DTest123".to_string(),
            vec!["DKey1".to_string()],
        );
        let ked = params.to_ked().unwrap();

        assert_eq!(ked["t"], "icp");
        assert_eq!(ked["i"], "DTest123");
        assert_eq!(ked["s"], "0");
    }

    #[test]
    fn test_inception_builder() {
        let params = InceptionBuilder::new(vec!["DKey1".to_string(), "DKey2".to_string()])
            .threshold(Threshold::simple(2))
            .witnesses(vec!["BWit1".to_string()])
            .witness_threshold(Threshold::simple(1))
            .config_trait("DND")
            .build()
            .unwrap();

        assert_eq!(params.keys.len(), 2);
        assert_eq!(params.threshold, Threshold::Simple(2));
        assert_eq!(params.witnesses.len(), 1);
        assert!(params.config.contains(&"DND".to_string()));
    }

    #[test]
    fn test_inception_builder_no_keys_fails() {
        let result = InceptionBuilder::new(vec![]).build();
        assert!(result.is_err());
    }

    #[test]
    fn test_inception_builder_with_prefix() {
        let params = InceptionBuilder::new(vec!["DKey1".to_string()])
            .build_with_prefix("DCustomPrefix".to_string())
            .unwrap();

        assert_eq!(params.prefix, "DCustomPrefix");
    }
}
