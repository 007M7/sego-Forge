use serde::{Deserialize, Serialize};

/// Execution phase status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PhaseStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Skipped,
}

/// A single phase in the workflow pipeline.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Phase {
    /// Unique key (e.g. "context", "exec")
    pub key: String,
    /// Human-readable name
    pub name: String,
    /// Current status
    pub status: PhaseStatus,
    /// Status message for display
    pub message: Option<String>,
    /// Duration in milliseconds
    pub elapsed_ms: Option<u64>,
    /// Arbitrary detail data (routed to work log)
    pub detail: Option<String>,
}

/// A unit of work within a session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub description: String,
    pub status: PhaseStatus,
    pub phases: Vec<Phase>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// Top-level execution session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: String,
    pub title: String,
    pub phases: Vec<Phase>,
    pub created_at: String,
    pub duration_ms: u64,
    pub completed: bool,
}
