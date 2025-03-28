#!/usr/bin/env python3
import json
import argparse
import os
from datetime import datetime, timedelta

METRICS_FILE = "reports/certik/metrics.json"

def load_metrics():
    if not os.path.exists(METRICS_FILE):
        return {}
    
    with open(METRICS_FILE, 'r') as f:
        return json.load(f)

def generate_report(output_file):
    metrics = load_metrics()
    
    if not metrics:
        print("No metrics data found")
        return
    
    # Calculate remediation times
    remediation_times = []
    for issue_id, details in metrics["issues"].items():
        if details["status"] == "fixed" and "fixed_at" in details:
            created = datetime.fromisoformat(details["created_at"])
            fixed = datetime.fromisoformat(details["fixed_at"])
            days = (fixed - created).days
            remediation_times.append({
                "id": issue_id, 
                "days": days,
                "severity": details["severity"]
            })
    
    # Calculate average remediation time
    avg_time = 0
    if remediation_times:
        avg_time = sum(item["days"] for item in remediation_times) / len(remediation_times)
    
    # Generate markdown report
    report = f"""# CertiK Audit Remediation Report
    
Generated: {datetime.now().strftime("%Y-%m-%d %H:%M:%S")}

## Summary
- Critical Issues: {metrics["summary"]["critical"]}
- High Issues: {metrics["summary"]["high"]}
- Medium Issues: {metrics["summary"]["medium"]}
- Low Issues: {metrics["summary"]["low"]}
- Total Fixed: {metrics["summary"]["fixed"]}
- Remaining: {metrics["summary"]["pending"]}
- Average Remediation Time: {avg_time:.1f} days

## Component Analysis
| Component | Issues | Fixed | Compliance % |
|-----------|--------|-------|-------------|
"""
    
    for component, data in metrics["components"].items():
        compliance = 100.0
        if data["issues"] > 0:
            compliance = (data["fixed"] / data["issues"]) * 100
        report += f"| {component} | {data['issues']} | {data['fixed']} | {compliance:.1f}% |\n"
    
    report += """
## BIP Compliance
| BIP | Issues | Fixed | Compliance % |
|-----|--------|-------|-------------|
"""
    
    for bip, data in metrics["bips"].items():
        compliance = 100.0
        if data["issues"] > 0:
            compliance = (data["fixed"] / data["issues"]) * 100
        report += f"| BIP-{bip} | {data['issues']} | {data['fixed']} | {compliance:.1f}% |\n"
    
    report += """
## Remediation Timeline
| Issue | Severity | Days to Fix |
|-------|----------|-------------|
"""
    
    for item in sorted(remediation_times, key=lambda x: int(x["days"])):
        report += f"| #{item['id']} | {item['severity'].capitalize()} | {item['days']} |\n"
    
    # Save report
    os.makedirs(os.path.dirname(output_file), exist_ok=True)
    with open(output_file, 'w') as f:
        f.write(report)
    
    print(f"Report generated: {output_file}")

def main():
    parser = argparse.ArgumentParser(description='Generate CertiK Audit Report')
    parser.add_argument('--output', required=True, help='Output file path')
    parser.add_argument('--metrics', default='metrics/certik.json', 
                       help='Path to metrics JSON file')
    args = parser.parse_args()
    
    generate_report(args.output)

if __name__ == "__main__":
    main()
