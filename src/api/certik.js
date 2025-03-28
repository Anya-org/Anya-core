const express = require('express');
const router = express.Router();
const db = require('../db/postgres');

// Get current metrics data
router.get('/metrics', async (req, res) => {
  try {
    // Get findings summary
    const summary = await db.query(`
      SELECT 
        COUNT(*) FILTER (WHERE severity = 'Critical' AND status = 'open') as critical,
        COUNT(*) FILTER (WHERE severity = 'High' AND status = 'open') as high,
        COUNT(*) FILTER (WHERE severity = 'Medium' AND status = 'open') as medium,
        COUNT(*) FILTER (WHERE severity = 'Low' AND status = 'open') as low,
        COUNT(*) FILTER (WHERE status = 'fixed') as fixed,
        COUNT(*) FILTER (WHERE status = 'open') as pending
      FROM certik_findings
    `);
    
    // Get component data
    const components = await db.query(`
      SELECT 
        component, 
        COUNT(*) as issues,
        COUNT(*) FILTER (WHERE status = 'fixed') as fixed
      FROM certik_findings
      GROUP BY component
    `);
    
    // Get BIP data
    const bips = await db.query(`
      SELECT 
        bip,
        COUNT(*) as issues,
        COUNT(*) FILTER (WHERE status = 'fixed') as fixed
      FROM certik_findings
      WHERE bip IS NOT NULL
      GROUP BY bip
    `);
    
    // Format the response
    const metrics = {
      summary: summary.rows[0],
      components: {},
      bips: {},
      updated_at: new Date().toISOString()
    };
    
    // Format components data
    components.rows.forEach(row => {
      metrics.components[row.component] = {
        issues: parseInt(row.issues),
        fixed: parseInt(row.fixed)
      };
    });
    
    // Format BIPs data
    bips.rows.forEach(row => {
      metrics.bips[row.bip] = {
        issues: parseInt(row.issues),
        fixed: parseInt(row.fixed)
      };
    });
    
    res.json(metrics);
  } catch (error) {
    console.error('Error fetching metrics:', error);
    res.status(500).json({ error: 'Failed to fetch metrics' });
  }
});

// Get historical scores
router.get('/history', async (req, res) => {
  try {
    const history = await db.query(`
      SELECT 
        date, 
        security_score, 
        compliance_score,
        pending_issues,
        fixed_issues
      FROM certik_audit_history
      ORDER BY date ASC
      LIMIT 30
    `);
    
    res.json(history.rows.map(row => ({
      date: row.date.toISOString().split('T')[0],
      securityScore: row.security_score,
      complianceScore: row.compliance_score,
      pendingIssues: row.pending_issues,
      fixedIssues: row.fixed_issues
    })));
  } catch (error) {
    console.error('Error fetching history:', error);
    res.status(500).json({ error: 'Failed to fetch history' });
  }
});

module.exports = router; 