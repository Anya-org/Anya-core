# Enhanced with BIP pattern injection
param(
    [Parameter(Mandatory=$true)]
    [string]$Module,
    [ValidateSet("basic","compliance","stress")]
    [string]$TestLevel = "compliance"
)

$template = @"
;; AUTO-GENERATED TEST TEMPLATE
;; BIP-341/342/174 COMPLIANT
(use-trait dao-trait.{dao-trait})
(use-trait bip-compliance.{bip-341-342-trait})

(define-constant test-admin 'ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM)

;; BIP-341 Signature Validation
(define-test (test-$($Module)-bip-compliance)
    (let (
        (sig (unwrap! (contract-call? bip-compliance generate-test-sig) (err u401)))
        (pubkey (unwrap! (contract-call? bip-compliance get-test-pubkey) (err u402)))
    )
    ;; Test logic
    (asserts! (contract-call? $Module verify-taproot-signature ...))
)
"@

if ($TestLevel -eq "compliance") {
    $template += @"

;; PSBT v2 Compliance Check
(define-test (test-$($Module)-psbt-handling)
    (let (
        (psbt (unwrap! (contract-call? bip-compliance generate-test-psbt-v2) (err u403)))
    )
    (asserts! (is-ok (contract-call? $Module process-psbt psbt)))
)
"@
}

Set-Content -Path "tests/$Module.test.clar" -Value $template 