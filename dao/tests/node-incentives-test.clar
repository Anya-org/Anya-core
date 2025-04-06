;; Node Incentivization Tests
(use-trait auth-trait .authorization-trait.authorization-trait)
(use-trait ft-trait .sip-010-trait.sip-010-trait)

;; Test setup
(define-constant TEST_ADMIN 'ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM)
(define-constant TEST_NODE 'ST2CY5V39NHDPWSXMW9QDT3HC3GD6Q6XX4CFRK9AG)
(define-constant TEST_TOKEN 'ST2JHG361ZXG51QTKY2NQCVBPPRRE2KZB1HR05NNC.governance-token)

;; Test Cases

;; Reward Pool Tests
(define-public (test-create-reward-pool)
    (begin
        ;; Setup: Grant reward management permissions
        (try! (contract-call? .authorization grant-permission TEST_ADMIN "manage-rewards"))
        
        ;; Test: Create new reward pool
        (let
            (
                (result (as-contract TEST_ADMIN
                    (contract-call? .node-incentives create-reward-pool
                        u10000
                        u1000
                        TEST_TOKEN)))
            )
            (asserts! (is-ok result) (err "Failed to create reward pool"))
            
            ;; Verify pool was created
            (let
                (
                    (stored-pool (contract-call? .node-incentives get-pool-info
                        (unwrap-panic result)))
                )
                (asserts! (is-ok stored-pool) (err "Failed to retrieve pool"))
                (asserts! (is-eq (get total-amount (unwrap-panic stored-pool)) u10000)
                    (err "Invalid pool amount"))
                (asserts! (is-eq (get reward-token (unwrap-panic stored-pool)) TEST_TOKEN)
                    (err "Invalid reward token"))
            )
            (ok true)
        )
    )
)

;; Node Performance Tests
(define-public (test-update-node-performance)
    (begin
        ;; Setup: Grant performance management permissions
        (try! (contract-call? .authorization grant-permission TEST_ADMIN "manage-performance"))
        
        ;; Test: Update node performance
        (let
            (
                (result (as-contract TEST_ADMIN
                    (contract-call? .node-incentives update-node-performance
                        TEST_NODE
                        u90)))
            )
            (asserts! (is-ok result) (err "Failed to update performance"))
            
            ;; Verify performance was updated
            (let
                (
                    (node-stats (contract-call? .node-incentives get-node-stats TEST_NODE))
                )
                (asserts! (is-ok node-stats) (err "Failed to retrieve node stats"))
                (asserts! (is-eq (get performance-score (unwrap-panic node-stats)) u90)
                    (err "Invalid performance score"))
                (asserts! (is-eq (get penalty-rate (unwrap-panic node-stats)) u0)
                    (err "Should not have penalties"))
            )
            (ok true)
        )
    )
)

;; Reward Distribution Tests
(define-public (test-distribute-rewards)
    (begin
        ;; Setup: Create pool and set performance
        (try! (test-create-reward-pool))
        (try! (test-update-node-performance))
        
        ;; Test: Distribute rewards
        (let
            (
                (result (as-contract TEST_ADMIN
                    (contract-call? .node-incentives distribute-rewards u1)))
            )
            (asserts! (is-ok result) (err "Failed to distribute rewards"))
            
            ;; Verify distribution
            (let
                (
                    (distribution (contract-call? .node-incentives get-distribution
                        (var-get .node-incentives current-cycle)
                        TEST_NODE))
                )
                (asserts! (is-ok distribution) (err "Failed to retrieve distribution"))
                (asserts! (> (get amount (unwrap-panic distribution)) u0)
                    (err "No rewards distributed"))
            )
            (ok true)
        )
    )
)

;; Reward Claiming Tests
(define-public (test-claim-rewards)
    (begin
        ;; Setup: Distribute rewards first
        (try! (test-distribute-rewards))
        
        ;; Test: Claim rewards
        (let
            (
                (result (as-contract TEST_NODE
                    (contract-call? .node-incentives claim-rewards
                        u1
                        (var-get .node-incentives current-cycle))))
            )
            (asserts! (is-ok result) (err "Failed to claim rewards"))
            
            ;; Verify claim
            (let
                (
                    (distribution (contract-call? .node-incentives get-distribution
                        (var-get .node-incentives current-cycle)
                        TEST_NODE))
                )
                (asserts! (is-ok distribution) (err "Failed to retrieve distribution"))
                (asserts! (is-eq (get amount (unwrap-panic distribution)) u0)
                    (err "Rewards not cleared after claim"))
            )
            (ok true)
        )
    )
)

;; Penalty Tests
(define-public (test-penalty-application)
    (begin
        ;; Setup: Set poor performance multiple times
        (try! (as-contract TEST_ADMIN
            (contract-call? .node-incentives update-node-performance
                TEST_NODE
                u70)))
        (try! (as-contract TEST_ADMIN
            (contract-call? .node-incentives update-node-performance
                TEST_NODE
                u65)))
        
        ;; Verify penalties were applied
        (let
            (
                (node-stats (contract-call? .node-incentives get-node-stats TEST_NODE))
            )
            (asserts! (is-ok node-stats) (err "Failed to retrieve node stats"))
            (asserts! (> (get penalty-rate (unwrap-panic node-stats)) u0)
                (err "No penalties applied"))
            (asserts! (> (get consecutive-penalties (unwrap-panic node-stats)) u0)
                (err "Consecutive penalties not tracked"))
            (ok true)
        )
    )
)

;; Edge Cases and Error Tests
(define-public (test-error-cases)
    (begin
        ;; Test: Create pool without permission
        (let
            (
                (result (contract-call? .node-incentives create-reward-pool
                    u1
                    u1
                    TEST_TOKEN))
            )
            (asserts! (is-err result) (err "Should fail without permission"))
            
            ;; Test: Claim non-existent rewards
            (let
                (
                    (result (contract-call? .node-incentives claim-rewards
                        u999
                        u999))
                )
                (asserts! (is-err result) (err "Should fail for non-existent rewards"))
                
                ;; Test: Update performance for non-existent node
                (let
                    (
                        (result (as-contract TEST_ADMIN
                            (contract-call? .node-incentives update-node-performance
                                'ST3NBRSFKX28FQ2ZJ1MAKX58HKHSDGNV5N7R21XCP
                                u100)))
                    )
                    (asserts! (is-ok result) (err "Should create new node entry"))
                    (ok true)
                )
            )
        )
    )
)

;; Run all tests
(define-public (run-all-tests)
    (begin
        (try! (test-create-reward-pool))
        (try! (test-update-node-performance))
        (try! (test-distribute-rewards))
        (try! (test-claim-rewards))
        (try! (test-penalty-application))
        (try! (test-error-cases))
        (ok true)
    )
)