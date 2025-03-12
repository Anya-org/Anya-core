Describe "Test Comprehensive Deployments" {
    Context "Standalone Deployment" {
        It "Should install successfully" {
            $result = Install-AnyaCore -DeploymentType "Standalone"
            $result | Should -Be $true
        }
    }

    Context "Node Deployment" {
        It "Should install successfully" {
            $result = Install-AnyaCore -DeploymentType "Node" -IsNetworked $true
            $result | Should -Be $true
        }
    }

    Context "Cluster Deployment" {
        It "Should install successfully" {
            $result = Install-AnyaCore -DeploymentType "Cluster" -IsNetworked $true
            $result | Should -Be $true
        }
    }
    
    Context "System Checks" {
        It "Should return valid system check info" {
            $checks = Test-SystemChecks
            $checks.FreeDiskSpace | Should -Match "\d+\.\d+ GB"
        }
    }
}