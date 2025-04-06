;; Authorization Trait
;; Defines the standard interface for authorization checks

(define-trait authorization-trait
    (
        ;; Check if a principal has a specific permission
        (has-permission (principal (string-ascii 50)) (response bool uint))
        
        ;; Check if a principal is authorized for a specific action
        (is-authorized (principal (string-ascii 100)) (response bool uint))
        
        ;; Grant a permission to a principal
        (grant-permission (principal (string-ascii 50)) (response bool uint))
        
        ;; Revoke a permission from a principal
        (revoke-permission (principal (string-ascii 50)) (response bool uint))
        
        ;; Check if a contract call is authorized
        (validate-contract-call 
            ({
                contract: principal,
                function: (string-ascii 100),
                args: (list 10 {value: uint, data: (optional buff 100)})
            })
            (response {is-valid: bool, from-cache: bool} uint)
        )
    )
)