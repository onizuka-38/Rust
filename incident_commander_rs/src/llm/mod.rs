mod mock;
mod ollama;
mod openai_compatible;

use anyhow::Result;
use async_trait::async_trait;
use mock::MockLlmClient;
use ollama::OllamaClient;
use openai_compatible::OpenAiCompatibleClient;

#[derive(Debug, Clone)]
pub struct GenerateRequest {
    pub prompt: String,
}

#[derive(Debug, Clone)]
pub struct GenerateResponse {
    pub text: String,
}

#[async_trait]
pub trait LlmClient: Send + Sync {
    async fn generate(&self, request: GenerateRequest) -> Result<GenerateResponse>;
}

#[derive(Debug, Clone)]
pub enum ProviderKind {
    Mock,
    Ollama,
    OpenAiCompatible,
}

#[derive(Debug, Clone)]
pub struct LlmConfig {
    pub provider: ProviderKind,
    pub base_url: String,
    pub model: String,
    pub api_key: Option<String>,
}

pub enum LlmBackend {
    Mock(MockLlmClient),
    Ollama(OllamaClient),
    OpenAiCompatible(OpenAiCompatibleClient),
}

impl LlmBackend {
    pub fn from_config(config: LlmConfig) -> Self {
        match config.provider {
            ProviderKind::Mock => Self::Mock(MockLlmClient),
            ProviderKind::Ollama => Self::Ollama(OllamaClient::new(config.base_url, config.model)),
            ProviderKind::OpenAiCompatible => Self::OpenAiCompatible(OpenAiCompatibleClient::new(
                config.base_url,
                config.model,
                config.api_key,
            )),
        }
    }
}

#[async_trait]
impl LlmClient for LlmBackend {
    async fn generate(&self, request: GenerateRequest) -> Result<GenerateResponse> {
        match self {
            Self::Mock(client) => client.generate(request).await,
            Self::Ollama(client) => client.generate(request).await,
            Self::OpenAiCompatible(client) => client.generate(request).await,
        }
    }
}
