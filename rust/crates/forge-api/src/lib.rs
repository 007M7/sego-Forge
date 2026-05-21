//! # forge-api
//!
//! API client layer — handles HTTP communication with external services.
//! Abstracts away provider-specific details behind a unified interface.

pub mod client;

pub use client::ForgeClient;
