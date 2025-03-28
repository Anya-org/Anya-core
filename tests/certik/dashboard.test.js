const { render, screen, waitFor } = require('@testing-library/react');
const axios = require('axios');
const MockAdapter = require('axios-mock-adapter');
const CertiKComplianceDashboard = require('../../src/web/dashboard/certik-compliance');

// Mock axios
const mockAxios = new MockAdapter(axios);

// Sample data
const sampleMetrics = {
  summary: {
    critical: 0,
    high: 1,
    medium: 1,
    low: 0,
    fixed: 1,
    pending: 2
  },
  components: {
    "HSM Interface": {issues: 1, fixed: 1},
    "Transaction Signer": {issues: 1, fixed: 0},
    "PSBT Manager": {issues: 1, fixed: 0}
  },
  bips: {
    "341": {issues: 2, fixed: 1},
    "370": {issues: 1, fixed: 0}
  },
  updated_at: "2025-08-10T14:30:00Z"
};

const sampleHistory = [
  {date: "2025-08-01", securityScore: 80, complianceScore: 85},
  {date: "2025-08-02", securityScore: 75, complianceScore: 85},
  {date: "2025-08-03", securityScore: 75, complianceScore: 90},
  {date: "2025-08-04", securityScore: 80, complianceScore: 90},
  {date: "2025-08-05", securityScore: 85, complianceScore: 95},
  {date: "2025-08-06", securityScore: 85, complianceScore: 95},
  {date: "2025-08-07", securityScore: 90, complianceScore: 95}
];

describe('CertiK Compliance Dashboard', () => {
  beforeEach(() => {
    mockAxios.reset();
    mockAxios.onGet('/api/certik/metrics').reply(200, sampleMetrics);
    mockAxios.onGet('/api/certik/history').reply(200, sampleHistory);
  });

  test('renders dashboard with metrics', async () => {
    render(<CertiKComplianceDashboard />);
    
    // Wait for data to load
    await waitFor(() => {
      expect(screen.getByText('CertiK Compliance Dashboard')).toBeInTheDocument();
    });
    
    // Check scores are displayed
    expect(screen.getByText('Security Score')).toBeInTheDocument();
    expect(screen.getByText('Compliance Score')).toBeInTheDocument();
    
    // Check component table
    expect(screen.getByText('HSM Interface')).toBeInTheDocument();
    expect(screen.getByText('Transaction Signer')).toBeInTheDocument();
    expect(screen.getByText('PSBT Manager')).toBeInTheDocument();
    
    // Check charts are rendered
    expect(screen.getByText('Score History')).toBeInTheDocument();
    expect(screen.getByText('Issue Distribution')).toBeInTheDocument();
  });

  test('calculates scores correctly', async () => {
    render(<CertiKComplianceDashboard />);
    
    await waitFor(() => {
      // The security score should be high since there are no critical issues
      const securityScoreElement = screen.getByText(/9[0-9]%/);
      expect(securityScoreElement).toBeInTheDocument();
      
      // BIP-341 is 50% compliant, BIP-370 is 0% compliant
      // With weights of 0.4, 0.3, and 0.3 for BIP341, BIP370, and AIS3
      // Score should be around 65-75%
      const complianceScoreElement = screen.getByText(/[6-7][0-9]%/);
      expect(complianceScoreElement).toBeInTheDocument();
    });
  });
}); 