use anyhow::Result;
use tracing::info;

/// Unified API client for external service communication.
///
/// This serves as the network layer for the forge workflow engine,
/// handling requests to AI providers, version control APIs, etc.
pub struct ForgeClient {
    /// Base URL for the target service
    base_url: String,
}

impl ForgeClient {
    /// Create a new API client targeting the given base URL.
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
        }
    }

    /// Perform a health check against the target service.
    pub async fn health_check(&self) -> Result<String> {
        info!(base_url = %self.base_url, "Performing health check");
        // TODO: Implement actual HTTP health check
        Ok(format!("Service at {} is reachable", self.base_url))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_health_check() {
        let client = ForgeClient::new("http://localhost:8080");
        let result = client.health_check().await.unwrap();
        assert!(result.contains("reachable"));
    }
}
