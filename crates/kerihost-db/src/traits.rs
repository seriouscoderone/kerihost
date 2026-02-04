//! Database traits for KERI storage
//!
//! These traits define the interface for storing and retrieving KERI data.
//! Implementations can use different backends (DynamoDB, in-memory, etc.)

use crate::error::DbResult;
use async_trait::async_trait;
use kerihost_core::{KeyState, NontransferableReceipt, SignedEvent};
use serde::{Deserialize, Serialize};

/// Key Event Log storage
#[async_trait]
pub trait KelStore: Send + Sync {
    /// Append event to KEL
    ///
    /// For inception events (sn=0): Uses attribute_not_exists condition
    /// For subsequent events: Uses prior_digest condition check
    ///
    /// This ensures proper ordering and prevents duplicates.
    async fn append_event(&self, event: &SignedEvent) -> DbResult<()>;

    /// Get event by prefix and sequence number
    async fn get_event(&self, prefix: &str, sn: u64) -> DbResult<Option<SignedEvent>>;

    /// Get events in a range
    async fn get_events(
        &self,
        prefix: &str,
        start_sn: u64,
        end_sn: Option<u64>,
    ) -> DbResult<Vec<SignedEvent>>;

    /// Get latest event for prefix
    async fn get_latest(&self, prefix: &str) -> DbResult<Option<SignedEvent>>;

    /// Get event by digest
    async fn get_event_by_digest(&self, prefix: &str, digest: &str) -> DbResult<Option<SignedEvent>>;
}

/// Key state storage
#[async_trait]
pub trait StateStore: Send + Sync {
    /// Get current state for identifier
    async fn get_state(&self, prefix: &str) -> DbResult<Option<KeyState>>;

    /// Put/update state for identifier
    async fn put_state(&self, state: &KeyState) -> DbResult<()>;

    /// Delete state (for testing)
    async fn delete_state(&self, prefix: &str) -> DbResult<()>;
}

/// Receipt storage
#[async_trait]
pub trait ReceiptStore: Send + Sync {
    /// Add a receipt
    async fn add_receipt(&self, receipt: &NontransferableReceipt) -> DbResult<()>;

    /// Get receipts for an event
    async fn get_receipts(&self, event_digest: &str) -> DbResult<Vec<NontransferableReceipt>>;

    /// Get receipt from specific witness
    async fn get_receipt(
        &self,
        event_digest: &str,
        witness_prefix: &str,
    ) -> DbResult<Option<NontransferableReceipt>>;

    /// Count receipts for an event
    async fn count_receipts(&self, event_digest: &str) -> DbResult<usize>;
}

/// Escrow storage
#[async_trait]
pub trait EscrowStore: Send + Sync {
    /// Escrow an event
    async fn escrow_event(&self, event: &SignedEvent, reason: EscrowReason) -> DbResult<()>;

    /// Get escrowed events for a prefix
    async fn get_escrowed(&self, prefix: &str) -> DbResult<Vec<EscrowedEvent>>;

    /// Get all escrowed events (for scheduled processing)
    async fn get_all_escrowed(&self) -> DbResult<Vec<EscrowedEvent>>;

    /// Promote escrowed event (move to KEL)
    async fn promote_escrowed(&self, event_digest: &str) -> DbResult<Option<SignedEvent>>;

    /// Remove escrowed event
    async fn remove_escrowed(&self, event_digest: &str) -> DbResult<()>;
}

/// Reasons for escrowing an event
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EscrowReason {
    /// Missing prior events
    OutOfOrder,
    /// Not enough signatures yet
    PartiallySigned,
    /// Waiting for delegator approval
    MissingDelegator,
    /// Waiting for witness receipts
    MissingReceipts,
}

impl std::fmt::Display for EscrowReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EscrowReason::OutOfOrder => write!(f, "out_of_order"),
            EscrowReason::PartiallySigned => write!(f, "partially_signed"),
            EscrowReason::MissingDelegator => write!(f, "missing_delegator"),
            EscrowReason::MissingReceipts => write!(f, "missing_receipts"),
        }
    }
}

/// Escrowed event with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EscrowedEvent {
    /// The escrowed event
    pub event: SignedEvent,
    /// Reason for escrow
    pub reason: EscrowReason,
    /// When it was escrowed (ISO 8601)
    pub created: String,
    /// TTL timestamp (Unix epoch seconds)
    pub ttl: u64,
}

impl EscrowedEvent {
    /// Create new escrowed event
    pub fn new(event: SignedEvent, reason: EscrowReason, ttl_seconds: u64) -> Self {
        let now = chrono::Utc::now();
        EscrowedEvent {
            event,
            reason,
            created: now.to_rfc3339(),
            ttl: (now.timestamp() as u64) + ttl_seconds,
        }
    }

    /// Check if escrow has expired
    pub fn is_expired(&self) -> bool {
        let now = chrono::Utc::now().timestamp() as u64;
        now >= self.ttl
    }
}

/// Combined database interface
///
/// Implementations should implement all traits to provide
/// full database functionality.
pub trait WitnessDatabase: KelStore + StateStore + ReceiptStore + EscrowStore {}

// Blanket implementation for any type that implements all traits
impl<T> WitnessDatabase for T where T: KelStore + StateStore + ReceiptStore + EscrowStore {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escrow_reason_display() {
        assert_eq!(EscrowReason::OutOfOrder.to_string(), "out_of_order");
        assert_eq!(EscrowReason::PartiallySigned.to_string(), "partially_signed");
        assert_eq!(EscrowReason::MissingDelegator.to_string(), "missing_delegator");
        assert_eq!(EscrowReason::MissingReceipts.to_string(), "missing_receipts");
    }

    #[test]
    fn test_escrow_reason_serialization() {
        let reason = EscrowReason::OutOfOrder;
        let json = serde_json::to_string(&reason).unwrap();
        assert_eq!(json, "\"out_of_order\"");

        let parsed: EscrowReason = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, reason);
    }
}
