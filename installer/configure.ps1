# Add enterprise DID template support
function Install-EnterpriseDID {
    param([string]$OrgName)
    $template = Get-Content "$PSScriptRoot/templates/enterprise_did.json"
    $template = $template.Replace("{{ORG_NAME}}", $OrgName)
    web5 did create --template $template
} 