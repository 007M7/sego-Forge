use thiserror::Error;

#[derive(Error, Debug)]
pub enum ForgeError {
    #[error("Workflow error: {0}")]
    Workflow(String),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Phase not found: {0}")]
    PhaseNotFound(String),

    #[error("Task already completed: {0}")]
    TaskCompleted(String),
}

pub type ForgeResult<T> = Result<T, ForgeError>;
