#!/usr/bin/env python3
"""
Bitcoin Core Security Conformance Test Script
This script automates running the comprehensive security test suite
to verify full alignment with Bitcoin Core security principles.
"""

import os
import sys
import subprocess
import json
import time
from datetime import datetime
from pathlib import Path

# Configuration
TEST_LOG_DIR = "logs/security_tests"
HTML_REPORT = "security_report.html"
SECURITY_SCORE_FILE = "security_score.json"

# Define test categories
TEST_CATEGORIES = {
    "consensus_bugs": [
        "test_cve_2010_5139_value_overflow",
        "test_cve_2018_17144_duplicate_inputs",
        "test_historical_consensus_bugs"
    ],
    "consensus_invariants": [
        "test_consensus_invariant_checker", 
        "test_differential_fuzzing"
    ],
    "side_channel": [
        "test_timing_side_channels"
    ],
    "hardware_optimizations": [
        "test_hardware_optimizations_consensus"
    ]
}

def ensure_directory(dir_path):
    """Ensure the directory exists"""
    Path(dir_path).mkdir(parents=True, exist_ok=True)

def run_single_test(test_name):
    """Run a single security test by name and return result"""
    print(f"Running test: {test_name}")
    
    # Format the test path correctly
    test_path = f"tests::bitcoin::principles::{test_name}"
    
    # Run the test with detailed output
    cmd = ["cargo", "test", test_path, "--", "--nocapture"]
    
    try:
        result = subprocess.run(
            cmd,
            capture_output=True,
            text=True,
            check=False
        )
        
        success = result.returncode == 0
        
        # Extract timing information if available
        duration_ms = None
        for line in result.stdout.splitlines():
            if "test result:" in line and "elapsed:" in line:
                try:
                    time_part = line.split("elapsed:")[1].strip()
                    if "ms" in time_part:
                        duration_ms = float(time_part.split("ms")[0].strip())
                except Exception:
                    pass
        
        return {
            "name": test_name,
            "success": success,
            "output": result.stdout,
            "error": result.stderr,
            "duration_ms": duration_ms
        }
    except Exception as e:
        return {
            "name": test_name,
            "success": False,
            "output": "",
            "error": str(e),
            "duration_ms": None
        }

def run_all_tests():
    """Run all security tests and return results"""
    all_results = []
    
    # Flatten test categories into a single list
    all_tests = []
    for category, tests in TEST_CATEGORIES.items():
        for test in tests:
            all_tests.append((category, test))
    
    # Run each test
    for category, test in all_tests:
        result = run_single_test(test)
        result["category"] = category
        all_results.append(result)
        
        # Print immediate feedback
        status = "✅ PASSED" if result["success"] else "❌ FAILED"
        duration = f" ({result['duration_ms']:.1f}ms)" if result['duration_ms'] else ""
        print(f"{status} {test}{duration}")
    
    return all_results

def calculate_security_score(results):
    """Calculate security score based on test results"""
    # Each category has equal weight
    category_scores = {}
    
    # Initialize each category score
    for category in TEST_CATEGORIES:
        category_scores[category] = {
            "total": 0,
            "passed": 0,
            "score": 0.0
        }
    
    # Count tests by category
    for result in results:
        category = result["category"]
        category_scores[category]["total"] += 1
        if result["success"]:
            category_scores[category]["passed"] += 1
    
    # Calculate category scores
    for category, data in category_scores.items():
        if data["total"] > 0:
            data["score"] = (data["passed"] / data["total"]) * 100.0
        else:
            data["score"] = 0.0
    
    # Calculate overall score (equal weight for each category)
    total_score = sum(data["score"] for data in category_scores.values()) / len(category_scores)
    
    # Scale to 0-5 for Security principle score
    security_principle_score = total_score / 20.0
    
    return {
        "timestamp": datetime.now().isoformat(),
        "overall_score": total_score,
        "security_principle_score": security_principle_score,
        "categories": category_scores
    }

def generate_html_report(results, score_data):
    """Generate HTML report from test results"""
    # Create report directory
    ensure_directory(os.path.dirname(HTML_REPORT))
    
    html = """
    <!DOCTYPE html>
    <html>
    <head>
        <meta charset="utf-8">
        <meta name="viewport" content="width=device-width, initial-scale=1">
        <title>Bitcoin Core Security Conformance Report</title>
        <style>
            body { font-family: Arial, sans-serif; line-height: 1.6; color: #333; max-width: 1200px; margin: 0 auto; padding: 20px; }
            h1, h2, h3 { color: #0D1F2D; }
            .header { text-align: center; margin-bottom: 30px; }
            .summary { background-color: #f8f9fa; border-radius: 5px; padding: 20px; margin-bottom: 30px; }
            .score-card { display: flex; justify-content: space-between; flex-wrap: wrap; margin-bottom: 20px; }
            .score-box { flex: 1; min-width: 200px; text-align: center; background: #fff; margin: 10px; padding: 15px; border-radius: 5px; box-shadow: 0 1px 3px rgba(0,0,0,0.1); }
            .score { font-size: 24px; font-weight: bold; margin: 10px 0; }
            .excellent { color: #28a745; }
            .good { color: #17a2b8; }
            .fair { color: #ffc107; }
            .poor { color: #dc3545; }
            .test-results { margin-top: 20px; }
            .test-category { margin-bottom: 20px; }
            .test-item { background: #fff; padding: 15px; margin-bottom: 10px; border-radius: 5px; box-shadow: 0 1px 3px rgba(0,0,0,0.1); }
            .test-item.passed { border-left: 4px solid #28a745; }
            .test-item.failed { border-left: 4px solid #dc3545; }
            .test-name { font-weight: bold; display: flex; justify-content: space-between; }
            .output { background: #f8f9fa; padding: 10px; border-radius: 5px; white-space: pre-wrap; margin-top: 10px; font-family: monospace; font-size: 14px; max-height: 200px; overflow-y: auto; }
            .error { background: #fff3f3; color: #dc3545; }
            .toggle-btn { cursor: pointer; color: #007bff; font-size: 12px; margin-left: 10px; }
            .hidden { display: none; }
        </style>
    </head>
    <body>
        <div class="header">
            <h1>Bitcoin Core Security Conformance Report</h1>
            <p>Generated on """ + datetime.now().strftime("%Y-%m-%d %H:%M:%S") + """</p>
        </div>
        
        <div class="summary">
            <h2>Security Score Summary</h2>
            <div class="score-card">
    """
    
    # Overall score
    score_class = ""
    if score_data["overall_score"] >= 90:
        score_class = "excellent"
    elif score_data["overall_score"] >= 75:
        score_class = "good"
    elif score_data["overall_score"] >= 50:
        score_class = "fair"
    else:
        score_class = "poor"
    
    html += f"""
        <div class="score-box">
            <h3>Overall Score</h3>
            <div class="score {score_class}">{score_data['overall_score']:.1f}%</div>
        </div>
        
        <div class="score-box">
            <h3>Security Principle Score</h3>
            <div class="score {score_class}">{score_data['security_principle_score']:.1f}/5.0</div>
        </div>
    """
    
    # Category scores
    for category, data in score_data["categories"].items():
        score_class = ""
        if data["score"] >= 90:
            score_class = "excellent"
        elif data["score"] >= 75:
            score_class = "good"
        elif data["score"] >= 50:
            score_class = "fair"
        else:
            score_class = "poor"
        
        category_name = category.replace("_", " ").title()
        html += f"""
            <div class="score-box">
                <h3>{category_name}</h3>
                <div class="score {score_class}">{data['score']:.1f}%</div>
                <div>{data['passed']} / {data['total']} tests passed</div>
            </div>
        """
    
    html += """
            </div>
        </div>
        
        <div class="test-results">
            <h2>Detailed Test Results</h2>
    """
    
    # Group results by category
    results_by_category = {}
    for result in results:
        category = result["category"]
        if category not in results_by_category:
            results_by_category[category] = []
        results_by_category[category].append(result)
    
    # Generate test results by category
    for category, category_results in results_by_category.items():
        category_name = category.replace("_", " ").title()
        html += f"""
            <div class="test-category">
                <h3>{category_name} Tests</h3>
        """
        
        for result in category_results:
            status_class = "passed" if result["success"] else "failed"
            status_text = "PASSED" if result["success"] else "FAILED"
            duration = f"{result['duration_ms']:.1f}ms" if result['duration_ms'] else "unknown"
            
            html += f"""
                <div class="test-item {status_class}">
                    <div class="test-name">
                        <span>{result['name']} - {status_text}</span>
                        <span>{duration} <span class="toggle-btn" onclick="toggleOutput(this)">Show Details</span></span>
                    </div>
                    <div class="output hidden">{result['output']}</div>
            """
            
            if result["error"]:
                html += f"""
                    <div class="output error hidden">{result['error']}</div>
                """
            
            html += """
                </div>
            """
        
        html += """
            </div>
        """
    
    html += """
        </div>
        
        <script>
            function toggleOutput(btn) {
                const output = btn.parentElement.parentElement.querySelector('.output');
                const error = btn.parentElement.parentElement.querySelector('.error');
                
                if (output.classList.contains('hidden')) {
                    output.classList.remove('hidden');
                    if (error) error.classList.remove('hidden');
                    btn.textContent = 'Hide Details';
                } else {
                    output.classList.add('hidden');
                    if (error) error.classList.add('hidden');
                    btn.textContent = 'Show Details';
                }
            }
        </script>
    </body>
    </html>
    """
    
    with open(HTML_REPORT, "w") as f:
        f.write(html)
    
    print(f"HTML report generated: {os.path.abspath(HTML_REPORT)}")

def save_score_data(score_data):
    """Save score data to JSON file"""
    ensure_directory(os.path.dirname(SECURITY_SCORE_FILE))
    
    with open(SECURITY_SCORE_FILE, "w") as f:
        json.dump(score_data, f, indent=2)
    
    print(f"Security score saved to: {os.path.abspath(SECURITY_SCORE_FILE)}")

def main():
    """Main entry point"""
    print("🔒 Running Bitcoin Core Security Conformance Tests")
    print("=====================================================")
    
    # Ensure log directory exists
    ensure_directory(TEST_LOG_DIR)
    
    # Log file for this run
    timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
    log_file = os.path.join(TEST_LOG_DIR, f"security_test_{timestamp}.log")
    
    # Run all tests
    start_time = time.time()
    results = run_all_tests()
    elapsed = time.time() - start_time
    
    # Calculate security score
    score_data = calculate_security_score(results)
    
    # Print summary
    successful = sum(1 for r in results if r["success"])
    print("\n=====================================================")
    print(f"✅ {successful}/{len(results)} tests passed")
    print(f"⏱️ Total time: {elapsed:.2f}s")
    print(f"🔐 Security principle score: {score_data['security_principle_score']:.1f}/5.0")
    print(f"📊 Overall security score: {score_data['overall_score']:.1f}%")
    
    # Save detailed log
    with open(log_file, "w") as f:
        f.write(f"Bitcoin Core Security Conformance Test - {timestamp}\n")
        f.write("=====================================================\n\n")
        
        for result in results:
            status = "PASSED" if result["success"] else "FAILED"
            f.write(f"{status} - {result['name']}\n")
            f.write(f"Category: {result['category']}\n")
            f.write(f"Duration: {result['duration_ms']}ms\n\n")
            f.write("OUTPUT:\n")
            f.write(result["output"])
            f.write("\n\n")
            if result["error"]:
                f.write("ERROR:\n")
                f.write(result["error"])
                f.write("\n\n")
            f.write("-----------------------------------------------------\n\n")
        
        f.write(f"Security Score: {score_data['security_principle_score']:.1f}/5.0\n")
        f.write(f"Overall Score: {score_data['overall_score']:.1f}%\n")
    
    print(f"📝 Log saved to: {os.path.abspath(log_file)}")
    
    # Generate HTML report
    generate_html_report(results, score_data)
    
    # Save score data
    save_score_data(score_data)
    
    # Update system integration score
    update_system_integration()
    
    return 0 if successful == len(results) else 1

def update_system_integration():
    """Update the system integration script with the new security score"""
    try:
        # This is optional - update the system integration script
        integration_script = "scripts/integration/hardware_system_integration.py"
        
        if os.path.exists(integration_script):
            print("Updating system integration score...")
            
            # Run the system integration script to verify full alignment
            subprocess.run(["python", integration_script], check=False)
            
            print("System integration updated.")
    except Exception as e:
        print(f"Warning: Could not update system integration: {e}")

if __name__ == "__main__":
    sys.exit(main())
