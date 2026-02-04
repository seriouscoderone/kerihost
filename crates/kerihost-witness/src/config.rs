//! Witness configuration

use serde::{Deserialize, Serialize};

/// Witness configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WitnessConfig {
    /// Witness identifier prefix
    pub prefix: String,

    /// Public URL for this witness
    pub public_url: String,

    /// Escrow TTL in seconds
    pub escrow_ttl: u64,

    /// Maximum events to process per batch
    pub max_batch_size: usize,

    /// Whether to validate signatures strictly
    pub strict_validation: bool,
}

impl WitnessConfig {
    /// Create config from environment variables
    pub fn from_env() -> Self {
        WitnessConfig {
            prefix: std::env::var("WITNESS_PREFIX").unwrap_or_default(),
            public_url: std::env::var("PUBLIC_URL")
                .unwrap_or_else(|_| "https://witness.keri.host".to_string()),
            escrow_ttl: std::env::var("ESCROW_TTL")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(3600),
            max_batch_size: std::env::var("MAX_BATCH_SIZE")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(100),
            strict_validation: std::env::var("STRICT_VALIDATION")
                .ok()
                .map(|s| s == "true" || s == "1")
                .unwrap_or(true),
        }
    }

    /// Create with custom values
    pub fn new(prefix: String, public_url: String) -> Self {
        WitnessConfig {
            prefix,
            public_url,
            escrow_ttl: 3600,
            max_batch_size: 100,
            strict_validation: true,
        }
    }

    /// Builder-style method to set escrow TTL
    pub fn with_escrow_ttl(mut self, ttl: u64) -> Self {
        self.escrow_ttl = ttl;
        self
    }

    /// Builder-style method to set strict validation
    pub fn with_strict_validation(mut self, strict: bool) -> Self {
        self.strict_validation = strict;
        self
    }
}

impl Default for WitnessConfig {
    fn default() -> Self {
        WitnessConfig {
            prefix: String::new(),
            public_url: "https://witness.keri.host".to_string(),
            escrow_ttl: 3600,
            max_batch_size: 100,
            strict_validation: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = WitnessConfig::default();
        assert_eq!(config.escrow_ttl, 3600);
        assert!(config.strict_validation);
    }

    #[test]
    fn test_config_builder() {
        let config = WitnessConfig::new("BTest123".to_string(), "https://example.com".to_string())
            .with_escrow_ttl(7200)
            .with_strict_validation(false);

        assert_eq!(config.prefix, "BTest123");
        assert_eq!(config.escrow_ttl, 7200);
        assert!(!config.strict_validation);
    }
}
