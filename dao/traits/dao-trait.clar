(define-trait dao-trait
    (
        ;; Token Management
        (mint-tokens (uint principal) (response uint uint))
        
        ;; Proposal Management
        (submit-proposal ((string-ascii 256) (string-utf8 4096) uint) (response uint uint))
        
        ;; Queries
        (get-dao-name () (response (string-ascii 256) uint))
        (get-proposal (uint) (response {
            title: (string-ascii 256),
            description: (string-utf8 4096),
            proposer: principal,
            start-block: uint,
            end-block: uint,
            status: (string-ascii 12)
        } uint))
    )
)