#!/usr/bin/env python3
import unittest
import os
import json
import tempfile
import shutil
import sys
from datetime import datetime, timedelta

# Add scripts directory to path
sys.path.append(os.path.join(os.path.dirname(__file__), '../../scripts/certik'))

import update_metrics
import generate_report

class TestCertiKRemediation(unittest.TestCase):
    def setUp(self):
        # Create a temporary directory for test files
        self.test_dir = tempfile.mkdtemp()
        # Set the metrics file path to use test directory
        update_metrics.METRICS_FILE = os.path.join(self.test_dir, "metrics.json")
        generate_report.METRICS_FILE = update_metrics.METRICS_FILE
        
    def tearDown(self):
        # Remove the test directory
        shutil.rmtree(self.test_dir)
    
    def test_add_issue(self):
        """Test adding a new issue to the tracker"""
        update_metrics.update_metrics(
            issue_id="123", 
            severity="Critical", 
            component="PSBT Manager", 
            bip="370", 
            status="open"
        )
        
        # Verify metrics file was created
        self.assertTrue(os.path.exists(update_metrics.METRICS_FILE))
        
        # Load metrics and verify data
        with open(update_metrics.METRICS_FILE, 'r') as f:
            metrics = json.load(f)
        
        self.assertEqual(metrics["summary"]["critical"], 1)
        self.assertEqual(metrics["summary"]["pending"], 1)
        self.assertEqual(metrics["components"]["PSBT Manager"]["issues"], 1)
        self.assertEqual(metrics["bips"]["370"]["issues"], 1)
    
    def test_fix_issue(self):
        """Test marking an issue as fixed"""
        # First add the issue
        update_metrics.update_metrics(
            issue_id="123", 
            severity="Critical", 
            component="PSBT Manager", 
            bip="370", 
            status="open"
        )
        
        # Now mark it as fixed
        update_metrics.update_metrics(
            issue_id="123", 
            severity="Critical", 
            component="PSBT Manager", 
            bip="370", 
            status="closed"
        )
        
        # Load metrics and verify data
        with open(update_metrics.METRICS_FILE, 'r') as f:
            metrics = json.load(f)
        
        self.assertEqual(metrics["summary"]["critical"], 0)
        self.assertEqual(metrics["summary"]["fixed"], 1)
        self.assertEqual(metrics["issues"]["123"]["status"], "fixed")
        self.assertTrue("fixed_at" in metrics["issues"]["123"])
        self.assertEqual(metrics["components"]["PSBT Manager"]["fixed"], 1)
        self.assertEqual(metrics["bips"]["370"]["fixed"], 1)
    
    def test_generate_report(self):
        """Test report generation"""
        # Add several issues with varying severities
        update_metrics.update_metrics("101", "Critical", "HSM Interface", "341", "open")
        update_metrics.update_metrics("102", "High", "Transaction Signer", "341", "open")
        update_metrics.update_metrics("103", "Medium", "PSBT Manager", "370", "open")
        
        # Fix one issue
        # Manually modify the creation date to test remediation time calculation
        with open(update_metrics.METRICS_FILE, 'r') as f:
            metrics = json.load(f)
        
        # Set issue 101 created date to 7 days ago
        seven_days_ago = (datetime.now() - timedelta(days=7)).isoformat()
        metrics["issues"]["101"]["created_at"] = seven_days_ago
        
        with open(update_metrics.METRICS_FILE, 'w') as f:
            json.dump(metrics, f)
        
        # Now mark issue 101 as fixed
        update_metrics.update_metrics("101", "Critical", "HSM Interface", "341", "closed")
        
        # Generate report
        report_file = os.path.join(self.test_dir, "report.md")
        generate_report.generate_report(report_file)
        
        # Verify report was created
        self.assertTrue(os.path.exists(report_file))
        
        # Check report content
        with open(report_file, 'r') as f:
            report_content = f.read()
        
        self.assertIn("Critical Issues: 0", report_content)
        self.assertIn("High Issues: 1", report_content)
        self.assertIn("Medium Issues: 1", report_content)
        self.assertIn("Total Fixed: 1", report_content)
        self.assertIn("Remaining: 2", report_content)
        
        # Check component and BIP tables
        self.assertIn("HSM Interface", report_content)
        self.assertIn("BIP-341", report_content)
        self.assertIn("BIP-370", report_content)
        
        # Check remediation timeline
        self.assertIn("#101", report_content)
        self.assertIn("7", report_content)  # 7 days to fix

if __name__ == "__main__":
    unittest.main()
