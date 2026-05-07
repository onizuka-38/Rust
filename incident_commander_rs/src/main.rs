mod analysis;
mod api;
mod llm;
mod model;
mod report;

use anyhow::Result;
use clap::{Parser, ValueEnum};
use llm::{LlmBackend, LlmConfig};
use std::net::SocketAddr;
use std::sync::Arc;
use tracing::info;

#[derive(Parser, Debug)]
#[command(name = "incident-commander-rs")]
#[command(about = "AI-assisted incident triage service for internal operations teams")]
struct Cli {
    #[arg(long, default_value = "127.0.0.1:8088")]
    listen: SocketAddr,

    #[arg(long, value_enum, default_value_t = Provider::Mock)]
    provider: Provider,

    #[arg(long, env = "LLM_BASE_URL", default_value = "http://localhost:11434")]
    llm_base_url: String,

    #[arg(long, env = "LLM_MODEL", default_value = "qwen3.6:35b")]
    llm_model: String,

    #[arg(long, env = "LLM_API_KEY")]
    llm_api_key: Option<String>,
}

#[derive(Clone, Debug, ValueEnum)]
enum Provider {
    Mock,
    Ollama,
    OpenaiCompatible,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "incident_commander_rs=info,tower_http=info".into()),
        )
        .init();

    let cli = Cli::parse();
    let llm = LlmBackend::from_config(LlmConfig {
        provider: match cli.provider {
            Provider::Mock => llm::ProviderKind::Mock,
            Provider::Ollama => llm::ProviderKind::Ollama,
            Provider::OpenaiCompatible => llm::ProviderKind::OpenAiCompatible,
        },
        base_url: cli.llm_base_url,
        model: cli.llm_model,
        api_key: cli.llm_api_key,
    });

    let app = api::router(Arc::new(llm));
    info!("listening on http://{}", cli.listen);

    let listener = tokio::net::TcpListener::bind(cli.listen).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
