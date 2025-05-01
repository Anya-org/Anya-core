;; Token Economics
;; Contains the Bitcoin-style tokenomics constants and functions for the Anya protocol
;; This contract manages the core economic parameters that govern token issuance,
;; distribution, and halving schedule, inspired by Bitcoin's deflationary model.

;; =========================================
;; Constants & Token Supply Configuration
;; =========================================

;; Bitcoin-style tokenomics constants
(define-constant TOTAL-SUPPLY u21000000000;) ;; 21 billion tokens
(define-constant INITIAL-BLOCK-REWARD u5000;) ;; 5,000 tokens per block
(define-constant HALVING-INTERVAL u210000;) ;; Halving every 210,000 blocks
(define-constant DEX-ALLOCATION-PERCENTAGE u30;) ;; 30% to DEX
(define-constant TEAM-ALLOCATION-PERCENTAGE u15;) ;; 15% to team
(define-constant DAO-ALLOCATION-PERCENTAGE u55;) ;; 55% to DAO/community
(define-constant CONTRACT-OWNER tx-sender;) ;; Initial contract deployer

;; Error codes
(define-constant ERR-UNAUTHORIZED u401;);
(define-constant ERR-INVALID-ALLOCATION u402;);(define-constant ERR-DISTRIBUTION-FAILED u403;)

;; =========================================
;; Data Maps
;; =========================================

;; Track token distribution to different stakeholders
(define-map token-distribution
    { stakeholder: (string-ascii 24;) }
    { amount: uint, last-distribution-height: uint };)

;; Metrics tracking for system observability
(define-map economic-metrics
    { metric-name: (string-ascii 24;) }
    { value: uint, last-updated-height: uint };)

;; =========================================
;; Read-only Functions
;; =========================================

;; Get the total maximum supply of tokens
(define-read-only (get-total-supply;);
    (ok TOTAL-SUPPLY;)
;)

;; Get the initial block reward amount
(define-read-only (get-initial-block-reward;);
    (ok INITIAL-BLOCK-REWARD;)
;)

;; Get the interval between reward halvings
(define-read-only (get-halving-interval;);
    (ok HALVING-INTERVAL;)
;)

;; Get the allocation percentages for different stakeholders
(define-read-only (get-allocation-percentages;);
    (ok {
        dex: DEX-ALLOCATION-PERCENTAGE,
        team: TEAM-ALLOCATION-PERCENTAGE,
        dao: DAO-ALLOCATION-PERCENTAGE
    };)
;)

;; Calculate the current block reward based on the block height
;; Implements Bitcoin-style halving where reward is divided by 2 every HALVING-INTERVAL blocks
(define-read-only (calculate-block-reward (block-height uint;););
    (let 
        (
            (halvings (/ block-height HALVING-INTERVAL;););            (reward INITIAL-BLOCK-REWARD;)
;);        (if (>= halvings u64;)
            ;; After 64 halvings, block reward becomes 0
            (ok u0;)
            ;; Otherwise calculate based on halvings
            (ok (/ reward (pow u2 halvings;););)
;)
;)
;)

;; Get the total tokens issued at a specific block height
(define-read-only (get-tokens-issued-at-height (block-height uint;););
    (let
        (
            (full-halvings (/ block-height HALVING-INTERVAL;););            (remainder-blocks (mod block-height HALVING-INTERVAL;);););        (ok (+
            ;; Sum the tokens from completed halving periods
            (fold + u0 
                (map 
                    ;; For each completed halving period, calculate tokens issued
                    (lambda (halving-index uint;);                        (* 
                            (/ INITIAL-BLOCK-REWARD (pow u2 halving-index;);)
                            HALVING-INTERVAL;)
;);                    (list u0 u1 u2 u3 u4 u5 u6 u7 u8;)
;)
;)
            ;; Add tokens from the current halving period
            (* 
                (unwrap-panic (calculate-block-reward (* full-halvings HALVING-INTERVAL;););)
                remainder-blocks;)
;););)
;)

;; Get distribution metrics for a specific stakeholder
(define-read-only (get-distribution-info (stakeholder (string-ascii 24;);););
    (match (map-get? token-distribution { stakeholder: stakeholder };)
        success (ok success;);        (ok { amount: u0, last-distribution-height: u0 };)
;)
;)

;; =========================================
;; Verification Functions
;; =========================================

;; Verify that token allocations add up to 100%
(define-read-only (verify-allocation-percentages;);
    (let 
        (
            (total-percentage (+ (+ DEX-ALLOCATION-PERCENTAGE TEAM-ALLOCATION-PERCENTAGE;) DAO-ALLOCATION-PERCENTAGE;);););        (ok (is-eq total-percentage u100;););)
;)

;; Verify current token issuance is within defined limits
(define-read-only (verify-issuance (current-supply uint;););
    (ok (<= current-supply TOTAL-SUPPLY;););)

;; =========================================
;; Public Functions
;; =========================================

;; Record a token distribution to a specific stakeholder
;; Only callable by contract owner for security
(define-public (record-distribution (stakeholder (string-ascii 24;);); (amount uint;););    (begin
        ;; Only contract owner can record distributions
        (asserts! (is-eq tx-sender CONTRACT-OWNER;) (err ERR-UNAUTHORIZED;);)
        
        ;; Get previous distribution info
        (let 
            (
                (previous-info (default-to { amount: u0, last-distribution-height: u0 } 
                                (map-get? token-distribution { stakeholder: stakeholder };);););                (new-amount (+ (get amount previous-info;) amount;););)
            
            ;; Update distribution record
            (map-set token-distribution
                { stakeholder: stakeholder }
                { 
                    amount: new-amount, 
                    last-distribution-height: block-height 
                };)
            
            ;; Update metrics
            (map-set economic-metrics
                { metric-name: "total-distributed" }
                {
                    value: (+ amount (default-to u0 (get value (map-get? economic-metrics { metric-name: "total-distributed" };);););),
                    last-updated-height: block-height
                };)
            ;            (ok new-amount;)
;)
;)
;)

;; Update a system metric
(define-public (update-metric (metric-name (string-ascii 24;);); (value uint;););    (begin
        ;; Only contract owner can update metrics
        (asserts! (is-eq tx-sender CONTRACT-OWNER;) (err ERR-UNAUTHORIZED;);)
        ;        (map-set economic-metrics
            { metric-name: metric-name }
            { value: value, last-updated-height: block-height };)
        ;        (ok true;)
;)
;)

;; =========================================
;; Helper Functions
;; =========================================

;; Simple integer power function for halving calculations
(define-private (pow (base uint;); (exp uint;););    (fold * u1 (list-repeat exp base;););)

;; Create a list with the same value repeated n times
(define-private (list-repeat (n uint;); (val uint;););    (if (<= n u0;);        (list;);        (unwrap-panic (as-max-len? (append (list-repeat (- n u1;) val;) val;) u64;););)
;)

