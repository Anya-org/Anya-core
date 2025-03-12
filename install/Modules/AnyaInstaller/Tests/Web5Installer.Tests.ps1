Describe "Web5Installer" {
    Context "InstallWeb5Layer" {
        It "Should install Web5 layer successfully" {
            $web5Installer = [Web5Installer]::new()
            $web5Installer.InstallWeb5Layer() | Should -Be $true
        }
    }
} 