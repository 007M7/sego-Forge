use crate::error::ForgeResult;
use crate::model::{Phase, PhaseStatus, Session};
use tracing::{debug, info};

/// Executes a sequence of phases in order, collecting timing and status.
pub struct WorkflowEngine {
    session: Session,
}

impl WorkflowEngine {
    /// Create a new workflow engine for a given session.
    pub fn new(title: String) -> Self {
        let session = Session {
            id: uuid::Uuid::new_v4().to_string(),
            title,
            phases: Vec::new(),
            created_at: chrono::Utc::now().to_rfc3339(),
            duration_ms: 0,
            completed: false,
        };
        Self { session }
    }

    /// Register a phase in the workflow.
    pub fn add_phase(&mut self, key: &str, name: &str) {
        self.session.phases.push(Phase {
            key: key.to_string(),
            name: name.to_string(),
            status: PhaseStatus::Pending,
            message: None,
            elapsed_ms: None,
            detail: None,
        });
        debug!(phase = key, "Registered phase");
    }

    /// Execute a single phase with the given closure.
    pub async fn execute_phase<F, Fut>(&mut self, key: &str, mut f: F) -> ForgeResult<()>
    where
        F: FnMut() -> Fut,
        Fut: std::future::Future<Output = ForgeResult<String>>,
    {
        let phase = self
            .session
            .phases
            .iter_mut()
            .find(|p| p.key == key)
            .ok_or_else(|| crate::error::ForgeError::PhaseNotFound(key.to_string()))?;

        phase.status = PhaseStatus::Running;
        let start = std::time::Instant::now();
        info!(phase = key, "Starting phase");

        match f().await {
            Ok(detail) => {
                phase.status = PhaseStatus::Completed;
                phase.elapsed_ms = Some(start.elapsed().as_millis() as u64);
                phase.detail = Some(detail);
                info!(
                    phase = key,
                    elapsed_ms = phase.elapsed_ms,
                    "Phase completed"
                );
            }
            Err(e) => {
                phase.status = PhaseStatus::Failed;
                phase.message = Some(e.to_string());
                return Err(e);
            }
        }
        Ok(())
    }

    /// Finalize the session and return it.
    pub fn finish(mut self) -> Session {
        self.session.completed = true;
        self.session.duration_ms = self
            .session
            .phases
            .iter()
            .filter_map(|p| p.elapsed_ms)
            .sum();
        info!(
            total_ms = self.session.duration_ms,
            phases = self.session.phases.len(),
            "Workflow complete"
        );
        self.session
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_add_and_execute_phase() {
        let mut engine = WorkflowEngine::new("test".into());
        engine.add_phase("init", "Initialize");
        engine.add_phase("run", "Execute");

        engine
            .execute_phase("init", || async { Ok("initialized".to_string()) })
            .await
            .unwrap();

        let session = engine.finish();
        assert_eq!(session.phases.len(), 2);
        assert_eq!(session.phases[0].status, PhaseStatus::Completed);
        assert_eq!(session.phases[1].status, PhaseStatus::Pending);
    }

    #[tokio::test]
    async fn test_phase_not_found() {
        let mut engine = WorkflowEngine::new("test".into());
        let result = engine
            .execute_phase("nonexistent", || async { Ok("nope".to_string()) })
            .await;
        assert!(result.is_err());
    }
}
