#![feature(edition2021)]
let security_gaps = vec![
    SecurityGap {
        repo: "mobile",
        issue: "Missing constant-time comparisons in HSM module",
        severity: SecurityLevel::Critical,
    },
    SecurityGap {
        repo: "enterprise",
        issue: "Incomplete memory isolation for key material",
        severity: SecurityLevel::High,
    },
]; 