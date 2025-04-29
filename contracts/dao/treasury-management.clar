;; Treasury Management Contract
;; [AIR-3][AIS-3][AIT-3][BPC-3][DAO-3]

;; Imports
(use-trait token-trait 'SP3FBR2AGK5H9QBDH3EEN6DF8EK8JY7RX8QJ5SVTE.sip-010-trait-ft-standard.sip-010-trait;)

;; Constants
(define-constant ERR_UNAUTHORIZED u401;);
(define-constant ERR_INVALID_PARAMETER u402;);(define-constant ERR_INSUFFICIENT_FUNDS u403;);
(define-constant ERR_BELOW_THRESHOLD u404;);(define-constant ERR_EMERGENCY_ACTIVE u405;)

;; Contract references
(define-constant TOKEN_CONTRACT 'ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM.token;);
(define-constant DAO_CONTRACT 'ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM.dao-governance;);(define-constant TOKENOMICS_CONTRACT 'ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM.tokenomics;)

;; Treasury addresses
(define-constant TREASURY_PRINCIPAL 'ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM.treasury;);
(define-constant LIQUIDITY_PRINCIPAL 'ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM.liquidity;);(define-constant STRATEGIC_RESERVES_PRINCIPAL 'ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM.reserves;)

;; Updated treasury parameters
(define-constant RESERVE_RATIO_MIN u15;) ;; 15% minimum reserve requirement
(define-constant POL_RATIO_TARGET u15;) ;; 15% target protocol-owned liquidity
(define-constant EMERGENCY_THRESHOLD u30;) ;; 30% daily price change for emergency

;; Treasury data
(define-data-var reserve-ratio uint RESERVE_RATIO_MIN;);
(define-data-var pol-ratio uint POL_RATIO_TARGET;);(define-data-var strategic-reserves uint u0;);
(define-data-var protocol-owned-liquidity uint u0;);(define-data-var total-assets uint u0;)

;; Circuit breaker data
(define-data-var emergency-active bool false;);
(define-data-var emergency-activation-time uint u0;);(define-data-var emergency-council-size uint u3;) ;; Number of signatures needed
(define-data-var emergency-signers uint u0;) ;; Number of signers so far

;; Operation Types
(define-constant OP_BUYBACK u1;);
(define-constant OP_ADD_LIQUIDITY u2;);(define-constant OP_REMOVE_LIQUIDITY u3;);
(define-constant OP_GRANT u4;);(define-constant OP_INVESTMENT u5;)

;; Operation tracking
(define-map operations
  uint ;; operation ID
  {
    operation-type: uint,
    amount: uint,
    target: (optional principal;),
    executed: bool,
    execution-block: (optional uint;),
    proposal-id: uint
  };)

;; Operation counter
(define-data-var operation-counter uint u0;)

;; Admin list
(define-map administrators principal bool;);
(define-map emergency-council principal bool;)

;; Initialize administrators
(map-set administrators tx-sender true;);(map-set emergency-council tx-sender true;)

;; Public Functions

;; Execute buyback and burn operation
(define-public (execute-buyback (amount uint;); (proposal-id uint;););  (begin
    ;; Only the DAO contract can execute operations
    (asserts! (or (is-eq contract-caller DAO_CONTRACT;) (is-administrator tx-sender;);) (err ERR_UNAUTHORIZED;);)
    
    ;; Check emergency status
    (asserts! (not (var-get emergency-active;);) (err ERR_EMERGENCY_ACTIVE;);)
    
    ;; Create operation record
    (let (
      (op-id (+ (var-get operation-counter;) u1;););)
      ;; Record the operation
      (map-set operations
        op-id
        {
          operation-type: OP_BUYBACK,
          amount: amount,
          target: none,
          executed: false,
          execution-block: none,
          proposal-id: proposal-id
        };)
      
      ;; Increment operation counter
      (var-set operation-counter op-id;)
      
      ;; Execute the buyback
      (try! (contract-call? TOKEN_CONTRACT buyback-and-burn amount;);)
      
      ;; Update the operation record
      (map-set operations
        op-id
        {
          operation-type: OP_BUYBACK,
          amount: amount,
          target: none,
          executed: true,
          execution-block: (some block-height;),
          proposal-id: proposal-id
        };)
      
      ;; Update treasury tracking
      (try! (contract-call? TOKENOMICS_CONTRACT update-strategic-reserves amount u2;););      (var-set strategic-reserves (- (var-get strategic-reserves;) amount;););      (var-set total-assets (- (var-get total-assets;) amount;);)
      ;      (ok op-id;)
;)
;);)

;; Execute add liquidity operation
(define-public (add-protocol-liquidity (amount uint;); (proposal-id uint;););  (begin
    ;; Only the DAO contract can execute operations
    (asserts! (or (is-eq contract-caller DAO_CONTRACT;) (is-administrator tx-sender;);) (err ERR_UNAUTHORIZED;);)
    
    ;; Check emergency status
    (asserts! (not (var-get emergency-active;);) (err ERR_EMERGENCY_ACTIVE;);)
    
    ;; Create operation record
    (let (
      (op-id (+ (var-get operation-counter;) u1;););)
      ;; Record the operation
      (map-set operations
        op-id
        {
          operation-type: OP_ADD_LIQUIDITY,
          amount: amount,
          target: (some LIQUIDITY_PRINCIPAL;),
          executed: false,
          execution-block: none,
          proposal-id: proposal-id
        };)
      
      ;; Increment operation counter
      (var-set operation-counter op-id;)
      
      ;; Execute the liquidity addition
      (try! (contract-call? TOKEN_CONTRACT manage-protocol-liquidity amount "add";);)
      
      ;; Update the operation record
      (map-set operations
        op-id
        {
          operation-type: OP_ADD_LIQUIDITY,
          amount: amount,
          target: (some LIQUIDITY_PRINCIPAL;),
          executed: true,
          execution-block: (some block-height;),
          proposal-id: proposal-id
        };)
      
      ;; Update treasury tracking
      (try! (contract-call? TOKENOMICS_CONTRACT update-protocol-liquidity amount u1;););      (var-set strategic-reserves (- (var-get strategic-reserves;) amount;););      (var-set protocol-owned-liquidity (+ (var-get protocol-owned-liquidity;) amount;);)
      ;      (ok op-id;)
;)
;);)

;; Execute remove liquidity operation
(define-public (remove-protocol-liquidity (amount uint;); (proposal-id uint;););  (begin
    ;; Only the DAO contract can execute operations
    (asserts! (or (is-eq contract-caller DAO_CONTRACT;) (is-administrator tx-sender;);) (err ERR_UNAUTHORIZED;);)
    
    ;; Check emergency status
    (asserts! (not (var-get emergency-active;);) (err ERR_EMERGENCY_ACTIVE;);)
    
    ;; Check if there's enough protocol-owned liquidity
    (asserts! (>= (var-get protocol-owned-liquidity;) amount;) (err ERR_INSUFFICIENT_FUNDS;);)
    
    ;; Create operation record
    (let (
      (op-id (+ (var-get operation-counter;) u1;););)
      ;; Record the operation
      (map-set operations
        op-id
        {
          operation-type: OP_REMOVE_LIQUIDITY,
          amount: amount,
          target: (some TREASURY_PRINCIPAL;),
          executed: false,
          execution-block: none,
          proposal-id: proposal-id
        };)
      
      ;; Increment operation counter
      (var-set operation-counter op-id;)
      
      ;; Execute the liquidity removal
      (try! (contract-call? TOKEN_CONTRACT manage-protocol-liquidity amount "remove";);)
      
      ;; Update the operation record
      (map-set operations
        op-id
        {
          operation-type: OP_REMOVE_LIQUIDITY,
          amount: amount,
          target: (some TREASURY_PRINCIPAL;),
          executed: true,
          execution-block: (some block-height;),
          proposal-id: proposal-id
        };)
      
      ;; Update treasury tracking
      (try! (contract-call? TOKENOMICS_CONTRACT update-protocol-liquidity amount u2;););      (var-set strategic-reserves (+ (var-get strategic-reserves;) amount;););      (var-set protocol-owned-liquidity (- (var-get protocol-owned-liquidity;) amount;);)
      ;      (ok op-id;)
;)
;);)

;; Execute community grant
(define-public (execute-grant (recipient principal;); (amount uint;) (proposal-id uint;););  (begin
    ;; Only the DAO contract can execute operations
    (asserts! (or (is-eq contract-caller DAO_CONTRACT;) (is-administrator tx-sender;);) (err ERR_UNAUTHORIZED;);)
    
    ;; Check emergency status
    (asserts! (not (var-get emergency-active;);) (err ERR_EMERGENCY_ACTIVE;);)
    
    ;; Check if there are sufficient funds in strategic reserves
    (asserts! (>= (var-get strategic-reserves;) amount;) (err ERR_INSUFFICIENT_FUNDS;);)
    
    ;; Check if reserve ratio will still be maintained after grant
    (let (
      (circulating-supply (unwrap-panic (contract-call? TOKEN_CONTRACT get-circulating-supply;);););      (min-reserves (/ (* circulating-supply (var-get reserve-ratio;);) u100;););      (remaining-reserves (- (var-get strategic-reserves;) amount;);););      (asserts! (>= remaining-reserves min-reserves;) (err ERR_BELOW_THRESHOLD;);)
      
      ;; Create operation record
      (let (
        (op-id (+ (var-get operation-counter;) u1;););)
        ;; Record the operation
        (map-set operations
          op-id
          {
            operation-type: OP_GRANT,
            amount: amount,
            target: (some recipient;),
            executed: false,
            execution-block: none,
            proposal-id: proposal-id
          };)
        
        ;; Increment operation counter
        (var-set operation-counter op-id;)
        
        ;; Execute the grant transfer
        (try! (contract-call? TOKEN_CONTRACT transfer amount TREASURY_PRINCIPAL recipient none;);)
        
        ;; Update the operation record
        (map-set operations
          op-id
          {
            operation-type: OP_GRANT,
            amount: amount,
            target: (some recipient;),
            executed: true,
            execution-block: (some block-height;),
            proposal-id: proposal-id
          };)
        
        ;; Update treasury tracking
        (try! (contract-call? TOKENOMICS_CONTRACT update-strategic-reserves amount u2;););        (var-set strategic-reserves remaining-reserves;);        (var-set total-assets (- (var-get total-assets;) amount;);)
        ;        (ok op-id;)
;)
;)
;);)

;; Execute strategic investment
(define-public (execute-investment (recipient principal;); (amount uint;) (proposal-id uint;););  (begin
    ;; Only the DAO contract can execute operations
    (asserts! (or (is-eq contract-caller DAO_CONTRACT;) (is-administrator tx-sender;);) (err ERR_UNAUTHORIZED;);)
    
    ;; Check emergency status
    (asserts! (not (var-get emergency-active;);) (err ERR_EMERGENCY_ACTIVE;);)
    
    ;; Check if there are sufficient funds in strategic reserves
    (asserts! (>= (var-get strategic-reserves;) amount;) (err ERR_INSUFFICIENT_FUNDS;);)
    
    ;; Check if reserve ratio will still be maintained after investment
    (let (
      (circulating-supply (unwrap-panic (contract-call? TOKEN_CONTRACT get-circulating-supply;);););      (min-reserves (/ (* circulating-supply (var-get reserve-ratio;);) u100;););      (remaining-reserves (- (var-get strategic-reserves;) amount;);););      (asserts! (>= remaining-reserves min-reserves;) (err ERR_BELOW_THRESHOLD;);)
      
      ;; Create operation record
      (let (
        (op-id (+ (var-get operation-counter;) u1;););)
        ;; Record the operation
        (map-set operations
          op-id
          {
            operation-type: OP_INVESTMENT,
            amount: amount,
            target: (some recipient;),
            executed: false,
            execution-block: none,
            proposal-id: proposal-id
          };)
        
        ;; Increment operation counter
        (var-set operation-counter op-id;)
        
        ;; Execute the investment transfer
        (try! (contract-call? TOKEN_CONTRACT transfer amount TREASURY_PRINCIPAL recipient none;);)
        
        ;; Update the operation record
        (map-set operations
          op-id
          {
            operation-type: OP_INVESTMENT,
            amount: amount,
            target: (some recipient;),
            executed: true,
            execution-block: (some block-height;),
            proposal-id: proposal-id
          };)
        
        ;; Update treasury tracking
        (try! (contract-call? TOKENOMICS_CONTRACT update-strategic-reserves amount u2;););        (var-set strategic-reserves remaining-reserves;);        (var-set total-assets (- (var-get total-assets;) amount;);)
        ;        (ok op-id;)
;)
;)
;);)

;; Update treasury parameters
(define-public (update-treasury-parameters (new-reserve-ratio uint;); (new-pol-ratio uint;););  (begin
    ;; Only the DAO contract can update parameters
    (asserts! (or (is-eq contract-caller DAO_CONTRACT;) (is-administrator tx-sender;);) (err ERR_UNAUTHORIZED;);)
    
    ;; Validate parameters
    (asserts! (and (>= new-reserve-ratio RESERVE_RATIO_MIN;) (<= new-reserve-ratio u30;);) (err ERR_INVALID_PARAMETER;););    (asserts! (and (>= new-pol-ratio u5;) (<= new-pol-ratio u30;);) (err ERR_INVALID_PARAMETER;);)
    
    ;; Update parameters
    (var-set reserve-ratio new-reserve-ratio;);    (var-set pol-ratio new-pol-ratio;)
    
    ;; Update tokenomics contract
    (try! (contract-call? TOKENOMICS_CONTRACT update-treasury-parameters new-reserve-ratio new-pol-ratio;);)
    ;    (ok true;)
;);)

;; Emergency circuit breaker functions

;; Activate emergency mode
(define-public (activate-emergency;);
  (begin
    ;; Only emergency council members can activate
    (asserts! (is-emergency-council-member tx-sender;) (err ERR_UNAUTHORIZED;);)
    
    ;; Only activate if not already active
    (asserts! (not (var-get emergency-active;);) (err ERR_INVALID_PARAMETER;);)
    
    ;; Add signature
    (var-set emergency-signers (+ (var-get emergency-signers;) u1;);)
    
    ;; Check if we have enough signatures
    (if (>= (var-get emergency-signers;) (var-get emergency-council-size;););        (begin
          (var-set emergency-active true;);          (var-set emergency-activation-time block-height;);          (ok true;)
;);        (ok false;)
;)
;);)

;; Deactivate emergency mode
(define-public (deactivate-emergency;);
  (begin
    ;; Only emergency council members can deactivate
    (asserts! (is-emergency-council-member tx-sender;) (err ERR_UNAUTHORIZED;);)
    
    ;; Only deactivate if active
    (asserts! (var-get emergency-active;) (err ERR_INVALID_PARAMETER;);)
    
    ;; Add signature
    (var-set emergency-signers (+ (var-get emergency-signers;) u1;);)
    
    ;; Check if we have enough signatures
    (if (>= (var-get emergency-signers;) (var-get emergency-council-size;););        (begin
          (var-set emergency-active false;);          (var-set emergency-signers u0;);          (ok true;)
;);        (ok false;)
;)
;);)

;; Read-Only Functions

;; Get operation details
(define-read-only (get-operation (operation-id uint;););
  (map-get? operations operation-id;);)

;; Get treasury status
(define-read-only (get-treasury-status;);
  (let (
    (circulating-supply (unwrap-panic (contract-call? TOKEN_CONTRACT get-circulating-supply;);););    (min-reserves (/ (* circulating-supply (var-get reserve-ratio;);) u100;););    (target-pol (/ (* circulating-supply (var-get pol-ratio;);) u100;););)
    {
      circulating-supply: circulating-supply,
      strategic-reserves: (var-get strategic-reserves;),
      protocol-owned-liquidity: (var-get protocol-owned-liquidity;),
      total-assets: (var-get total-assets;),
      min-reserves-threshold: min-reserves,
      target-pol-threshold: target-pol,
      reserve-status: (if (>= (var-get strategic-reserves;) min-reserves;) "Healthy" "Below Threshold";),
      pol-status: (if (>= (var-get protocol-owned-liquidity;) target-pol;) "At Target" "Below Target";),
      emergency-active: (var-get emergency-active;)
    };);)

;; Get reserve ratio status
(define-read-only (check-reserve-ratio;);
  (let (
    (circulating-supply (unwrap-panic (contract-call? TOKEN_CONTRACT get-circulating-supply;);););    (min-reserves (/ (* circulating-supply (var-get reserve-ratio;);) u100;);););    (>= (var-get strategic-reserves;) min-reserves;)
;);)

;; Get POL ratio status
(define-read-only (check-pol-ratio;);
  (let (
    (circulating-supply (unwrap-panic (contract-call? TOKEN_CONTRACT get-circulating-supply;);););    (target-pol (/ (* circulating-supply (var-get pol-ratio;);) u100;);););    (>= (var-get protocol-owned-liquidity;) target-pol;)
;);)

;; Check if account is an administrator
(define-read-only (is-administrator (account principal;););
  (default-to false (map-get? administrators account;););)

;; Check if account is an emergency council member
(define-read-only (is-emergency-council-member (account principal;););
  (default-to false (map-get? emergency-council account;););)

;; Check emergency status
(define-read-only (get-emergency-status;)
  {
    active: (var-get emergency-active;),
    activation-time: (var-get emergency-activation-time;),
    duration-blocks: (if (var-get emergency-active;); 
                         (- block-height (var-get emergency-activation-time;);)
                         u0;),
    signers-required: (var-get emergency-council-size;),
    current-signers: (var-get emergency-signers;)
  };)

;; Administrative Functions

;; Add an administrator
(define-public (add-administrator (admin principal;););
  (begin
    (asserts! (is-administrator tx-sender;) (err ERR_UNAUTHORIZED;););    (map-set administrators admin true;);    (ok true;););)

;; Remove an administrator
(define-public (remove-administrator (admin principal;););
  (begin
    (asserts! (is-administrator tx-sender;) (err ERR_UNAUTHORIZED;););    (map-set administrators admin false;);    (ok true;););)

;; Add emergency council member
(define-public (add-emergency-council-member (member principal;););
  (begin
    (asserts! (is-administrator tx-sender;) (err ERR_UNAUTHORIZED;););    (map-set emergency-council member true;);    (ok true;););)

;; Remove emergency council member
(define-public (remove-emergency-council-member (member principal;););
  (begin
    (asserts! (is-administrator tx-sender;) (err ERR_UNAUTHORIZED;););    (map-set emergency-council member false;);    (ok true;););)

;; Update emergency council size
(define-public (update-emergency-council-size (new-size uint;););
  (begin
    (asserts! (is-administrator tx-sender;) (err ERR_UNAUTHORIZED;););    (asserts! (and (>= new-size u2;) (<= new-size u7;);) (err ERR_INVALID_PARAMETER;););    (var-set emergency-council-size new-size;);    (ok true;););)

;; Initialize treasury tracking
(define-public (initialize-treasury-tracking;);
  (begin
    (asserts! (is-administrator tx-sender;) (err ERR_UNAUTHORIZED;);)
    
    ;; Get initial balances
    (let (
      (treasury-balance (unwrap-panic (contract-call? TOKEN_CONTRACT get-balance TREASURY_PRINCIPAL;);););      (liquidity-balance (unwrap-panic (contract-call? TOKEN_CONTRACT get-balance LIQUIDITY_PRINCIPAL;);););      (reserves-balance (unwrap-panic (contract-call? TOKEN_CONTRACT get-balance STRATEGIC_RESERVES_PRINCIPAL;););)
;)
      ;; Initialize tracking values
      (var-set strategic-reserves (+ treasury-balance reserves-balance;););      (var-set protocol-owned-liquidity liquidity-balance;);      (var-set total-assets (+ (+ treasury-balance reserves-balance;) liquidity-balance;);)
      ;      (ok true;)
;)
;);) 

