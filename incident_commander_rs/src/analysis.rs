use crate::llm::{GenerateRequest, LlmClient};
use crate::model::{
    DeploymentCorrelation, DeploymentEvent, DetectionSummary, EvidenceSignal, IncidentInput,
    IncidentReport, IncidentSeverity, RollbackAdvice, RunbookMatch, ServiceImpact, TimelineEntry,
};
use crate::report::render_markdown;
use anyhow::Result;
use std::collections::{BTreeMap, BTreeSet};

pub async fn analyze_incident(input: IncidentInput, llm: &dyn LlmClient) -> Result<IncidentReport> {
    let detection = detect(&input);
    let prompt = build_prompt(&input, &detection);
    let ai = llm.generate(GenerateRequest { prompt }).await?;
    let actions = recommended_actions(&detection);
    let markdown = render_markdown(&input.title, &detection, &ai.text, &actions);

    Ok(IncidentReport {
        title: input.title,
        detection,
        ai_summary: ai.text,
        recommended_actions: actions,
        markdown,
    })
}

fn detect(input: &IncidentInput) -> DetectionSummary {
    let mut affected = BTreeSet::new();
    let mut by_service_errors: BTreeMap<String, usize> = BTreeMap::new();
    let mut representative_errors = Vec::new();
    let mut error_logs = 0usize;
    let mut warn_logs = 0usize;

    for log in &input.logs {
        let level = log.level.to_ascii_lowercase();
        if level == "error" || level == "fatal" {
            error_logs += 1;
            affected.insert(log.service.clone());
            *by_service_errors.entry(log.service.clone()).or_default() += 1;
            if representative_errors.len() < 8 {
                representative_errors.push(log.clone());
            }
        } else if level == "warn" || level == "warning" {
            warn_logs += 1;
            affected.insert(log.service.clone());
        }
    }

    for alert in &input.alerts {
        affected.insert(alert.service.clone());
    }

    let suspected_deployments = input
        .deployments
        .iter()
        .filter(|deploy| affected.contains(&deploy.service))
        .cloned()
        .collect::<Vec<_>>();

    let risk_score = risk_score(input, error_logs, warn_logs, affected.len(), suspected_deployments.len());
    let severity = severity_from_score(risk_score);
    let timeline = build_timeline(input);

    let affected_services = ordered_affected_services(by_service_errors, affected);
    let service_impacts = build_service_impacts(input);
    let deployment_correlations = correlate_deployments(input, &suspected_deployments);
    let evidence_signals = collect_evidence_signals(input);
    let runbook_matches = match_runbooks(input);
    let confidence_score = confidence_score(
        input,
        error_logs,
        suspected_deployments.len(),
        deployment_correlations.len(),
        runbook_matches.len(),
    );
    let rollback_advice = rollback_advice(risk_score, &deployment_correlations);

    DetectionSummary {
        severity,
        risk_score,
        confidence_score,
        total_logs: input.logs.len(),
        error_logs,
        warn_logs,
        affected_services,
        service_impacts,
        suspected_deployments,
        deployment_correlations,
        representative_errors,
        timeline,
        evidence_signals,
        runbook_matches,
        rollback_advice,
    }
}

fn build_prompt(input: &IncidentInput, detection: &DetectionSummary) -> String {
    let alerts = serde_json::to_string_pretty(&input.alerts).unwrap_or_else(|_| "[]".to_string());
    let deployments =
        serde_json::to_string_pretty(&detection.suspected_deployments).unwrap_or_else(|_| "[]".to_string());
    let errors =
        serde_json::to_string_pretty(&detection.representative_errors).unwrap_or_else(|_| "[]".to_string());

    format!(
        r#"You are an internal SRE incident commander.
Analyze the incident using only the provided alerts, deployments, and logs.
Return a concise root-cause hypothesis, blast radius, confidence, and next actions.
Do not invent systems or facts that are not present.

Incident title: {title}

Detected metrics:
- total_logs: {total_logs}
- error_logs: {error_logs}
- warn_logs: {warn_logs}
- severity: {severity:?}
- risk_score: {risk_score}/100
- confidence_score: {confidence_score}/100
- affected_services: {affected_services:?}
- service_impacts: {service_impacts}
- deployment_correlations: {deployment_correlations}
- evidence_signals: {evidence_signals}
- runbook_matches: {runbook_matches}
- rollback_advice: {rollback_advice}

Alerts:
{alerts}

Suspected deployments:
{deployments}

Representative errors:
{errors}
"#,
        title = input.title,
        total_logs = detection.total_logs,
        error_logs = detection.error_logs,
        warn_logs = detection.warn_logs,
        severity = detection.severity,
        risk_score = detection.risk_score,
        confidence_score = detection.confidence_score,
        affected_services = &detection.affected_services,
        service_impacts = serde_json::to_string_pretty(&detection.service_impacts)
            .unwrap_or_else(|_| "[]".to_string()),
        deployment_correlations = serde_json::to_string_pretty(&detection.deployment_correlations)
            .unwrap_or_else(|_| "[]".to_string()),
        evidence_signals = serde_json::to_string_pretty(&detection.evidence_signals)
            .unwrap_or_else(|_| "[]".to_string()),
        runbook_matches = serde_json::to_string_pretty(&detection.runbook_matches)
            .unwrap_or_else(|_| "[]".to_string()),
        rollback_advice = serde_json::to_string_pretty(&detection.rollback_advice)
            .unwrap_or_else(|_| "{}".to_string()),
    )
}

fn recommended_actions(detection: &DetectionSummary) -> Vec<String> {
    let mut actions = Vec::new();

    if !detection.suspected_deployments.is_empty() {
        actions.push(
            "Compare recent deployments against the error spike window and prepare a rollback if the blast radius expands."
                .to_string(),
        );
    }
    if detection.error_logs > 0 {
        actions.push(
            "Inspect representative trace_id values and verify upstream/downstream dependency failures."
                .to_string(),
        );
    }
    if !detection.affected_services.is_empty() {
        actions.push(format!(
            "Check latency, error rate, and saturation dashboards for affected services: {}.",
            detection.affected_services.join(", ")
        ));
    }
    if actions.is_empty() {
        actions.push("Collect additional logs and metrics, then rerun the analysis.".to_string());
    }

    actions
}

fn risk_score(
    input: &IncidentInput,
    error_logs: usize,
    warn_logs: usize,
    affected_services: usize,
    suspected_deployments: usize,
) -> u8 {
    let mut score = 0u16;
    score += (error_logs as u16).saturating_mul(8).min(40);
    score += (warn_logs as u16).saturating_mul(3).min(15);
    score += (affected_services as u16).saturating_mul(10).min(25);
    score += (suspected_deployments as u16).saturating_mul(10).min(20);

    for alert in &input.alerts {
        let severity = alert.severity.to_ascii_lowercase();
        if severity == "critical" || severity == "page" || severity == "sev1" {
            score += 25;
        } else if severity == "warning" || severity == "sev2" {
            score += 10;
        }
    }

    score.min(100) as u8
}

fn severity_from_score(score: u8) -> IncidentSeverity {
    match score {
        0..=24 => IncidentSeverity::Low,
        25..=49 => IncidentSeverity::Medium,
        50..=79 => IncidentSeverity::High,
        _ => IncidentSeverity::Critical,
    }
}

fn build_timeline(input: &IncidentInput) -> Vec<TimelineEntry> {
    let mut timeline = Vec::new();

    for alert in &input.alerts {
        timeline.push(TimelineEntry {
            ts: alert.ts.clone(),
            kind: "alert".to_string(),
            service: alert.service.clone(),
            summary: format!("{}: {}", alert.severity, alert.message),
        });
    }

    for deploy in &input.deployments {
        timeline.push(TimelineEntry {
            ts: deploy.ts.clone(),
            kind: "deployment".to_string(),
            service: deploy.service.clone(),
            summary: format!("version {} by {}", deploy.version, deploy.actor),
        });
    }

    for log in input.logs.iter().take(20) {
        timeline.push(TimelineEntry {
            ts: log.ts.clone(),
            kind: "log".to_string(),
            service: log.service.clone(),
            summary: format!("{}: {}", log.level, log.message),
        });
    }

    timeline.sort_by(|a, b| a.ts.cmp(&b.ts).then_with(|| a.kind.cmp(&b.kind)));
    timeline
}

fn build_service_impacts(input: &IncidentInput) -> Vec<ServiceImpact> {
    let mut impacts: BTreeMap<String, ServiceImpact> = BTreeMap::new();

    for alert in &input.alerts {
        let entry = impacts.entry(alert.service.clone()).or_insert_with(|| ServiceImpact {
            service: alert.service.clone(),
            error_logs: 0,
            warn_logs: 0,
            alert_count: 0,
            sample_trace_ids: Vec::new(),
        });
        entry.alert_count += 1;
    }

    for log in &input.logs {
        let entry = impacts.entry(log.service.clone()).or_insert_with(|| ServiceImpact {
            service: log.service.clone(),
            error_logs: 0,
            warn_logs: 0,
            alert_count: 0,
            sample_trace_ids: Vec::new(),
        });

        let level = log.level.to_ascii_lowercase();
        if level == "error" || level == "fatal" {
            entry.error_logs += 1;
        } else if level == "warn" || level == "warning" {
            entry.warn_logs += 1;
        }

        if let Some(trace_id) = &log.trace_id {
            if entry.sample_trace_ids.len() < 3 && !entry.sample_trace_ids.contains(trace_id) {
                entry.sample_trace_ids.push(trace_id.clone());
            }
        }
    }

    let mut out = impacts.into_values().collect::<Vec<_>>();
    out.sort_by(|a, b| {
        b.error_logs
            .cmp(&a.error_logs)
            .then_with(|| b.alert_count.cmp(&a.alert_count))
            .then_with(|| a.service.cmp(&b.service))
    });
    out
}

fn correlate_deployments(
    input: &IncidentInput,
    suspected_deployments: &[DeploymentEvent],
) -> Vec<DeploymentCorrelation> {
    suspected_deployments
        .iter()
        .map(|deployment| {
            let first_error_ts = input
                .logs
                .iter()
                .filter(|log| {
                    log.service == deployment.service
                        && matches!(log.level.to_ascii_lowercase().as_str(), "error" | "fatal")
                })
                .map(|log| log.ts.clone())
                .min();

            let correlation = match &first_error_ts {
                Some(first_error_ts) if first_error_ts >= &deployment.ts => "after_deploy",
                Some(_) => "before_deploy",
                None => "no_direct_error",
            }
            .to_string();

            let reason = match correlation.as_str() {
                "after_deploy" => "first service error appears at or after the deployment timestamp",
                "before_deploy" => "service already had errors before this deployment timestamp",
                _ => "service is affected, but no direct error log was found for this deployment",
            }
            .to_string();

            DeploymentCorrelation {
                service: deployment.service.clone(),
                version: deployment.version.clone(),
                commit: deployment.commit.clone(),
                deployment_ts: deployment.ts.clone(),
                first_error_ts,
                correlation,
                reason,
            }
        })
        .collect()
}

fn collect_evidence_signals(input: &IncidentInput) -> Vec<EvidenceSignal> {
    let mut signals = Vec::new();

    for alert in &input.alerts {
        signals.push(EvidenceSignal {
            signal_type: "alert".to_string(),
            service: Some(alert.service.clone()),
            severity: alert.severity.clone(),
            summary: alert.message.clone(),
        });
    }

    for log in input.logs.iter().filter(|log| {
        matches!(
            log.level.to_ascii_lowercase().as_str(),
            "error" | "fatal" | "warn" | "warning"
        )
    }) {
        signals.push(EvidenceSignal {
            signal_type: "log".to_string(),
            service: Some(log.service.clone()),
            severity: log.level.clone(),
            summary: log.message.clone(),
        });
    }

    signals.truncate(12);
    signals
}

fn ordered_affected_services(
    by_service_errors: BTreeMap<String, usize>,
    affected: BTreeSet<String>,
) -> Vec<String> {
    let mut ranked = by_service_errors.into_iter().collect::<Vec<_>>();
    ranked.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));

    let mut out = Vec::new();
    for (service, _) in ranked {
        if !out.contains(&service) {
            out.push(service);
        }
    }
    for service in affected {
        if !out.contains(&service) {
            out.push(service);
        }
    }
    out
}

fn match_runbooks(input: &IncidentInput) -> Vec<RunbookMatch> {
    let mut matches = Vec::new();
    let corpus = input
        .logs
        .iter()
        .map(|log| log.message.to_ascii_lowercase())
        .chain(input.alerts.iter().map(|alert| alert.message.to_ascii_lowercase()))
        .collect::<Vec<_>>()
        .join("\n");

    if corpus.contains("timeout") || corpus.contains("timed out") {
        matches.push(RunbookMatch {
            id: "rb-upstream-timeout".to_string(),
            title: "Upstream timeout triage".to_string(),
            reason: "timeout-related errors were found in incident evidence".to_string(),
        });
    }
    if corpus.contains("connection pool") || corpus.contains("pool saturation") {
        matches.push(RunbookMatch {
            id: "rb-pool-saturation".to_string(),
            title: "Connection pool saturation".to_string(),
            reason: "connection pool saturation indicators were found".to_string(),
        });
    }
    if corpus.contains("database") || corpus.contains("db ") {
        matches.push(RunbookMatch {
            id: "rb-database-dependency".to_string(),
            title: "Database dependency degradation".to_string(),
            reason: "database-related errors were found".to_string(),
        });
    }
    if corpus.contains("5xx") || corpus.contains("502") || corpus.contains("503") {
        matches.push(RunbookMatch {
            id: "rb-http-5xx-spike".to_string(),
            title: "HTTP 5xx spike response".to_string(),
            reason: "HTTP 5xx symptoms were found".to_string(),
        });
    }

    matches
}

fn confidence_score(
    input: &IncidentInput,
    error_logs: usize,
    suspected_deployments: usize,
    deployment_correlations: usize,
    runbook_matches: usize,
) -> u8 {
    let mut score = 20u16;
    score += input.alerts.len().min(3) as u16 * 10;
    score += error_logs.min(5) as u16 * 8;
    score += suspected_deployments.min(2) as u16 * 10;
    score += deployment_correlations.min(2) as u16 * 8;
    score += runbook_matches.min(3) as u16 * 6;
    score.min(100) as u8
}

fn rollback_advice(risk_score: u8, correlations: &[DeploymentCorrelation]) -> RollbackAdvice {
    let targets = correlations
        .iter()
        .filter(|correlation| correlation.correlation == "after_deploy")
        .map(|correlation| correlation.service.clone())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();

    let should_prepare_rollback = risk_score >= 70 && !targets.is_empty();
    let reason = if should_prepare_rollback {
        "high risk score with post-deployment error correlation".to_string()
    } else if targets.is_empty() {
        "no clear post-deployment error correlation was found".to_string()
    } else {
        "risk score is below rollback preparation threshold".to_string()
    };

    RollbackAdvice {
        should_prepare_rollback,
        target_services: targets,
        reason,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{AlertEvent, DeploymentEvent, LogEvent};

    #[test]
    fn detects_high_risk_deployment_related_incident() {
        let input = IncidentInput {
            title: "checkout 5xx spike".to_string(),
            alerts: vec![AlertEvent {
                service: "checkout-api".to_string(),
                severity: "page".to_string(),
                message: "5xx above threshold".to_string(),
                ts: "2026-05-07T10:10:00Z".to_string(),
            }],
            deployments: vec![DeploymentEvent {
                service: "checkout-api".to_string(),
                version: "2026.05.07".to_string(),
                actor: "deploy-bot".to_string(),
                ts: "2026-05-07T10:00:00Z".to_string(),
                commit: Some("abc123".to_string()),
            }],
            logs: vec![LogEvent {
                service: "checkout-api".to_string(),
                level: "error".to_string(),
                message: "upstream timeout".to_string(),
                ts: "2026-05-07T10:11:00Z".to_string(),
                trace_id: Some("trc-1".to_string()),
            }],
        };

        let detection = detect(&input);

        assert_eq!(detection.error_logs, 1);
        assert_eq!(detection.suspected_deployments.len(), 1);
        assert_eq!(detection.service_impacts[0].service, "checkout-api");
        assert!(detection.risk_score >= 50);
        assert!(detection.confidence_score >= 50);
        assert!(matches!(
            detection.severity,
            IncidentSeverity::High | IncidentSeverity::Critical
        ));
        assert_eq!(detection.timeline.len(), 3);
        assert!(detection
            .runbook_matches
            .iter()
            .any(|runbook| runbook.id == "rb-upstream-timeout"));
        assert!(detection
            .deployment_correlations
            .iter()
            .any(|correlation| correlation.correlation == "after_deploy"));
    }

    #[test]
    fn validation_rejects_empty_incident() {
        let input = IncidentInput {
            title: " ".to_string(),
            alerts: Vec::new(),
            deployments: Vec::new(),
            logs: Vec::new(),
        };

        assert!(input.validate().is_err());
    }

    #[test]
    fn validation_rejects_blank_event_fields() {
        let input = IncidentInput {
            title: "bad payload".to_string(),
            alerts: Vec::new(),
            deployments: Vec::new(),
            logs: vec![LogEvent {
                service: "checkout-api".to_string(),
                level: "error".to_string(),
                message: " ".to_string(),
                ts: "2026-05-07T10:11:00Z".to_string(),
                trace_id: None,
            }],
        };

        let error = input.validate().expect_err("blank log message should fail");
        assert!(error.contains("logs[0].message"));
    }
}
