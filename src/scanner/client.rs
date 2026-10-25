//! Lightwalletd connection and network client interface.
//!
//! Note on Full Implementation Architecture:
//! In Phase 2, this module implements the full gRPC CompactTxStreamer client
//! (using tonic and prost) to connect to lightwalletd (e.g. testnet.zec.rocks:443),
//! stream CompactBlocks, and query raw transactions.

use std::time::Duration;

#[derive(Clone, Debug)]
pub struct LightwalletdClient {
    pub endpoint: String,
    http_client: reqwest::Client,
}

impl LightwalletdClient {
    pub fn new(endpoint: impl Into<String>) -> Self {
        let http_client = reqwest::Client::builder()
            .timeout(Duration::from_secs(5))
            .build()
            .unwrap_or_default();
        Self {
            endpoint: endpoint.into(),
            http_client,
        }
    }

    /// Verifies connectivity and liveness of the configured lightwalletd server.
    pub async fn check_liveness(&self) -> bool {
        self.http_client
            .get(&self.endpoint)
            .send()
            .await
            .map(|r| r.status().is_success() || r.status().is_redirection())
            .unwrap_or(false)
    }
}
