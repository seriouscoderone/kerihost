//! kerihost-db: Database abstraction layer for kerihost
//!
//! This crate provides the database traits and implementations for storing
//! KERI data including:
//! - Key Event Log (KEL) storage
//! - Key state storage
//! - Receipt storage
//! - Escrow storage
//!
//! # Implementations
//!
//! - `DynamoDbDatabase`: Production implementation using AWS DynamoDB
//! - `InMemoryDatabase`: Testing implementation using in-memory storage

pub mod dynamodb;
pub mod error;
pub mod memory;
pub mod traits;

pub use error::*;
pub use traits::*;

// Re-export implementations
pub use dynamodb::DynamoDbDatabase;
pub use memory::InMemoryDatabase;
