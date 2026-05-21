use anyhow::Result;
use forge_core::WorkflowEngine;

pub async fn execute(config_path: &str) -> Result<()> {
    tracing::info!(config = %config_path, "Starting workflow");

    let mut engine = WorkflowEngine::new(format!("Workflow from {}", config_path));
    engine.add_phase("context", "Build execution context");
    engine.add_phase("exec", "Execute tasks");
    engine.add_phase("persist", "Save results");

    engine
        .execute_phase("context", || async {
            Ok("Context built (mock)".to_string())
        })
        .await?;

    engine
        .execute_phase("exec", || async { Ok("Tasks executed (mock)".to_string()) })
        .await?;

    engine
        .execute_phase("persist", || async {
            Ok("Results persisted (mock)".to_string())
        })
        .await?;

    let session = engine.finish();
    println!(
        "✅ Workflow complete: {}/{} phases",
        session
            .phases
            .iter()
            .filter(|p| matches!(p.status, forge_core::PhaseStatus::Completed))
            .count(),
        session.phases.len()
    );
    Ok(())
}
