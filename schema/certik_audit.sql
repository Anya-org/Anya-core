-- CertiK Audit Database Schema
CREATE TABLE certik_findings (
    id SERIAL PRIMARY KEY,
    issue_number INTEGER NOT NULL,
    title VARCHAR(255) NOT NULL,
    severity VARCHAR(50) NOT NULL,
    component VARCHAR(100) NOT NULL,
    bip VARCHAR(50),
    description TEXT,
    status VARCHAR(50) NOT NULL DEFAULT 'open',
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    fixed_at TIMESTAMP NULL,
    fix_commit VARCHAR(100) NULL,
    cvss_score DECIMAL(3,1) NULL
);

CREATE TABLE certik_components (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL UNIQUE,
    bip VARCHAR(50) NULL,
    ais_level INTEGER NULL,
    priority INTEGER NOT NULL DEFAULT 1
);

CREATE TABLE certik_audit_history (
    id SERIAL PRIMARY KEY,
    date DATE NOT NULL DEFAULT CURRENT_DATE,
    security_score INTEGER NOT NULL,
    compliance_score INTEGER NOT NULL,
    pending_issues INTEGER NOT NULL,
    fixed_issues INTEGER NOT NULL,
    notes TEXT NULL
);

CREATE TABLE certik_validators (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    address VARCHAR(100) NOT NULL UNIQUE,
    type VARCHAR(50) NOT NULL,
    active BOOLEAN DEFAULT TRUE
);

-- Foreign key relationships
ALTER TABLE certik_findings
ADD CONSTRAINT fk_component
FOREIGN KEY (component) REFERENCES certik_components(name);

-- Indexes
CREATE INDEX idx_findings_component ON certik_findings(component);
CREATE INDEX idx_findings_severity ON certik_findings(severity);
CREATE INDEX idx_findings_status ON certik_findings(status);
CREATE INDEX idx_findings_bip ON certik_findings(bip);
