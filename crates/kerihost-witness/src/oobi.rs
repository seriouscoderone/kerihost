//! OOBI (Out-Of-Band Introduction) handling
//!
//! OOBIs provide a way to discover witnesses and resolve identifiers.

use serde::{Deserialize, Serialize};

/// OOBI type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Oobi {
    /// Witness OOBI - introduces a witness
    Witness {
        /// Witness identifier
        aid: String,
        /// Witness URL
        url: String,
    },
    /// Controller OOBI - introduces a controller via its witness
    Controller {
        /// Controller identifier
        aid: String,
        /// Witness identifier
        witness: String,
        /// Witness URL
        url: String,
    },
    /// Well-known OOBI URL
    WellKnown {
        /// Well-known URL
        url: String,
    },
}

impl Oobi {
    /// Create a witness OOBI
    pub fn witness(aid: &str, url: &str) -> Self {
        Oobi::Witness {
            aid: aid.to_string(),
            url: url.to_string(),
        }
    }

    /// Create a controller OOBI
    pub fn controller(aid: &str, witness: &str, url: &str) -> Self {
        Oobi::Controller {
            aid: aid.to_string(),
            witness: witness.to_string(),
            url: url.to_string(),
        }
    }

    /// Create a well-known OOBI
    pub fn well_known(url: &str) -> Self {
        Oobi::WellKnown {
            url: url.to_string(),
        }
    }

    /// Get the URL from the OOBI
    pub fn url(&self) -> &str {
        match self {
            Oobi::Witness { url, .. } => url,
            Oobi::Controller { url, .. } => url,
            Oobi::WellKnown { url } => url,
        }
    }

    /// Convert to OOBI URL string
    pub fn to_url_string(&self) -> String {
        match self {
            Oobi::Witness { aid, url } => format!("{}/oobi/{}", url, aid),
            Oobi::Controller { aid, witness, url } => {
                format!("{}/oobi/{}/witness/{}", url, aid, witness)
            }
            Oobi::WellKnown { url } => url.clone(),
        }
    }
}

impl std::fmt::Display for Oobi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_url_string())
    }
}

/// Parse an OOBI URL into its components
pub fn parse_oobi_url(url: &str) -> Option<Oobi> {
    // Simple parsing of OOBI URLs
    // Format: {base}/oobi/{aid}/witness/{witness_aid}
    //      or {base}/oobi/{aid}

    if !url.contains("/oobi/") {
        return Some(Oobi::well_known(url));
    }

    let parts: Vec<&str> = url.split("/oobi/").collect();
    if parts.len() != 2 {
        return None;
    }

    let base_url = parts[0];
    let path = parts[1];

    if path.contains("/witness/") {
        let path_parts: Vec<&str> = path.split("/witness/").collect();
        if path_parts.len() == 2 {
            return Some(Oobi::controller(path_parts[0], path_parts[1], base_url));
        }
    } else if !path.is_empty() {
        // Simple witness or self OOBI
        let aid = path.split('/').next()?;
        return Some(Oobi::witness(aid, base_url));
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_witness_oobi() {
        let oobi = Oobi::witness("BWitness123", "https://witness.example.com");

        assert_eq!(
            oobi.to_url_string(),
            "https://witness.example.com/oobi/BWitness123"
        );
    }

    #[test]
    fn test_controller_oobi() {
        let oobi = Oobi::controller(
            "DController123",
            "BWitness123",
            "https://witness.example.com",
        );

        assert_eq!(
            oobi.to_url_string(),
            "https://witness.example.com/oobi/DController123/witness/BWitness123"
        );
    }

    #[test]
    fn test_well_known_oobi() {
        let oobi = Oobi::well_known("https://example.com/.well-known/keri/oobi");
        assert_eq!(oobi.url(), "https://example.com/.well-known/keri/oobi");
    }

    #[test]
    fn test_parse_witness_oobi() {
        let url = "https://witness.example.com/oobi/BWitness123";
        let oobi = parse_oobi_url(url).unwrap();

        match oobi {
            Oobi::Witness { aid, url } => {
                assert_eq!(aid, "BWitness123");
                assert_eq!(url, "https://witness.example.com");
            }
            _ => panic!("Expected Witness OOBI"),
        }
    }

    #[test]
    fn test_parse_controller_oobi() {
        let url = "https://witness.example.com/oobi/DController123/witness/BWitness123";
        let oobi = parse_oobi_url(url).unwrap();

        match oobi {
            Oobi::Controller { aid, witness, url } => {
                assert_eq!(aid, "DController123");
                assert_eq!(witness, "BWitness123");
                assert_eq!(url, "https://witness.example.com");
            }
            _ => panic!("Expected Controller OOBI"),
        }
    }

    #[test]
    fn test_parse_well_known_oobi() {
        let url = "https://example.com/.well-known/keri";
        let oobi = parse_oobi_url(url).unwrap();

        match oobi {
            Oobi::WellKnown { url } => {
                assert_eq!(url, "https://example.com/.well-known/keri");
            }
            _ => panic!("Expected WellKnown OOBI"),
        }
    }

    #[test]
    fn test_oobi_display() {
        let oobi = Oobi::witness("BWitness123", "https://witness.example.com");
        assert_eq!(
            format!("{}", oobi),
            "https://witness.example.com/oobi/BWitness123"
        );
    }

    #[test]
    fn test_oobi_serialization() {
        let oobi = Oobi::witness("BWitness123", "https://witness.example.com");
        let json = serde_json::to_string(&oobi).unwrap();
        assert!(json.contains("\"type\":\"witness\""));
        assert!(json.contains("\"aid\":\"BWitness123\""));
    }
}
