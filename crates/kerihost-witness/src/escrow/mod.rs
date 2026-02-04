//! Escrow handling
//!
//! Events that cannot be immediately processed are escrowed.
//! This module provides utilities for managing escrowed events.

use kerihost_db::{EscrowReason, EscrowedEvent};

/// Check if an escrowed event has expired
pub fn is_expired(escrowed: &EscrowedEvent) -> bool {
    escrowed.is_expired()
}

/// Get the escrow reason as a string
pub fn reason_string(reason: EscrowReason) -> &'static str {
    match reason {
        EscrowReason::OutOfOrder => "out_of_order",
        EscrowReason::PartiallySigned => "partially_signed",
        EscrowReason::MissingDelegator => "missing_delegator",
        EscrowReason::MissingReceipts => "missing_receipts",
    }
}

/// Parse escrow reason from string
pub fn parse_reason(s: &str) -> Option<EscrowReason> {
    match s {
        "out_of_order" => Some(EscrowReason::OutOfOrder),
        "partially_signed" => Some(EscrowReason::PartiallySigned),
        "missing_delegator" => Some(EscrowReason::MissingDelegator),
        "missing_receipts" => Some(EscrowReason::MissingReceipts),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reason_string() {
        assert_eq!(reason_string(EscrowReason::OutOfOrder), "out_of_order");
        assert_eq!(reason_string(EscrowReason::PartiallySigned), "partially_signed");
    }

    #[test]
    fn test_parse_reason() {
        assert_eq!(parse_reason("out_of_order"), Some(EscrowReason::OutOfOrder));
        assert_eq!(parse_reason("invalid"), None);
    }

    #[test]
    fn test_roundtrip() {
        let reason = EscrowReason::MissingDelegator;
        let s = reason_string(reason);
        let parsed = parse_reason(s);
        assert_eq!(parsed, Some(reason));
    }
}
