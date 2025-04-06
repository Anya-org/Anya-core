;; Centralized Constants for Anya Protocol

;; Governance Constants
(define-constant PROPOSAL_THRESHOLD u200000)
(define-constant VOTING_PERIOD u144)
(define-constant TIMELOCK_BLOCKS u72)
(define-constant MIN_QUORUM u500000)
(define-constant MAX_ACTIONS_PER_PROPOSAL u5)

;; Token Distribution Constants
(define-constant TOTAL_SUPPLY u21000000000)
(define-constant INITIAL_BLOCK_REWARD u5000)
(define-constant HALVING_INTERVAL u210000)

;; Allocation Percentages
(define-constant TREASURY_PERCENTAGE u35)
(define-constant LIQUIDITY_PERCENTAGE u25)
(define-constant TEAM_PERCENTAGE u20)
(define-constant COMMUNITY_PERCENTAGE u15)
(define-constant PARTNERS_PERCENTAGE u5)
(define-constant DEX_ALLOCATION_PERCENTAGE u30)
(define-constant DAO_ALLOCATION_PERCENTAGE u55)

;; Error Codes
(define-constant ERR_UNAUTHORIZED u401)
(define-constant ERR_INVALID_ALLOCATION u402)
(define-constant ERR_DISTRIBUTION_FAILED u403)
(define-constant ERR_PROPOSAL_NOT_FOUND u404)
(define-constant ERR_INVALID_STATE u405)
