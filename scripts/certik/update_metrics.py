#!/usr/bin/env python3
import json
import argparse
import os
import time
from datetime import datetime

METRICS_FILE = "reports/certik/metrics.json"

def load_metrics():
    if not os.path.exists(METRICS_FILE):
        return {
            "issues": {},
            "components": {},
            "bips": {},
            "summary": {
                "critical": 0,
                "high": 0,
                "medium": 0,
                "low": 0,
                "fixed": 0,
                "pending": 0
            },
            "updated_at": datetime.now().isoformat()
        }
    
    with open(METRICS_FILE, 'r') as f:
        return json.load(f)

def update_metrics(issue_id, severity, component, bip, status):
    metrics = load_metrics()
    
    # Update issue tracking
    if status == "closed":
        if issue_id in metrics["issues"]:
            old_severity = metrics["issues"][issue_id]["severity"].lower()
            metrics["summary"][old_severity] -= 1
            metrics["summary"]["fixed"] += 1
            metrics["summary"]["pending"] -= 1
            metrics["issues"][issue_id]["fixed_at"] = datetime.now().isoformat()
            metrics["issues"][issue_id]["status"] = "fixed"
    else:
        if issue_id not in metrics["issues"]:
            metrics["issues"][issue_id] = {
                "severity": severity.lower(),
                "component": component,
                "bip": bip,
                "created_at": datetime.now().isoformat(),
                "status": "pending"
            }
            metrics["summary"][severity.lower()] += 1
            metrics["summary"]["pending"] += 1
        
    # Update component metrics
    if component:
        if component not in metrics["components"]:
            metrics["components"][component] = {"issues": 0, "fixed": 0}
        if status == "closed":
            metrics["components"][component]["fixed"] += 1
        else:
            metrics["components"][component]["issues"] += 1
    
    # Update BIP metrics
    if bip:
        if bip not in metrics["bips"]:
            metrics["bips"][bip] = {"issues": 0, "fixed": 0}
        if status == "closed":
            metrics["bips"][bip]["fixed"] += 1
        else:
            metrics["bips"][bip]["issues"] += 1
    
    metrics["updated_at"] = datetime.now().isoformat()
    
    # Save updated metrics
    os.makedirs(os.path.dirname(METRICS_FILE), exist_ok=True)
    with open(METRICS_FILE, 'w') as f:
        json.dump(metrics, f, indent=2)

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description='Update CertiK audit metrics')
    parser.add_argument('--issue', required=True, help='Issue ID')
    parser.add_argument('--severity', choices=['Critical', 'High', 'Medium', 'Low'], help='Severity')
    parser.add_argument('--component', help='Affected component')
    parser.add_argument('--bip', help='BIP number')
    parser.add_argument('--status', choices=['open', 'closed'], help='Issue status')
    
    args = parser.parse_args()
    update_metrics(args.issue, args.severity, args.component, args.bip, args.status)
    print(f"Updated metrics for issue #{args.issue}")
