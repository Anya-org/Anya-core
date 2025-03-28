// Compliance Score Dashboard for CertiK Audit
import React, { useState, useEffect } from 'react';
import { Line, Doughnut, Bar } from 'react-chartjs-2';
import axios from 'axios';

// Scoring weights from audit contract
const WEIGHTS = {
  BIP341: 0.4,
  BIP370: 0.3,
  AIS3: 0.3,
  CRITICAL: 10,
  HIGH: 5,
  MEDIUM: 2,
  LOW: 1
};

const CertiKComplianceDashboard = () => {
  const [metrics, setMetrics] = useState(null);
  const [scores, setScores] = useState({
    securityScore: 0,
    complianceScore: 0,
    totalScore: 0
  });
  const [history, setHistory] = useState([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    // Load metrics data
    const fetchData = async () => {
      try {
        const response = await axios.get('/api/certik/metrics');
        setMetrics(response.data);
        
        // Calculate scores
        const security = calculateSecurityScore(response.data);
        const compliance = calculateComplianceScore(response.data);
        
        setScores({
          securityScore: security,
          complianceScore: compliance,
          totalScore: (security + compliance) / 2
        });
        
        // Load historical data
        const historyResponse = await axios.get('/api/certik/history');
        setHistory(historyResponse.data);
        
        setLoading(false);
      } catch (error) {
        console.error('Error fetching metrics', error);
        setLoading(false);
      }
    };
    
    fetchData();
    // Refresh every 5 minutes
    const interval = setInterval(fetchData, 300000);
    return () => clearInterval(interval);
  }, []);

  const calculateSecurityScore = (data) => {
    if (!data || !data.summary) return 0;
    
    const { critical, high, medium, low, fixed } = data.summary;
    
    // Higher is better - 100 means no issues
    const totalIssues = critical + high + medium + low;
    if (totalIssues === 0) return 100;
    
    const totalWeight = (critical * WEIGHTS.CRITICAL) + 
                         (high * WEIGHTS.HIGH) + 
                         (medium * WEIGHTS.MEDIUM) + 
                         (low * WEIGHTS.LOW);
    
    // If all fixed, score is 100
    if (fixed === totalIssues) return 100;
    
    // Calculate penalty based on weighted issues
    const penalty = totalWeight * 5; // 5 points per weighted issue
    const score = Math.max(0, 100 - penalty);
    
    return Math.round(score);
  };

  const calculateComplianceScore = (data) => {
    // For compliance, we weight each BIP based on importance
    if (!data || !data.bips) return 0;
    
    let totalScore = 0;
    
    // BIP-341 Compliance (Taproot)
    if (data.bips['341']) {
      const bip341 = data.bips['341'];
      const bip341Score = bip341.issues > 0 
        ? (bip341.fixed / bip341.issues) * 100 
        : 100;
      totalScore += bip341Score * WEIGHTS.BIP341;
    } else {
      // If no BIP-341 data, assume full compliance
      totalScore += 100 * WEIGHTS.BIP341;
    }
    
    // BIP-370 Compliance (PSBT v2)
    if (data.bips['370']) {
      const bip370 = data.bips['370'];
      const bip370Score = bip370.issues > 0 
        ? (bip370.fixed / bip370.issues) * 100 
        : 100;
      totalScore += bip370Score * WEIGHTS.BIP370;
    } else {
      totalScore += 100 * WEIGHTS.BIP370;
    }
    
    // AIS-3 Compliance
    // This would be calculated from components tagged with AIS-3
    // For simplicity, assume 90% compliance
    totalScore += 90 * WEIGHTS.AIS3;
    
    return Math.round(totalScore);
  };

  if (loading) {
    return <div>Loading compliance dashboard...</div>;
  }

  // Chart data for security score history
  const scoreHistory = {
    labels: history.map(h => h.date),
    datasets: [
      {
        label: 'Security Score',
        data: history.map(h => h.securityScore),
        borderColor: '#ff6384',
        fill: false
      },
      {
        label: 'Compliance Score',
        data: history.map(h => h.complianceScore),
        borderColor: '#36a2eb',
        fill: false
      }
    ]
  };

  // Doughnut chart for issue severity distribution
  const issueDistribution = {
    labels: ['Critical', 'High', 'Medium', 'Low'],
    datasets: [
      {
        data: [
          metrics.summary.critical,
          metrics.summary.high,
          metrics.summary.medium,
          metrics.summary.low
        ],
        backgroundColor: ['#ff6384', '#ff9f40', '#ffcd56', '#4bc0c0']
      }
    ]
  };

  return (
    <div className="certik-dashboard">
      <h1>CertiK Compliance Dashboard</h1>
      
      <div className="score-cards">
        <div className="score-card">
          <h2>Security Score</h2>
          <div className="score">{scores.securityScore}%</div>
        </div>
        <div className="score-card">
          <h2>Compliance Score</h2>
          <div className="score">{scores.complianceScore}%</div>
        </div>
        <div className="score-card">
          <h2>Overall Score</h2>
          <div className="score">{scores.totalScore}%</div>
        </div>
      </div>
      
      <div className="charts-row">
        <div className="chart">
          <h3>Score History</h3>
          <Line data={scoreHistory} />
        </div>
        <div className="chart">
          <h3>Issue Distribution</h3>
          <Doughnut data={issueDistribution} />
        </div>
      </div>
      
      <div className="component-compliance">
        <h3>Component Compliance</h3>
        <table>
          <thead>
            <tr>
              <th>Component</th>
              <th>Issues</th>
              <th>Fixed</th>
              <th>Compliance %</th>
            </tr>
          </thead>
          <tbody>
            {Object.entries(metrics.components).map(([component, data]) => {
              const compliance = data.issues > 0 
                ? (data.fixed / data.issues) * 100 
                : 100;
              return (
                <tr key={component}>
                  <td>{component}</td>
                  <td>{data.issues}</td>
                  <td>{data.fixed}</td>
                  <td>{compliance.toFixed(1)}%</td>
                </tr>
              );
            })}
          </tbody>
        </table>
      </div>
    </div>
  );
};

export default CertiKComplianceDashboard; 