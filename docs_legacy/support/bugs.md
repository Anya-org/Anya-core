# Bug Reports

Bug reporting guidelines and procedures for Anya Core products and services.

## Overview

This document provides guidelines for reporting bugs, tracking issues, and contributing to the resolution of problems in Anya Core systems.

## How to Report a Bug

### Before Reporting

#### Check Existing Issues

1. Search the [GitHub Issues](https://github.com/anya-core/issues) for similar problems
2. Check our [Known Issues](../known-issues.md) documentation
3. Review the [FAQ](../support/technical.md#frequently-asked-questions) for common solutions

#### Gather Information

```bash
# System Information Script
#!/bin/bash

echo "=== Anya Core Bug Report Information ==="
echo "Generated: $(date)"
echo

# System details
echo "System Information:"
echo "  OS: $(uname -a)"
echo "  Architecture: $(uname -m)"
echo "  Kernel: $(uname -r)"
echo

# Software versions
echo "Software Versions:"
if command -v anya-core &> /dev/null; then
    echo "  Anya Core: $(anya-core --version)"
fi

if command -v node &> /dev/null; then
    echo "  Node.js: $(node --version)"
fi

if command -v npm &> /dev/null; then
    echo "  NPM: $(npm --version)"
fi

if command -v cargo &> /dev/null; then
    echo "  Rust: $(rustc --version)"
fi

if command -v python3 &> /dev/null; then
    echo "  Python: $(python3 --version)"
fi
echo

# Memory and disk usage
echo "Resource Usage:"
echo "  Memory: $(free -h | grep Mem | awk '{print $3 "/" $2}')"
echo "  Disk: $(df -h / | awk 'NR==2{print $3 "/" $2 " (" $5 " used)"}')"
echo

# Network connectivity
echo "Network Status:"
if ping -c 1 8.8.8.8 &> /dev/null; then
    echo "  Internet: Connected"
else
    echo "  Internet: Disconnected"
fi

if curl -s https://api.anya-core.org/health &> /dev/null; then
    echo "  Anya API: Accessible"
else
    echo "  Anya API: Inaccessible"
fi
echo

echo "=== End of System Information ==="
```

### Bug Report Template

#### Severity Classification

- **Critical**: System crash, data loss, security vulnerability
- **High**: Major feature broken, significant impact on functionality
- **Medium**: Minor feature issue, workaround available
- **Low**: Cosmetic issue, documentation problem

#### Report Format

```markdown
# Bug Report

## Summary
Brief description of the issue

## Environment
- **OS**: [Operating System and version]
- **Anya Core Version**: [Version number]
- **Browser**: [If web-related, browser and version]
- **Hardware**: [Relevant hardware information]

## Steps to Reproduce
1. Step one
2. Step two
3. Step three
4. ...

## Expected Behavior
What you expected to happen

## Actual Behavior
What actually happened

## Screenshots/Logs
[Attach relevant screenshots, error logs, or console output]

## Additional Context
Any additional information that might be helpful

## Severity
[Critical/High/Medium/Low]

## Workaround
[If you found a workaround, describe it here]
```

### Code Examples for Common Bug Reports

#### Performance Issues

```python
import time
import psutil
import logging

class PerformanceBugReporter:
    def __init__(self):
        self.logger = logging.getLogger(__name__)
        
    def report_performance_issue(self, operation_name: str, expected_time: float):
        """Report performance issues with detailed metrics"""
        
        # Measure performance
        start_time = time.time()
        start_memory = psutil.Process().memory_info().rss
        
        try:
            # Execute the operation being tested
            result = self.execute_operation(operation_name)
            
            end_time = time.time()
            end_memory = psutil.Process().memory_info().rss
            
            execution_time = end_time - start_time
            memory_delta = end_memory - start_memory
            
            if execution_time > expected_time:
                self.logger.warning(f"Performance issue detected in {operation_name}")
                self.logger.warning(f"Expected: {expected_time}s, Actual: {execution_time}s")
                self.logger.warning(f"Memory usage: {memory_delta / 1024 / 1024:.2f} MB")
                
                # Create detailed bug report
                bug_report = {
                    'type': 'performance',
                    'operation': operation_name,
                    'expected_time': expected_time,
                    'actual_time': execution_time,
                    'memory_usage': memory_delta,
                    'system_info': self.get_system_info(),
                    'timestamp': time.time()
                }
                
                return bug_report
                
        except Exception as e:
            self.logger.error(f"Exception during {operation_name}: {str(e)}")
            raise
    
    def get_system_info(self):
        """Collect system information for bug reports"""
        return {
            'cpu_percent': psutil.cpu_percent(),
            'memory_percent': psutil.virtual_memory().percent,
            'disk_usage': psutil.disk_usage('/').percent,
            'load_average': psutil.getloadavg(),
            'process_count': len(psutil.pids())
        }
```

#### Transaction Issues

```typescript
interface TransactionBugReport {
  transaction_id: string;
  error_type: 'validation' | 'network' | 'fee' | 'confirmation' | 'other';
  error_message: string;
  transaction_details: TransactionDetails;
  network_conditions: NetworkConditions;
  retry_attempts: number;
  timestamp: Date;
}

class TransactionBugReporter {
  async reportTransactionIssue(
    transactionId: string, 
    error: Error
  ): Promise<TransactionBugReport> {
    
    // Gather transaction details
    const transactionDetails = await this.getTransactionDetails(transactionId);
    
    // Check network conditions
    const networkConditions = await this.getNetworkConditions();
    
    // Classify error type
    const errorType = this.classifyTransactionError(error);
    
    const bugReport: TransactionBugReport = {
      transaction_id: transactionId,
      error_type: errorType,
      error_message: error.message,
      transaction_details: transactionDetails,
      network_conditions: networkConditions,
      retry_attempts: transactionDetails.retry_count || 0,
      timestamp: new Date()
    };
    
    // Log the bug report
    await this.logBugReport(bugReport);
    
    // Submit to bug tracking system
    await this.submitBugReport(bugReport);
    
    return bugReport;
  }
  
  private classifyTransactionError(error: Error): string {
    const errorMessage = error.message.toLowerCase();
    
    if (errorMessage.includes('insufficient funds')) return 'validation';
    if (errorMessage.includes('fee too low')) return 'fee';
    if (errorMessage.includes('network')) return 'network';
    if (errorMessage.includes('confirmation')) return 'confirmation';
    
    return 'other';
  }
  
  private async getNetworkConditions(): Promise<NetworkConditions> {
    return {
      mempool_size: await this.getMempoolSize(),
      average_fee_rate: await this.getAverageFeeRate(),
      block_height: await this.getCurrentBlockHeight(),
      network_hash_rate: await this.getNetworkHashRate(),
      confirmation_times: await this.getRecentConfirmationTimes()
    };
  }
}
```

## Bug Tracking Workflow

### Issue Lifecycle

#### States

```typescript
enum BugStatus {
  REPORTED = 'reported',
  CONFIRMED = 'confirmed',
  ASSIGNED = 'assigned',
  IN_PROGRESS = 'in_progress',
  TESTING = 'testing',
  RESOLVED = 'resolved',
  VERIFIED = 'verified',
  CLOSED = 'closed',
  REOPENED = 'reopened'
}

interface BugTrackingWorkflow {
  bug_id: string;
  status: BugStatus;
  assignee: string;
  reporter: string;
  created_at: Date;
  updated_at: Date;
  resolved_at?: Date;
  resolution: string;
  verification_steps: string[];
  related_issues: string[];
}

class BugTracker {
  async progressBugStatus(
    bugId: string, 
    newStatus: BugStatus, 
    notes: string
  ): Promise<void> {
    
    const bug = await this.getBug(bugId);
    const validTransition = this.validateStatusTransition(bug.status, newStatus);
    
    if (!validTransition) {
      throw new Error(`Invalid status transition from ${bug.status} to ${newStatus}`);
    }
    
    // Update bug status
    await this.updateBugStatus(bugId, newStatus, notes);
    
    // Trigger appropriate actions
    await this.triggerStatusActions(bugId, newStatus);
    
    // Send notifications
    await this.sendStatusNotifications(bug, newStatus);
  }
  
  private validateStatusTransition(
    currentStatus: BugStatus, 
    newStatus: BugStatus
  ): boolean {
    const validTransitions: Record<BugStatus, BugStatus[]> = {
      [BugStatus.REPORTED]: [BugStatus.CONFIRMED, BugStatus.CLOSED],
      [BugStatus.CONFIRMED]: [BugStatus.ASSIGNED, BugStatus.CLOSED],
      [BugStatus.ASSIGNED]: [BugStatus.IN_PROGRESS, BugStatus.CLOSED],
      [BugStatus.IN_PROGRESS]: [BugStatus.TESTING, BugStatus.RESOLVED, BugStatus.CLOSED],
      [BugStatus.TESTING]: [BugStatus.RESOLVED, BugStatus.IN_PROGRESS],
      [BugStatus.RESOLVED]: [BugStatus.VERIFIED, BugStatus.REOPENED],
      [BugStatus.VERIFIED]: [BugStatus.CLOSED, BugStatus.REOPENED],
      [BugStatus.CLOSED]: [BugStatus.REOPENED],
      [BugStatus.REOPENED]: [BugStatus.ASSIGNED, BugStatus.IN_PROGRESS]
    };
    
    return validTransitions[currentStatus]?.includes(newStatus) || false;
  }
}
```

### Triage Process

#### Priority Assignment

```python
class BugTriageSystem:
    def __init__(self):
        self.severity_weights = {
            'critical': 10,
            'high': 7,
            'medium': 4,
            'low': 1
        }
        
        self.impact_multipliers = {
            'security': 3.0,
            'data_loss': 2.5,
            'performance': 1.5,
            'functionality': 1.2,
            'cosmetic': 0.5
        }
    
    def calculate_priority_score(self, bug_report: dict) -> float:
        """Calculate priority score for bug triage"""
        
        severity = bug_report.get('severity', 'low')
        impact_type = bug_report.get('impact_type', 'functionality')
        affected_users = bug_report.get('affected_users', 1)
        
        base_score = self.severity_weights.get(severity, 1)
        impact_multiplier = self.impact_multipliers.get(impact_type, 1.0)
        user_factor = min(math.log10(affected_users + 1), 3.0)  # Cap at 1000 users
        
        priority_score = base_score * impact_multiplier * (1 + user_factor)
        
        return priority_score
    
    def assign_bug_priority(self, bug_report: dict) -> str:
        """Assign priority level based on calculated score"""
        
        score = self.calculate_priority_score(bug_report)
        
        if score >= 20:
            return 'P0 - Critical'
        elif score >= 10:
            return 'P1 - High'
        elif score >= 5:
            return 'P2 - Medium'
        else:
            return 'P3 - Low'
    
    async def triage_bug(self, bug_id: str) -> TriageResult:
        """Perform complete bug triage"""
        
        bug_report = await self.get_bug_report(bug_id)
        
        # Calculate priority
        priority = self.assign_bug_priority(bug_report)
        
        # Assign to appropriate team
        team = self.assign_team(bug_report)
        
        # Estimate effort
        effort_estimate = await self.estimate_effort(bug_report)
        
        # Check for duplicates
        duplicates = await self.find_duplicate_bugs(bug_report)
        
        # Generate triage recommendations
        recommendations = self.generate_triage_recommendations(
            bug_report, priority, effort_estimate, duplicates
        )
        
        return TriageResult(
            bug_id=bug_id,
            priority=priority,
            assigned_team=team,
            effort_estimate=effort_estimate,
            duplicate_candidates=duplicates,
            recommendations=recommendations,
            triage_timestamp=datetime.now()
        )
```

## Quality Assurance

### Bug Verification

#### Verification Checklist

```markdown
## Bug Verification Checklist

### Pre-Verification
- [ ] Bug report contains all required information
- [ ] Steps to reproduce are clear and complete
- [ ] Environment information is provided
- [ ] Severity classification is appropriate

### Reproduction
- [ ] Able to reproduce the issue following provided steps
- [ ] Issue occurs consistently
- [ ] Issue occurs in clean environment
- [ ] Screenshots/logs match the reported behavior

### Impact Assessment
- [ ] Confirmed severity level
- [ ] Identified affected user groups
- [ ] Assessed business impact
- [ ] Checked for data integrity issues

### Technical Analysis
- [ ] Root cause identified
- [ ] Dependencies analyzed
- [ ] Similar issues reviewed
- [ ] Regression potential assessed

### Documentation
- [ ] Verification notes documented
- [ ] Additional reproduction steps added
- [ ] Impact analysis recorded
- [ ] Resolution approach outlined
```

#### Verification Tools

```rust
use std::process::Command;
use serde_json::Value;

pub struct BugVerificationTools {
    pub environment: String,
    pub test_data_path: String,
}

impl BugVerificationTools {
    pub fn new(environment: &str) -> Self {
        Self {
            environment: environment.to_string(),
            test_data_path: format!("./test-data/{}", environment),
        }
    }
    
    pub async fn reproduce_bug(&self, bug_id: &str, steps: Vec<String>) -> Result<ReproductionResult, Error> {
        let mut reproduction_log = Vec::new();
        let mut success = true;
        
        for (index, step) in steps.iter().enumerate() {
            println!("Executing step {}: {}", index + 1, step);
            
            let result = self.execute_step(step).await;
            match result {
                Ok(output) => {
                    reproduction_log.push(format!("Step {} successful: {}", index + 1, output));
                }
                Err(error) => {
                    reproduction_log.push(format!("Step {} failed: {}", index + 1, error));
                    success = false;
                    break;
                }
            }
        }
        
        Ok(ReproductionResult {
            bug_id: bug_id.to_string(),
            reproduction_successful: success,
            execution_log: reproduction_log,
            environment: self.environment.clone(),
            timestamp: chrono::Utc::now(),
        })
    }
    
    pub async fn collect_diagnostic_data(&self, bug_id: &str) -> Result<DiagnosticData, Error> {
        // Collect system logs
        let system_logs = self.collect_system_logs().await?;
        
        // Collect application logs
        let app_logs = self.collect_application_logs().await?;
        
        // Collect performance metrics
        let performance_metrics = self.collect_performance_metrics().await?;
        
        // Collect network traces
        let network_traces = self.collect_network_traces().await?;
        
        Ok(DiagnosticData {
            bug_id: bug_id.to_string(),
            system_logs,
            application_logs,
            performance_metrics,
            network_traces,
            collection_timestamp: chrono::Utc::now(),
        })
    }
    
    async fn execute_step(&self, step: &str) -> Result<String, Error> {
        // Parse and execute the step command
        let output = Command::new("sh")
            .arg("-c")
            .arg(step)
            .output()
            .map_err(|e| Error::ExecutionError(e.to_string()))?;
        
        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Err(Error::StepFailed(String::from_utf8_lossy(&output.stderr).to_string()))
        }
    }
}
```

## Bug Prevention

### Static Analysis Integration

```yaml
# GitHub Actions Workflow for Bug Prevention
name: Bug Prevention Pipeline

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main ]

jobs:
  static-analysis:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v3
    
    - name: Set up Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        components: clippy, rustfmt
    
    - name: Run Clippy
      run: cargo clippy -- -D warnings
    
    - name: Run Security Audit
      run: cargo audit
    
    - name: Check Formatting
      run: cargo fmt -- --check
  
  unit-tests:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v3
    
    - name: Run Unit Tests
      run: cargo test --all
    
    - name: Generate Coverage Report
      run: cargo tarpaulin --out xml
    
    - name: Upload Coverage
      uses: codecov/codecov-action@v1

  integration-tests:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v3
    
    - name: Start Test Environment
      run: docker-compose -f docker-compose.test.yml up -d
    
    - name: Run Integration Tests
      run: cargo test --test integration_tests
    
    - name: Clean Up Test Environment
      run: docker-compose -f docker-compose.test.yml down
```

### Code Review Guidelines

```markdown
## Code Review Checklist for Bug Prevention

### General Code Quality
- [ ] Code follows established style guidelines
- [ ] Functions are appropriately sized and focused
- [ ] Variable and function names are descriptive
- [ ] Complex logic is commented and documented

### Error Handling
- [ ] All error conditions are properly handled
- [ ] Error messages are informative and actionable
- [ ] Resources are properly cleaned up in error cases
- [ ] Failures are logged with appropriate context

### Security Considerations
- [ ] Input validation is implemented
- [ ] SQL injection prevention measures are in place
- [ ] Authentication and authorization are properly enforced
- [ ] Sensitive data is properly protected

### Performance
- [ ] No obvious performance bottlenecks
- [ ] Database queries are optimized
- [ ] Caching is used appropriately
- [ ] Resource usage is reasonable

### Testing
- [ ] Unit tests cover critical functionality
- [ ] Edge cases are tested
- [ ] Error conditions are tested
- [ ] Tests are maintainable and readable
```

## Bug Metrics and Reporting

### Key Performance Indicators

```python
class BugMetricsCollector:
    def __init__(self):
        self.metrics_db = MetricsDatabase()
    
    async def calculate_bug_metrics(self, period: str) -> BugMetrics:
        """Calculate comprehensive bug metrics for reporting"""
        
        # Bug discovery metrics
        bugs_reported = await self.count_bugs_by_period(period, 'reported')
        bugs_resolved = await self.count_bugs_by_period(period, 'resolved')
        
        # Resolution time metrics
        avg_resolution_time = await self.calculate_avg_resolution_time(period)
        resolution_times_by_severity = await self.resolution_times_by_severity(period)
        
        # Quality metrics
        bug_reopen_rate = await self.calculate_reopen_rate(period)
        escaped_bugs = await self.count_escaped_bugs(period)
        
        # Trend analysis
        bug_trend = await self.analyze_bug_trends(period)
        
        return BugMetrics(
            period=period,
            bugs_reported=bugs_reported,
            bugs_resolved=bugs_resolved,
            resolution_rate=bugs_resolved / max(bugs_reported, 1),
            avg_resolution_time=avg_resolution_time,
            resolution_times_by_severity=resolution_times_by_severity,
            reopen_rate=bug_reopen_rate,
            escaped_bugs=escaped_bugs,
            trend_analysis=bug_trend
        )
    
    async def generate_bug_report(self, period: str) -> BugReport:
        """Generate comprehensive bug report"""
        
        metrics = await self.calculate_bug_metrics(period)
        
        # Top issues by impact
        top_issues = await self.identify_top_issues(period)
        
        # Team performance
        team_metrics = await self.calculate_team_metrics(period)
        
        # Recommendations
        recommendations = await self.generate_recommendations(metrics, top_issues)
        
        return BugReport(
            period=period,
            generation_date=datetime.now(),
            summary_metrics=metrics,
            top_issues=top_issues,
            team_performance=team_metrics,
            recommendations=recommendations,
            action_items=await self.create_action_items(recommendations)
        )
```

## See Also

- [Technical Support](./technical.md)
- [Contributing Guidelines](../CONTRIBUTING.md)
- [Security Guidelines](../archive/SECURITY_GUIDELINES.md)
- [Development Workflow](../development/workflow.md)

---

*This document is part of the Anya Core Quality Assurance Framework and is updated regularly.*
