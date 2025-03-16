import { readFileSync, writeFileSync, existsSync } from 'fs';
import { join } from 'path';

interface BipInfo {
  name: string;
  required: boolean;
}

interface BipComplianceDetail {
  name: string;
  required: boolean;
  status: string;
  implementations: string[];
}

interface ContractDetail {
  path: string;
  size: number;
  bipImplementations: Record<string, boolean>;
}

interface BIPCompliance {
  [key: string]: {
    implemented: boolean;
    implementations: string[];
    status: "Implemented" | "Not Implemented";
  };
}

interface ContractInfo {
  path: string;
  size: number;
  bipImplementations: Record<string, boolean>;
}

interface ComplianceReport {
  timestamp: string;
  contracts: Record<string, ContractInfo>;
  bipCompliance: BIPCompliance;
  overallCompliance: string;
}

// BIP Compliance Checklist
const bipChecklist: { [key: string]: { required: boolean } } = {
  '341': { required: true },
  '174': { required: true },
  '370': { required: false },
  '342': { required: true }
};

// Contract paths to check
const contractPaths = [
  "dao/core/dao-core.clar",
  "dao/traits/dao-trait.clar",
  "src/contracts/dao.clar",
  "src/contracts/governance_token.clar",
  "src/contracts/bitcoin-issuance.clar",
  "src/contracts/dex-adapter.clar",
  "dao/extensions/token-economics.clar"
];

// Generate report
function generateComplianceReport() {
  const report: ComplianceReport = {
    timestamp: new Date().toISOString(),
    contracts: {} as Record<string, ContractInfo>,
    bipCompliance: Object.keys(bipChecklist).reduce((acc, bip) => ({
      ...acc,
      [bip]: {
        implemented: false,
        implementations: [],
        status: "Not Implemented"
      }
    }), {}) as BIPCompliance,
    overallCompliance: "PASS"
  };
  
  console.log("Checking BIP compliance in contracts...");
  
  // Check each contract
  let validContractsFound = 0;
  for (const contractPath of contractPaths) {
    try {
      if (!existsSync(contractPath)) {
        console.log(`Contract file not found: ${contractPath}`);
        continue;
      }
      
      validContractsFound++;
      const content = readFileSync(join(process.cwd(), contractPath), 'utf8');
      const contractName = contractPath.split('/').pop() || '';
      
      console.log(`Analyzing contract: ${contractName}`);
      
      report.contracts[contractName] = {
        path: contractPath,
        size: content.length,
        bipImplementations: {} as Record<string, boolean>
      };
      
      // Check for BIP implementations
      for (const [bip, info] of Object.entries(bipChecklist)) {
        const bipPattern = new RegExp(`BIP-${bip}|BIP ${bip}`, 'i');
        
        if (bipPattern.test(content)) {
          report.bipCompliance[bip].implementations.push(contractName);
          report.contracts[contractName].bipImplementations[bip] = true;
          
          if (report.bipCompliance[bip].status !== "Implemented") {
            report.bipCompliance[bip].status = "Implemented";
            console.log(`✅ Found BIP-${bip} in ${contractName}`);
          }
        } else {
          report.contracts[contractName].bipImplementations[bip] = false;
        }
      }
    } catch (error) {
      console.error(`Error processing ${contractPath}:`, error);
    }
  }
  
  console.log(`Analyzed ${validContractsFound} contracts out of ${contractPaths.length}`);
  
  // Determine overall compliance
  for (const [bip, info] of Object.entries(bipChecklist)) {
    if (info.required && report.bipCompliance[bip].status !== "Implemented") {
      report.overallCompliance = "FAIL";
      console.log(`❌ Required BIP-${bip} not found in any contract`);
    }
  }
  
  // Write report
  const outputPath = join(process.cwd(), 'compliance-report.json');
  writeFileSync(
    outputPath,
    JSON.stringify(report, null, 2)
  );
  
  console.log(`Compliance report generated: ${outputPath}`);
  console.log(`Overall compliance status: ${report.overallCompliance}`);
}

// Run the report generator
generateComplianceReport(); 