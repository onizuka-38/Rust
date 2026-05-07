use super::{GenerateRequest, GenerateResponse, LlmClient};
use anyhow::{Context, Result};
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

pub struct OllamaClient {
    http: Client,
    base_url: String,
    model: String,
}

impl OllamaClient {
    pub fn new(base_url: String, model: String) -> Self {
        Self {
            http: Client::new(),
            base_url: base_url.trim_end_matches('/').to_string(),
            model,
        }
    }
}

#[derive(Serialize)]
struct OllamaGenerateRequest<'a> {
    model: &'a str,
    prompt: &'a str,
    stream: bool,
}

#[derive(Deserialize)]
struct OllamaGenerateResponse {
    response: String,
}

#[async_trait]
impl LlmClient for OllamaClient {
    async fn generate(&self, request: GenerateRequest) -> Result<GenerateResponse> {
        let url = format!("{}/api/generate", self.base_url);
        let response = self
            .http
            .post(url)
            .json(&OllamaGenerateRequest {
                model: &self.model,
                prompt: &request.prompt,
                stream: false,
            })
            .send()
            .await
            .context("failed to call Ollama")?
            .error_for_status()
            .context("Ollama returned non-success status")?
            .json::<OllamaGenerateResponse>()
            .await
            .context("failed to parse Ollama response")?;

        Ok(GenerateResponse {
            text: response.response,
        })
    }
}
