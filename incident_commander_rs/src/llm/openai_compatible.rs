use super::{GenerateRequest, GenerateResponse, LlmClient};
use anyhow::{bail, Context, Result};
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

pub struct OpenAiCompatibleClient {
    http: Client,
    base_url: String,
    model: String,
    api_key: Option<String>,
}

impl OpenAiCompatibleClient {
    pub fn new(base_url: String, model: String, api_key: Option<String>) -> Self {
        Self {
            http: Client::new(),
            base_url: base_url.trim_end_matches('/').to_string(),
            model,
            api_key,
        }
    }
}

#[derive(Serialize)]
struct ChatCompletionRequest<'a> {
    model: &'a str,
    messages: Vec<ChatMessage<'a>>,
    temperature: f32,
}

#[derive(Serialize)]
struct ChatMessage<'a> {
    role: &'a str,
    content: &'a str,
}

#[derive(Deserialize)]
struct ChatCompletionResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: ResponseMessage,
}

#[derive(Deserialize)]
struct ResponseMessage {
    content: String,
}

#[async_trait]
impl LlmClient for OpenAiCompatibleClient {
    async fn generate(&self, request: GenerateRequest) -> Result<GenerateResponse> {
        let url = format!("{}/v1/chat/completions", self.base_url);
        let mut builder = self.http.post(url).json(&ChatCompletionRequest {
            model: &self.model,
            messages: vec![
                ChatMessage {
                    role: "system",
                    content: "You are a careful internal SRE incident analyst.",
                },
                ChatMessage {
                    role: "user",
                    content: &request.prompt,
                },
            ],
            temperature: 0.2,
        });

        if let Some(api_key) = &self.api_key {
            builder = builder.bearer_auth(api_key);
        }

        let response = builder
            .send()
            .await
            .context("failed to call OpenAI-compatible endpoint")?
            .error_for_status()
            .context("OpenAI-compatible endpoint returned non-success status")?
            .json::<ChatCompletionResponse>()
            .await
            .context("failed to parse OpenAI-compatible response")?;

        let text = match response.choices.into_iter().next() {
            Some(choice) => choice.message.content,
            None => bail!("OpenAI-compatible endpoint returned no choices"),
        };

        Ok(GenerateResponse { text })
    }
}
