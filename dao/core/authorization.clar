;; Authorization Module
;; Implements authorization-trait and provides permission management

(impl-trait .authorization-trait.authorization-trait)

;; Constants
(define-constant ADMIN_ROLE "admin")
(define-constant AGENT_ROLE "agent")
(define-constant PROPOSER_ROLE "proposer")

;; Permission Maps
(define-map role-permissions
    { role: (string-ascii 50) }
    { permissions: (list 20 (string-ascii 50)) }
)

(define-map principal-roles
    principal
    { roles: (list 10 (string-ascii 50)) }
)

(define-map permission-cache
    {
        principal: principal,
        permission: (string-ascii 50)
    }
    {
        authorized: bool,
        last-checked: uint
    }
)

;; Initialize admin role permissions
(map-set role-permissions
    { role: ADMIN_ROLE }
    {
        permissions: (list 
            "manage-roles"
            "manage-permissions"
            "manage-agents"
            "manage-proposals"
            "execute-proposals"
            "update-parameters"
        )
    }
)

;; Initialize agent role permissions
(map-set role-permissions
    { role: AGENT_ROLE }
    {
        permissions: (list
            "submit-proposals"
            "vote"
            "execute-actions"
            "participate-governance"
        )
    }
)

;; Public Functions
(define-public (has-permission (caller principal) (permission (string-ascii 50)))
    (let
        (
            (cached (map-get? permission-cache 
                { principal: caller, permission: permission }))
        )
        (match cached
            cache (if (< (- block-height (get last-checked cache)) u144)
                (ok (get authorized cache))
                (check-and-cache-permission caller permission))
            (check-and-cache-permission caller permission))
    )
)

(define-public (is-authorized (caller principal) (action (string-ascii 100)))
    (let
        (
            (required-permission (get-required-permission action))
        )
        (has-permission caller required-permission)
    )
)

(define-public (grant-permission (to principal) (permission (string-ascii 50)))
    (begin
        (asserts! (has-permission tx-sender "manage-permissions") (err ERR_UNAUTHORIZED))
        
        (let
            (
                (current-roles (default-to { roles: (list) } 
                    (map-get? principal-roles to)))
            )
            (map-set principal-roles to
                (merge current-roles
                    {
                        roles: (unwrap! (as-max-len? 
                            (append (get roles current-roles) permission)
                            u10)
                            (err ERR_TOO_MANY_ROLES))
                    }
                )
            )
            (ok true)
        )
    )
)

(define-public (revoke-permission (from principal) (permission (string-ascii 50)))
    (begin
        (asserts! (has-permission tx-sender "manage-permissions") (err ERR_UNAUTHORIZED))
        
        (let
            (
                (current-roles (unwrap! (map-get? principal-roles from)
                    (err ERR_NO_ROLES)))
            )
            (map-set principal-roles from
                {
                    roles: (filter remove-permission 
                        (get roles current-roles))
                }
            )
            (ok true)
        )
    )
)

(define-public (validate-contract-call (action {
        contract: principal,
        function: (string-ascii 100),
        args: (list 10 {value: uint, data: (optional buff 100)})
    }))
    (let
        (
            (required-permission (get-function-permission 
                (get contract action)
                (get function action)))
        )
        (match (has-permission tx-sender required-permission)
            is-authorized (ok {
                is-valid: is-authorized,
                from-cache: false
            })
            (err e) (err e))
    )
)

;; Private Helper Functions
(define-private (check-and-cache-permission (caller principal) (permission (string-ascii 50)))
    (let
        (
            (roles (default-to { roles: (list) } (map-get? principal-roles caller)))
            (has-perm (any check-role-has-permission (get roles roles)))
        )
        (map-set permission-cache
            { principal: caller, permission: permission }
            {
                authorized: has-perm,
                last-checked: block-height
            }
        )
        (ok has-perm)
    )
)

(define-private (check-role-has-permission (role (string-ascii 50)))
    (match (map-get? role-permissions { role: role })
        role-data (contains? role (get permissions role-data))
        false)
)

(define-private (get-required-permission (action (string-ascii 100)))
    (match action
        "manage-roles" "manage-permissions"
        "update-parameters" "manage-system"
        "execute-proposal" "execute-proposals"
        "submit-proposal" "submit-proposals"
        "basic-action")
)

(define-private (get-function-permission (contract principal) (function (string-ascii 100)))
    (match function
        "execute-proposal" "execute-proposals"
        "vote-on-proposal" "vote"
        "update-parameters" "manage-system"
        "basic-action")
)

(define-private (remove-permission (p (string-ascii 50)))
    true
)