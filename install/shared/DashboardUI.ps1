#Requires -Version 5.0

# Unified UI components
$global:uiConfig = @{
    Colors = @{
        Red      = [System.ConsoleColor]::Red
        Green    = [System.ConsoleColor]::Green 
        Yellow   = [System.ConsoleColor]::Yellow
        Cyan     = [System.ConsoleColor]::Cyan
        White    = [System.ConsoleColor]::White
        DarkCyan = [System.ConsoleColor]::DarkCyan
        Gray     = [System.ConsoleColor]::Gray
        Blue     = [System.ConsoleColor]::Blue
    }
    Box = @{
        TopLeft     = '┌'
        TopRight    = '┐'
        BottomLeft  = '└'
        BottomRight = '┘'
        Horizontal  = '─'
        Vertical    = '│'
        TeeRight    = '├'
        TeeLeft     = '┤'
    }
    Progress = @{
        Bar = '█'
        Empty = '░'
        Width = 50
    }
}

function Write-DashboardBlock {
    param(
        [string]$Title,
        [string[]]$Content,
        [int]$Width = 50,
        [System.ConsoleColor]$TitleColor = $uiConfig.Colors.Cyan,
        [switch]$NoNewLine
    )
    # ...existing Write-DashboardBlock code...
}

function Show-InstallProgress {
    param(
        [string]$Status,
        [int]$PercentComplete,
        [System.ConsoleColor]$Color = $uiConfig.Colors.Cyan
    )
    # ...existing Show-InstallProgress code...
}

Export-ModuleMember -Function * -Variable uiConfig
