use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IncidentInput {
    pub title: String,
    #[serde(default)]
    pub alerts: Vec<AlertEvent>,
    #[serde(default)]
    pub deployments: Vec<DeploymentEvent>,
    #[serde(default)]
    pub logs: Vec<LogEvent>,
}

impl IncidentInput {
    pub fn validate(&self) -> Result<(), String> {
        if self.title.trim().is_empty() {
            return Err("title must not be empty".to_string());
        }
        if self.alerts.is_empty() && self.deployments.is_empty() && self.logs.is_empty() {
            return Err("at least one alert, deployment, or log event is required".to_string());
        }
        for (idx, alert) in self.alerts.iter().enumerate() {
            require_non_empty(&alert.service, format!("alerts[{idx}].service"))?;
            require_non_empty(&alert.severity, format!("alerts[{idx}].severity"))?;
            require_non_empty(&alert.message, format!("alerts[{idx}].message"))?;
            require_non_empty(&alert.ts, format!("alerts[{idx}].ts"))?;
        }
        for (idx, deployment) in self.deployments.iter().enumerate() {
            require_non_empty(&deployment.service, format!("deployments[{idx}].service"))?;
            require_non_empty(&deployment.version, format!("deployments[{idx}].version"))?;
            require_non_empty(&deployment.actor, format!("deployments[{idx}].actor"))?;
            require_non_empty(&deployment.ts, format!("deployments[{idx}].ts"))?;
        }
        for (idx, log) in self.logs.iter().enumerate() {
            require_non_empty(&log.service, format!("logs[{idx}].service"))?;
            require_non_empty(&log.level, format!("logs[{idx}].level"))?;
            require_non_empty(&log.message, format!("logs[{idx}].message"))?;
            require_non_empty(&log.ts, format!("logs[{idx}].ts"))?;
        }
        Ok(())
    }
}

fn require_non_empty(value: &str, field: String) -> Result<(), String> {
    if value.trim().is_empty() {
        Err(format!("{field} must not be empty"))
    } else {
        Ok(())
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AlertEvent {
    pub service: String,
    pub severity: String,
    pub message: String,
    pub ts: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DeploymentEvent {
    pub service: String,
    pub version: String,
    pub actor: String,
    pub ts: String,
    #[serde(default)]
    pub commit: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LogEvent {
    pub service: String,
    pub level: String,
    pub message: String,
    pub ts: String,
    #[serde(default)]
    pub trace_id: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum IncidentSeverity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize)]
pub struct TimelineEntry {
    pub ts: String,
    pub kind: String,
    pub service: String,
    pub summary: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct RunbookMatch {
    pub id: String,
    pub title: String,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ServiceImpact {
    pub service: String,
    pub error_logs: usize,
    pub warn_logs: usize,
    pub alert_count: usize,
    pub sample_trace_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct EvidenceSignal {
    pub signal_type: String,
    pub service: Option<String>,
    pub severity: String,
    pub summary: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct DeploymentCorrelation {
    pub service: String,
    pub version: String,
    pub commit: Option<String>,
    pub deployment_ts: String,
    pub first_error_ts: Option<String>,
    pub correlation: String,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct RollbackAdvice {
    pub should_prepare_rollback: bool,
    pub target_services: Vec<String>,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct DetectionSummary {
    pub severity: IncidentSeverity,
    pub risk_score: u8,
    pub confidence_score: u8,
    pub total_logs: usize,
    pub error_logs: usize,
    pub warn_logs: usize,
    pub affected_services: Vec<String>,
    pub service_impacts: Vec<ServiceImpact>,
    pub suspected_deployments: Vec<DeploymentEvent>,
    pub deployment_correlations: Vec<DeploymentCorrelation>,
    pub representative_errors: Vec<LogEvent>,
    pub timeline: Vec<TimelineEntry>,
    pub evidence_signals: Vec<EvidenceSignal>,
    pub runbook_matches: Vec<RunbookMatch>,
    pub rollback_advice: RollbackAdvice,
}

#[derive(Debug, Clone, Serialize)]
pub struct IncidentReport {
    pub title: String,
    pub detection: DetectionSummary,
    pub ai_summary: String,
    pub recommended_actions: Vec<String>,
    pub markdown: String,
}
