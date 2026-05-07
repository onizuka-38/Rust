use crate::model::DetectionSummary;

pub fn render_markdown(
    title: &str,
    detection: &DetectionSummary,
    ai_summary: &str,
    actions: &[String],
) -> String {
    let mut out = String::new();
    out.push_str(&format!("# Incident Report: {title}\n\n"));
    out.push_str("## Detection Summary\n\n");
    out.push_str(&format!("- Severity: {:?}\n", detection.severity));
    out.push_str(&format!("- Risk score: {}/100\n", detection.risk_score));
    out.push_str(&format!("- Confidence score: {}/100\n", detection.confidence_score));
    out.push_str(&format!("- Total logs: {}\n", detection.total_logs));
    out.push_str(&format!("- Error logs: {}\n", detection.error_logs));
    out.push_str(&format!("- Warning logs: {}\n", detection.warn_logs));
    out.push_str(&format!(
        "- Affected services: {}\n\n",
        empty_dash(&detection.affected_services.join(", "))
    ));

    out.push_str("## Service Impact\n\n");
    if detection.service_impacts.is_empty() {
        out.push_str("- None\n\n");
    } else {
        for impact in &detection.service_impacts {
            out.push_str(&format!(
                "- `{}` errors={} warnings={} alerts={} traces={}\n",
                impact.service,
                impact.error_logs,
                impact.warn_logs,
                impact.alert_count,
                empty_dash(&impact.sample_trace_ids.join(", "))
            ));
        }
        out.push('\n');
    }

    out.push_str("## Suspected Deployments\n\n");
    if detection.suspected_deployments.is_empty() {
        out.push_str("- None\n\n");
    } else {
        for deploy in &detection.suspected_deployments {
            out.push_str(&format!(
                "- `{}` version `{}` by `{}` at `{}`\n",
                deploy.service, deploy.version, deploy.actor, deploy.ts
            ));
        }
        out.push('\n');
    }

    out.push_str("## Deployment Correlations\n\n");
    if detection.deployment_correlations.is_empty() {
        out.push_str("- None\n\n");
    } else {
        for correlation in &detection.deployment_correlations {
            let first_error = correlation.first_error_ts.as_deref().map_or("-", |ts| ts);
            out.push_str(&format!(
                "- `{}` version `{}` correlation=`{}` first_error=`{}` reason={}\n",
                correlation.service,
                correlation.version,
                correlation.correlation,
                first_error,
                correlation.reason
            ));
        }
        out.push('\n');
    }

    out.push_str("## Rollback Advice\n\n");
    out.push_str(&format!(
        "- Prepare rollback: {}\n",
        detection.rollback_advice.should_prepare_rollback
    ));
    out.push_str(&format!(
        "- Target services: {}\n",
        empty_dash(&detection.rollback_advice.target_services.join(", "))
    ));
    out.push_str(&format!("- Reason: {}\n\n", detection.rollback_advice.reason));

    out.push_str("## Timeline\n\n");
    if detection.timeline.is_empty() {
        out.push_str("- None\n\n");
    } else {
        for event in detection.timeline.iter().take(12) {
            out.push_str(&format!(
                "- `{}` [{}] `{}` - {}\n",
                event.ts, event.kind, event.service, event.summary
            ));
        }
        out.push('\n');
    }

    out.push_str("## Matched Runbooks\n\n");
    if detection.runbook_matches.is_empty() {
        out.push_str("- None\n\n");
    } else {
        for runbook in &detection.runbook_matches {
            out.push_str(&format!(
                "- `{}` - {} ({})\n",
                runbook.id, runbook.title, runbook.reason
            ));
        }
        out.push('\n');
    }

    out.push_str("## AI Analysis\n\n");
    out.push_str(ai_summary.trim());
    out.push_str("\n\n## Recommended Actions\n\n");
    for action in actions {
        out.push_str(&format!("- {action}\n"));
    }

    out
}

fn empty_dash(value: &str) -> &str {
    if value.is_empty() {
        "-"
    } else {
        value
    }
}
