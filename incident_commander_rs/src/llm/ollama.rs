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
struct OllamaChatRequest<'a> {
    model: &'a str,
    messages: Vec<OllamaMessage<'a>>,
    stream: bool,
}

#[derive(Serialize)]
struct OllamaMessage<'a> {
    role: &'a str,
    content: &'a str,
}

#[derive(Deserialize)]
struct OllamaChatResponse {
    message: OllamaResponseMessage,
}

#[derive(Deserialize)]
struct OllamaResponseMessage {
    content: String,
}

#[async_trait]
impl LlmClient for OllamaClient {
    async fn generate(&self, request: GenerateRequest) -> Result<GenerateResponse> {
        let url = format!("{}/api/chat", self.base_url);
        let response = self
            .http
            .post(url)
            .json(&OllamaChatRequest {
                model: &self.model,
                messages: vec![
                    OllamaMessage {
                        role: "system",
                        content: "You are a careful internal SRE incident analyst.",
                    },
                    OllamaMessage {
                        role: "user",
                        content: &request.prompt,
                    },
                ],
                stream: false,
            })
            .send()
            .await
            .context("failed to call Ollama")?
            .error_for_status()
            .context("Ollama returned non-success status")?
            .json::<OllamaChatResponse>()
            .await
            .context("failed to parse Ollama response")?;

        Ok(GenerateResponse {
            text: response.message.content,
        })
    }
}
