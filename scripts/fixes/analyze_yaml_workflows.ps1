# Script to analyze and optimize GitHub workflow YAML files
# Identifies duplication and suggests consolidation opportunities

param(
    [switch]$GenerateReport,
    [switch]$ApplyFixes
)

# Set default values for switch parameters
$GenerateReport = $true
$ApplyFixes = $false

$scriptName = "GitHub Workflow YAML Analyzer"
$scriptVersion = "1.0.0"
$workflowDir = Join-Path $PSScriptRoot "..\..\\.github\\workflows"
$reportDir = Join-Path $PSScriptRoot "..\\..\\reports\\workflow-analysis"

# Ensure report directory exists
if ($GenerateReport -and -not (Test-Path $reportDir)) {
    New-Item -ItemType Directory -Path $reportDir -Force | Out-Null
}

Write-Host "===== $scriptName v$scriptVersion =====" -ForegroundColor Cyan
Write-Host "Analyzing YAML syntax in GitHub workflow files..."

# Get all workflow files
$workflowFiles = Get-ChildItem -Path $workflowDir -Filter "*.yml" -ErrorAction SilentlyContinue

if ($workflowFiles.Count -eq 0) {
    Write-Host "No workflow files found in $workflowDir" -ForegroundColor Yellow
    exit 0
}

Write-Host "Found $($workflowFiles.Count) workflow files to analyze" -ForegroundColor Cyan

# Structures to track common elements
$commonTriggers = @{}
$commonJobs = @{}
$commonSteps = @{}
$commonEnvVars = @{}
$fileStructures = @{}
$similarFiles = @{}
$duplicateContent = @{}

# Function to convert YAML to PowerShell object
function Convert-YamlToObject {
    param(
        [string]$YamlContent
    )
    
    try {
        # Remove any BOM if present
        $utf8NoBom = New-Object System.Text.UTF8Encoding $false
        $YamlContent = $utf8NoBom.GetString($utf8NoBom.GetBytes($YamlContent))

        # PowerShell Core 6.0+ has ConvertFrom-Yaml built in
        if ($PSVersionTable.PSVersion.Major -ge 6) {
            $yamlObject = $YamlContent | ConvertFrom-Yaml
        } else {
            # For older PowerShell, we use a simplified approach
            # This is a very basic YAML parser for simple structures only
            $yamlObject = @{}
            $lines = $YamlContent -split "`n"
            # These variables aren't used in the simplified implementation
            # but would be necessary for a full YAML parser
            $currentObject = $yamlObject
            
            foreach ($line in $lines) {
                if ([string]::IsNullOrWhiteSpace($line) -or $line.Trim().StartsWith("#")) {
                    continue  # Skip empty lines and comments
                }
                
                # Get the indent level from leading spaces
                if ($line -match "^(\s*)") {
                    # Not using indent in this simplified implementation
                    $line = $line.Trim()
                }
                
                if ($line -match "^(\w+):\s*(.*)$") {
                    $key = $matches[1]
                    $value = $matches[2].Trim()
                    
                    if ($value) {
                        $currentObject[$key] = $value
                    } else {
                        $currentObject[$key] = @{}
                    }
                }
            }
        }
        
        return $yamlObject
    } catch {
        Write-Host "Error parsing YAML: $_" -ForegroundColor Red
        return $null
    }
}

# Function to extract common triggers
function Get-CommonTriggers {
    param(
        [PSCustomObject]$YamlObject,
        [string]$FileName
    )
    
    if ($YamlObject.on) {
        $triggerHash = $YamlObject.on | ConvertTo-Json -Compress
        
        if (-not $commonTriggers.ContainsKey($triggerHash)) {
            $commonTriggers[$triggerHash] = @{
                Trigger = $YamlObject.on
                Files = @()
            }
        }
        
        $commonTriggers[$triggerHash].Files += $FileName
    }
}

# Function to extract common jobs
function Get-CommonJobs {
    param(
        [PSCustomObject]$YamlObject,
        [string]$FileName
    )
    
    if ($YamlObject.jobs) {
        foreach ($jobName in $YamlObject.jobs.Keys) {
            $job = $YamlObject.jobs[$jobName]
            $jobHash = $job | ConvertTo-Json -Compress
            
            if (-not $commonJobs.ContainsKey($jobHash)) {
                $commonJobs[$jobHash] = @{
                    Job = $job
                    Name = $jobName
                    Files = @()
                }
            }
            
            $commonJobs[$jobHash].Files += "${FileName}:${jobName}"
        }
    }
}

# Function to extract common steps
function Get-CommonSteps {
    param(
        [PSCustomObject]$YamlObject,
        [string]$FileName
    )
    
    if ($YamlObject.jobs) {
        foreach ($jobName in $YamlObject.jobs.Keys) {
            $job = $YamlObject.jobs[$jobName]
            
            if ($job.steps) {
                for ($i = 0; $i -lt $job.steps.Count; $i++) {
                    $step = $job.steps[$i]
                    $stepHash = $step | ConvertTo-Json -Compress
                    
                    if (-not $commonSteps.ContainsKey($stepHash)) {
                        $commonSteps[$stepHash] = @{
                            Step = $step
                            Files = @()
                        }
                    }
                    
                    $commonSteps[$stepHash].Files += "${FileName}:${jobName}:${i}"
                }
            }
        }
    }
}

# Function to extract common environment variables
function Get-CommonEnvVars {
    param(
        [PSCustomObject]$YamlObject,
        [string]$FileName
    )
    
    # Extract workflow-level env vars
    if ($YamlObject.env) {
        $envHash = $YamlObject.env | ConvertTo-Json -Compress
        
        if (-not $commonEnvVars.ContainsKey($envHash)) {
            $commonEnvVars[$envHash] = @{
                EnvVars = $YamlObject.env
                Files = @()
            }
        }
        
        $commonEnvVars[$envHash].Files += "${FileName}:workflow"
    }
    
    # Extract job-level env vars
    if ($YamlObject.jobs) {
        foreach ($jobName in $YamlObject.jobs.Keys) {
            $job = $YamlObject.jobs[$jobName]
            
            if ($job.env) {
                $envHash = $job.env | ConvertTo-Json -Compress
                
                if (-not $commonEnvVars.ContainsKey($envHash)) {
                    $commonEnvVars[$envHash] = @{
                        EnvVars = $job.env
                        Files = @()
                    }
                }
                
                $commonEnvVars[$envHash].Files += "${FileName}:${jobName}"
            }
        }
    }
}

# Function to detect similar files
function Get-SimilarFiles {
    param(
        [PSCustomObject]$YamlObject,
        [string]$FileName,
        [string]$FileContent
    )
    
    $fileHash = Get-FileStructureHash -YamlObject $YamlObject
    $fileStructures[$FileName] = $fileHash
    
    # Check for exact content duplicates
    $contentHash = [System.Security.Cryptography.SHA256]::Create().ComputeHash(
        [System.Text.Encoding]::UTF8.GetBytes($FileContent)
    ) | ForEach-Object { $_.ToString("x2") } | Join-String
    
    if (-not $duplicateContent.ContainsKey($contentHash)) {
        $duplicateContent[$contentHash] = @()
    }
    
    $duplicateContent[$contentHash] += $FileName
}

# Function to get file structure hash
function Get-FileStructureHash {
    param(
        [PSCustomObject]$YamlObject
    )
    
    # Create a simplified structure representation
    $structure = @{
        HasTriggers = $null -ne $YamlObject.on
        TriggerTypes = @()
        JobCount = 0
        JobTypes = @()
        StepPatterns = @()
    }
    
    # Add trigger types
    if ($YamlObject.on -is [Hashtable]) {
        $structure.TriggerTypes = $YamlObject.on.Keys
    } elseif ($YamlObject.on -is [String]) {
        $structure.TriggerTypes = @($YamlObject.on)
    }
    
    # Add job information
    if ($YamlObject.jobs) {
        $structure.JobCount = $YamlObject.jobs.Count
        
        foreach ($jobName in $YamlObject.jobs.Keys) {
            $job = $YamlObject.jobs[$jobName]
            $jobType = ""
            
            if ($job."runs-on") {
                $jobType += "runs-on:$($job."runs-on") "
            }
            
            if ($job.container) {
                $jobType += "container "
            }
            
            if ($job."needs") {
                $jobType += "needs:$($job."needs" -join ',') "
            }
            
            if ($job.steps -and $job.steps.Count -gt 0) {
                $jobType += "steps:$($job.steps.Count)"
            }
            
            $structure.JobTypes += $jobType.Trim()
            
            # Add step patterns
            if ($job.steps) {
                $stepPattern = ""
                foreach ($step in $job.steps) {
                    if ($step.uses) {
                        $uses = $step.uses -replace '@.*$', ''  # Remove version info
                        $stepPattern += "U:$uses;"
                    } elseif ($step.run) {
                        $stepPattern += "R;"
                    } else {
                        $stepPattern += "O;"
                    }
                }
                $structure.StepPatterns += $stepPattern
            }
        }
    }
    
    return $structure | ConvertTo-Json -Compress
}

# Main analysis loop
foreach ($file in $workflowFiles) {
    Write-Host "Analyzing: $($file.Name)" -ForegroundColor Yellow
    
    # Read the file content
    $content = Get-Content -Path $file.FullName -Raw
    
    # Convert YAML to object
    $yamlObject = Convert-YamlToObject -YamlContent $content
    
    if ($yamlObject) {
        # Extract common elements
        Get-CommonTriggers -YamlObject $yamlObject -FileName $file.Name
        Get-CommonJobs -YamlObject $yamlObject -FileName $file.Name
        Get-CommonSteps -YamlObject $yamlObject -FileName $file.Name
        Get-CommonEnvVars -YamlObject $yamlObject -FileName $file.Name
        Get-SimilarFiles -YamlObject $yamlObject -FileName $file.Name -FileContent $content
    } else {
        Write-Host "  - Could not parse YAML content" -ForegroundColor Red
    }
}

# Process similar files
foreach ($fileHash in $fileStructures.Values | Group-Object | Where-Object { $_.Count -gt 1 }) {
    $similarFileNames = $fileStructures.Keys | Where-Object { $fileStructures[$_] -eq $fileHash.Name }
    
    foreach ($fileName in $similarFileNames) {
        $similarFiles[$fileName] = $similarFileNames | Where-Object { $_ -ne $fileName }
    }
}

# Generate consolidated workflow file suggestions
$consolidationSuggestions = @()

foreach ($contentHash in $duplicateContent.Keys) {
    $files = $duplicateContent[$contentHash]
    if ($files.Count -gt 1) {
        $consolidationSuggestions += @{
            Type = "ExactDuplicate"
            Files = $files
            Suggestion = "These files have identical content and can be consolidated into a single file."
        }
    }
}

foreach ($triggerHash in $commonTriggers.Keys) {
    $triggerInfo = $commonTriggers[$triggerHash]
    if ($triggerInfo.Files.Count -gt 1) {
        $consolidationSuggestions += @{
            Type = "CommonTrigger"
            Files = $triggerInfo.Files
            Trigger = $triggerInfo.Trigger
            Suggestion = "These files share identical triggers and may be candidates for consolidation."
        }
    }
}

foreach ($fileName in $similarFiles.Keys) {
    $similar = $similarFiles[$fileName]
    if ($similar.Count -gt 0) {
        $consolidationSuggestions += @{
            Type = "SimilarStructure"
            File = $fileName
            SimilarFiles = $similar
            Suggestion = "This file has a similar structure to other files and may be consolidated."
        }
    }
}

# Generate categories of workflow files
$categories = @{}

foreach ($file in $workflowFiles) {
    $fileName = $file.Name
    $category = ""
    
    # Categorize by naming patterns
    if ($fileName -match "-combined\.yml$") {
        $category = "Combined Workflows"
    } elseif ($fileName -match "-test\.yml$") {
        $category = "Test Workflows"
    } elseif ($fileName -match "compliance") {
        $category = "Compliance Workflows"
    } elseif ($fileName -match "security") {
        $category = "Security Workflows"
    } elseif ($fileName -match "build") {
        $category = "Build Workflows"
    } elseif ($fileName -match "deploy") {
        $category = "Deployment Workflows"
    } else {
        $category = "Other Workflows"
    }
    
    if (-not $categories.ContainsKey($category)) {
        $categories[$category] = @()
    }
    
    $categories[$category] += $fileName
}

# Generate consolidation recommendations
$consolidationRecommendations = @()

# Look for combined workflow opportunities within categories
foreach ($category in $categories.Keys) {
    $categoryFiles = $categories[$category]
    
    if ($categoryFiles.Count -gt 1) {
        # Check if there's already a combined file
        $hasCombined = $categoryFiles | Where-Object { $_ -match "-combined\.yml$" }
        
        if (-not $hasCombined) {
            $consolidationRecommendations += @{
                Category = $category
                Files = $categoryFiles
                Recommendation = "Consider creating a combined workflow file for the $category category."
            }
        } elseif ($hasCombined.Count -gt 1) {
            $consolidationRecommendations += @{
                Category = $category
                Files = $hasCombined
                Recommendation = "Multiple combined workflow files exist for $category. Consider consolidating further."
            }
        }
    }
}

# Generate report
if ($GenerateReport) {
    $reportDate = Get-Date -Format "yyyyMMdd-HHmmss"
    $reportFile = Join-Path $reportDir "workflow-analysis-$reportDate.md"
    
    $report = @"
# GitHub Workflow Analysis Report

Generated: $(Get-Date -Format "yyyy-MM-dd HH:mm:ss")

## Overview

Total Workflow Files: $($workflowFiles.Count)

## Categories

$(foreach ($category in $categories.Keys | Sort-Object) {
"### $category

$($categories[$category] -join "`n")
"
})

## Duplication Analysis

### Exact Duplicates

Files with identical content:

$(foreach ($contentHash in $duplicateContent.Keys | Where-Object { $duplicateContent[$_].Count -gt 1 }) {
"- Duplicate set: $($duplicateContent[$contentHash] -join ", ")"
})

### Common Triggers

Files sharing identical trigger configurations:

$(foreach ($triggerHash in $commonTriggers.Keys | Where-Object { $commonTriggers[$_].Files.Count -gt 1 }) {
$triggerInfo = $commonTriggers[$triggerHash]
"- Trigger: ``$($triggerInfo.Trigger | ConvertTo-Json -Compress -Depth 2)``
  - Files: $($triggerInfo.Files -join ", ")"
})

### Similar Files

Files with similar structure:

$(foreach ($fileName in $similarFiles.Keys | Sort-Object) {
"- $fileName is similar to: $($similarFiles[$fileName] -join ", ")"
})

## Common Jobs

Jobs that appear in multiple workflow files:

$(foreach ($jobHash in $commonJobs.Keys | Where-Object { $commonJobs[$_].Files.Count -gt 1 }) {
$jobInfo = $commonJobs[$jobHash]
"- Job: $($jobInfo.Name)
  - Files: $($jobInfo.Files -join ", ")"
})

## Common Environment Variables

Environment variable sets that appear in multiple workflow files:

$(foreach ($envHash in $commonEnvVars.Keys | Where-Object { $commonEnvVars[$_].Files.Count -gt 1 }) {
$envInfo = $commonEnvVars[$envHash]
"- Environment Variables: ``$($envInfo.EnvVars | ConvertTo-Json -Compress)``
  - Files: $($envInfo.Files -join ", ")"
})

## Consolidation Recommendations

$(foreach ($recommendation in $consolidationRecommendations) {
"### $($recommendation.Category)

$($recommendation.Recommendation)

Files: $($recommendation.Files -join ", ")
"
})

## Implementation Plan

Based on the analysis, the following consolidation actions are recommended:

1. **Create Reusable Workflow Components**
   - Extract common job definitions into reusable workflow files
   - Use GitHub's `jobs.<job_id>.uses` syntax to reference these workflows

2. **Consolidate Category Workflows**
   - Combine similar workflows within the same category
   - Use conditional execution based on event types

3. **Standardize Naming Conventions**
   - Use consistent naming patterns for all workflows
   - Follow the format: `<category>-<purpose>[-combined].yml`

4. **Implement Workflow Call Patterns**
   - Use workflow calls instead of duplicating steps
   - Create a hierarchy of workflows for better organization

## Specific Consolidation Actions

$(foreach ($suggestion in $consolidationSuggestions | Where-Object { $_.Type -eq "ExactDuplicate" }) {
"- **Consolidate Exact Duplicates**: $($suggestion.Files -join ", ")
  - Create a single workflow file that can be triggered by multiple events"
})

$(foreach ($suggestion in $consolidationSuggestions | Where-Object { $_.Type -eq "CommonTrigger" }) {
"- **Merge Files with Common Triggers**: $($suggestion.Files -join ", ")
  - Create a combined workflow with conditional job execution"
})

$(foreach ($category in $categories.Keys | Where-Object { $categories[$_].Count -gt 3 }) {
"- **Organize $category**: $($categories[$category].Count) files
  - Create a centralized workflow file that conditionally includes required components"
})
"@
    
    $report | Set-Content -Path $reportFile -Encoding UTF8
    Write-Host "Analysis report written to: $reportFile" -ForegroundColor Green
}

# Output summary
Write-Host "`nWorkflow Analysis Summary:" -ForegroundColor Cyan
Write-Host "  Total workflow files: $($workflowFiles.Count)" -ForegroundColor White
Write-Host "  Categories identified: $($categories.Count)" -ForegroundColor White
Write-Host "  Exact duplicates: $($duplicateContent.Values | Where-Object { $_.Count -gt 1 } | Measure-Object).Count" -ForegroundColor $(if (($duplicateContent.Values | Where-Object { $_.Count -gt 1 } | Measure-Object).Count -gt 0) { "Yellow" } else { "Green" })
Write-Host "  Common trigger patterns: $($commonTriggers.Values | Where-Object { $_.Files.Count -gt 1 } | Measure-Object).Count" -ForegroundColor White
Write-Host "  Common job patterns: $($commonJobs.Values | Where-Object { $_.Files.Count -gt 1 } | Measure-Object).Count" -ForegroundColor White
Write-Host "  Consolidation recommendations: $($consolidationRecommendations.Count)" -ForegroundColor Yellow

# If directed to apply fixes, generate consolidation script
if ($ApplyFixes) {
    Write-Host "`nGenerating consolidation implementation script..." -ForegroundColor Cyan
    
    $implementationScriptPath = Join-Path $scriptRoot "consolidate_workflows.ps1"
    
    $implementationScript = @"
# Workflow Consolidation Implementation Script
# Generated by analyze_yaml_workflows.ps1 on $(Get-Date -Format "yyyy-MM-dd HH:mm:ss")

param(
    [switch]`$DryRun
)

`$scriptName = "GitHub Workflow Consolidation Implementation"
`$scriptVersion = "1.0.0"
`$workflowDir = Join-Path `$PSScriptRoot "..\\..\\\.github\\workflows"
`$backupDir = Join-Path `$PSScriptRoot "..\\..\\\.github\\workflows_backup_`$(Get-Date -Format 'yyyyMMdd-HHmmss')"

Write-Host "===== `$scriptName v`$scriptVersion =====" -ForegroundColor Cyan
Write-Host "Implementing workflow consolidation..."

# Create backup directory
if (-not `$DryRun) {
    New-Item -ItemType Directory -Path `$backupDir -Force | Out-Null
    Write-Host "Created backup directory: `$backupDir" -ForegroundColor Green
    
    # Copy all workflow files to backup
    Copy-Item -Path "`$workflowDir\\*.yml" -Destination `$backupDir
    Write-Host "Backed up all workflow files" -ForegroundColor Green
}

# TODO: Implement workflow consolidation logic based on analysis
# This will be customized based on the actual analysis results

# Example consolidation code (to be replaced with actual logic):
<# 
# 1. Create reusable workflow components

# 2. Consolidate exact duplicates

# 3. Implement category-based consolidation

# 4. Clean up redundant workflows
#>

Write-Host "Workflow consolidation completed" -ForegroundColor Green
"@
    
    $implementationScript | Set-Content -Path $implementationScriptPath -Encoding UTF8
    Write-Host "Implementation script generated at: $implementationScriptPath" -ForegroundColor Green
}

Write-Host "Workflow analysis completed" -ForegroundColor Green
