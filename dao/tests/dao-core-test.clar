;; DAO Core Test Script
;; This script tests the enhanced dao-core.clar implementation

;; Define constants for test accounts
(define-constant wallet-1 'ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM)
(define-constant wallet-2 'ST2PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM)
(define-constant wallet-3 'ST3PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM)

;; Define paths for the contracts
(define-constant dao-core-path '../core/dao-core.clar)
(define-constant token-path '../../src/contracts/governance_token.clar)

;; Import the contracts
(contract-call? .dao-core get-dao-name)

;; =========================================
;; Test Suite: Admin Functions
;; =========================================

;; Test 1: Administrator Management
(print "Test 1: Administrator Management")
(print "---------------------------------")

;; Initial admin check (contract owner should be admin)
(let ((is-admin-1 (contract-call? .dao-core is-admin wallet-1)))
    (print "Owner is admin: ")
    (print is-admin-1)
    (asserts! is-admin-1 (err u1))
)

;; Initial admin check for wallet-2 (should not be admin)
(let ((is-admin-2 (contract-call? .dao-core is-admin wallet-2)))
    (print "Wallet 2 is admin (before): ")
    (print is-admin-2)
    (asserts! (not is-admin-2) (err u2))
)

;; Add wallet-2 as admin
(as-contract 
    (print "Adding wallet-2 as admin: ")
    (print (contract-call? .dao-core add-administrator wallet-2))
)

;; Check admin status after adding
(let ((is-admin-2-after (contract-call? .dao-core is-admin wallet-2)))
    (print "Wallet 2 is admin (after): ")
    (print is-admin-2-after)
    (asserts! is-admin-2-after (err u3))
)

;; Try to remove wallet-1 (should fail as it's the contract owner)
(as-contract
    (print "Attempt to remove owner as admin: ")
    (print (contract-call? .dao-core remove-administrator wallet-1))
)

;; Remove wallet-2 as admin
(as-contract
    (print "Removing wallet-2 as admin: ")
    (print (contract-call? .dao-core remove-administrator wallet-2))
)

;; Check admin status after removing
(let ((is-admin-2-after-removal (contract-call? .dao-core is-admin wallet-2)))
    (print "Wallet 2 is admin (after removal): ")
    (print is-admin-2-after-removal)
    (asserts! (not is-admin-2-after-removal) (err u4))
)

;; =========================================
;; Test Suite: DAO Settings
;; =========================================

;; Test 2: DAO Settings Management
(print "\nTest 2: DAO Settings Management")
(print "--------------------------------")

;; Check initial DAO name
(let ((initial-name (contract-call? .dao-core get-dao-name)))
    (print "Initial DAO name: ")
    (print initial-name)
)

;; Update DAO name
(as-contract
    (print "Updating DAO name: ")
    (print (contract-call? .dao-core update-dao-name "New Anya DAO Name"))
)

;; Check updated DAO name
(let ((updated-name (contract-call? .dao-core get-dao-name)))
    (print "Updated DAO name: ")
    (print updated-name)
)

;; Check initial DAO settings
(let ((settings (contract-call? .dao-core get-dao-settings)))
    (print "Initial DAO settings: ")
    (print settings)
)

;; Update proposal threshold
(as-contract
    (print "Updating proposal threshold: ")
    (print (contract-call? .dao-core update-proposal-threshold u200))
)

;; Check updated DAO settings
(let ((updated-settings (contract-call? .dao-core get-dao-settings)))
    (print "Updated DAO settings: ")
    (print updated-settings)
)

;; =========================================
;; Test Suite: Proposal Functions
;; =========================================

;; Test 3: Proposal Creation and Validation
(print "\nTest 3: Proposal Creation and Validation")
(print "----------------------------------------")

;; Mock token balance to pass validation (in real testing, this would be done by interacting with the token contract)
;; For our test, we'll mock this by updating the proposal threshold to 0 temporarily
(as-contract
    (print "Setting proposal threshold to 0 for testing: ")
    (print (contract-call? .dao-core update-proposal-threshold u0))
)

;; Create a valid proposal
(let ((result (as-contract (contract-call? .dao-core submit-proposal "Test Proposal" "This is a test proposal description" u1500))))
    (print "Create valid proposal result: ")
    (print result)
)

;; Attempt to create a proposal with too short duration (should fail)
(let ((result (as-contract (try! (contract-call? .dao-core submit-proposal "Short Duration" "This should fail due to short duration" u100)))))
    (print "Create proposal with short duration: ")
    (print result)
)

;; Get proposal count
(let ((count (contract-call? .dao-core get-proposal-count)))
    (print "Proposal count: ")
    (print count)
)

;; Get proposal details
(let ((proposal (contract-call? .dao-core get-proposal u1)))
    (print "Proposal 1 details: ")
    (print proposal)
)

;; Update proposal status
(as-contract
    (print "Updating proposal status: ")
    (print (contract-call? .dao-core update-proposal-status u1 "EXECUTED"))
)

;; Check updated status
(let ((updated-proposal (contract-call? .dao-core get-proposal u1)))
    (print "Updated proposal details: ")
    (print updated-proposal)
)

;; =========================================
;; Test Suite: Logging System
;; =========================================

;; Test 4: Logging System
(print "\nTest 4: Logging System")
(print "---------------------")

;; Get log count
(let ((count (contract-call? .dao-core get-log-count)))
    (print "Log count: ")
    (print count)
)

;; Get recent logs (first 5)
(let ((logs (contract-call? .dao-core get-logs u1 u5)))
    (print "Recent logs: ")
    (print logs)
)

;; =========================================
;; Test Suite: Token Integration
;; =========================================

;; Test 5: Token Integration
(print "\nTest 5: Token Integration")
(print "-----------------------")

;; Check current token contract
(let ((details (contract-call? .dao-core get-dao-details)))
    (print "Current token contract: ")
    (print (get token-contract details))
)

;; Update token contract
(as-contract
    (print "Updating token contract: ")
    (print (contract-call? .dao-core set-token-contract wallet-3))
)

;; Check updated token contract
(let ((updated-details (contract-call? .dao-core get-dao-details)))
    (print "Updated token contract: ")
    (print (get token-contract updated-details))
)

;; In a real test, we would also test mint-tokens by interacting with an actual token contract
;; For this demonstration, we'll note that the contract would need to be deployed
;; and the token contract would need to have proper functions implemented

(print "\nAll tests completed! Verify the output above to confirm functionality.") 