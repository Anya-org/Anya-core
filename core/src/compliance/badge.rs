//! Automated compliance badge generation
use super::BipComplianceReport;
use urlencoding::encode;

const BADGE_TEMPLATE: &str =
    "https://img.shields.io/badge/BDF-%.2f%%25-4BC51D?logo=bitcoin&style=for-the-badge";

pub fn generate_badge(format: &str) -> anyhow::Result<String> {
    let report = BipComplianceReport::generate()?;
    let score = calculate_compliance_score(&report);

    let encoded_score = encode(&format!("{:.2}%", score * 100.0));
    let badge_url = BADGE_TEMPLATE.replace("%.2f%%25", &encoded_score);

    Ok(match format.to_lowercase().as_str() {
        "svg" => badge_url,
        "md" => format!(
            "[![BDF Compliance]({})](https://compliance.anya.btc)",
            badge_url
        ),
        "html" => format!(r#"<img src="{}" alt="BDF Compliance">"#, badge_url),
        _ => anyhow::bail!("Unsupported badge format"),
    })
}

fn calculate_compliance_score(report: &BipComplianceReport) -> f32 {
    let weights = vec![
        (report.bip340.implemented as u32, 0.3),
        (report.bip341.implemented as u32, 0.4),
        (report.bip342.implemented as u32, 0.3),
    ];

    weights
        .iter()
        .map(|(status, weight)| *status as f32 * weight)
        .sum::<f32>()
        / weights.iter().map(|(_, w)| w).sum::<f32>()
}
