//! # forge-core
//!
//! Core data models, workflow engine, and state machine for Sego-Forge.
//! This crate is the shared foundation used by all other forge crates.

pub mod error;
pub mod model;
pub mod workflow;

/// Re-export commonly used types
pub use error::ForgeError;
pub use model::{Phase, PhaseStatus, Session, Task};
pub use workflow::WorkflowEngine;
