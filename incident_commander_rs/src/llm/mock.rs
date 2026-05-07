use super::{GenerateRequest, GenerateResponse, LlmClient};
use anyhow::Result;
use async_trait::async_trait;

pub struct MockLlmClient;

#[async_trait]
impl LlmClient for MockLlmClient {
    async fn generate(&self, request: GenerateRequest) -> Result<GenerateResponse> {
        let has_deploy = request.prompt.contains("\"version\"");
        let text = if has_deploy {
            "The most likely hypothesis is a regression or capacity issue introduced near the recent deployment window. Compare the deployed version against the error spike, inspect representative traces, and prepare rollback criteria if the blast radius grows."
        } else {
            "The current input is not enough to confirm a single root cause. Collect a wider log and metric window, then inspect upstream dependencies and saturation indicators."
        };

        Ok(GenerateResponse {
            text: text.to_string(),
        })
    }
}
