//! kerihost-witness: Witness business logic for kerihost
//!
//! This crate provides the core witness functionality:
//! - Event processing and validation
//! - Receipt generation
//! - Escrow handling
//! - OOBI generation and resolution
//!
//! # KERI-Honest Design
//!
//! This implementation follows KERI-honest principles:
//! - Never claim "FINAL" - use confidence levels instead
//! - Escrow is state, not error
//! - Cryptographic eventual finality
//! - Explicit confidence qualifiers

pub mod config;
pub mod error;
pub mod escrow;
pub mod oobi;
pub mod processor;
pub mod receipt_generator;
pub mod witness;

pub use config::*;
pub use error::*;
pub use processor::*;
pub use witness::*;
