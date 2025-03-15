;; Token Vesting Contract
;; [AIR-3][AIS-3][AIT-3][BPC-3][DAO-3]

;; Imports
(use-trait token-trait 'SP3FBR2AGK5H9QBDH3EEN6DF8EK8JY7RX8QJ5SVTE.sip-010-trait-ft-standard.sip-010-trait)

;; Constants
(define-constant ERR_UNAUTHORIZED u401)
(define-constant ERR_INVALID_PARAMETER u402)
(define-constant ERR_NOT_FOUND u404)
(define-constant ERR_ALREADY_CLAIMED u409)

;; Contract references
(define-constant TOKEN_CONTRACT 'ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM.token)
(define-constant TOKENOMICS_CONTRACT 'ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM.tokenomics)

;; Updated distribution percentages and allocations
(define-constant TREASURY_PERCENTAGE u35) ;; 35% Protocol Treasury
(define-constant LIQUIDITY_PERCENTAGE u25) ;; 25% Liquidity Provision
(define-constant TEAM_PERCENTAGE u20) ;; 20% Team & Development
(define-constant COMMUNITY_PERCENTAGE u15) ;; 15% Community Incentives
(define-constant PARTNERS_PERCENTAGE u5) ;; 5% Strategic Partners

;; Vesting types
(define-constant VESTING_TYPE_LINEAR u1)  ;; Linear vesting over time
(define-constant VESTING_TYPE_CLIFF u2)   ;; Cliff then linear vesting
(define-constant VESTING_TYPE_MILESTONE u3) ;; Milestone-based vesting

;; Allocation principals
(define-constant TREASURY_PRINCIPAL 'ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM.treasury)
(define-constant LIQUIDITY_PRINCIPAL 'ST1SJ3DTE5DN7X54YDH5D64R3BCB6A2AG2ZQ8YPD5.liquidity)
(define-constant TEAM_PRINCIPAL 'ST2CY5V39NHDPWSXMW9QDT3HC3GD6Q6XX4CFRK9AG.team)
(define-constant COMMUNITY_PRINCIPAL 'ST2JHG361ZXG51QTKY2NQCVBPPRRE2KZB1HR05NNC.community)
(define-constant PARTNERS_PRINCIPAL 'ST2NEB84ASENDXKYGJPQW86YXQCEFEX2ZQPG87ND.partners)

;; Vesting schedule data
(define-data-var launch-timestamp uint u0)
(define-data-var blocks-per-month uint u4320) ;; ~30 days with 10-min blocks

;; Vesting data for each allocation type
(define-map allocation-vesting
  principal
  {
    vesting-type: uint,
    total-allocation: uint,
    initial-percentage: uint,
    cliff-months: uint,
    vesting-months: uint,
    released-amount: uint,
    last-release-block: uint
  }
)

;; Team member allocations (for 4-year vesting with 1-year cliff)
(define-map team-allocations
  principal
  {
    allocation-percentage: uint,
    total-amount: uint,
    released-amount: uint,
    last-release-block: uint,
    milestone-triggers: (list 10 uint),
    milestone-percentages: (list 10 uint)
  }
)

;; Admin list
(define-map administrators principal bool)

;; Initialize administrators
(map-set administrators tx-sender true)

;; Public Functions

;; Initialize vesting schedules
(define-public (initialize-vesting)
  (begin
    ;; Only admins can initialize
    (asserts! (is-administrator tx-sender) (err ERR_UNAUTHORIZED))
    
    ;; Can only be called once
    (asserts! (is-eq (var-get launch-timestamp) u0) (err ERR_UNAUTHORIZED))
    
    ;; Set launch timestamp
    (var-set launch-timestamp block-height)
    
    ;; Calculate total token supply
    (let (
      (total-supply (unwrap-panic (contract-call? TOKEN_CONTRACT get-total-supply)))
      
      ;; Calculate allocation amounts
      (treasury-amount (/ (* total-supply TREASURY_PERCENTAGE) u100))
      (liquidity-amount (/ (* total-supply LIQUIDITY_PERCENTAGE) u100))
      (team-amount (/ (* total-supply TEAM_PERCENTAGE) u100))
      (community-amount (/ (* total-supply COMMUNITY_PERCENTAGE) u100))
      (partners-amount (/ (* total-supply PARTNERS_PERCENTAGE) u100))
    )
      ;; Initialize Treasury vesting (20% initial, 48 month linear)
      (map-set allocation-vesting
        TREASURY_PRINCIPAL
        {
          vesting-type: VESTING_TYPE_LINEAR,
          total-allocation: treasury-amount,
          initial-percentage: u20,
          cliff-months: u0,
          vesting-months: u48,
          released-amount: (/ (* treasury-amount u20) u100),
          last-release-block: block-height
        }
      )
      
      ;; Initialize Liquidity vesting (50% initial, 18 month linear)
      (map-set allocation-vesting
        LIQUIDITY_PRINCIPAL
        {
          vesting-type: VESTING_TYPE_LINEAR,
          total-allocation: liquidity-amount,
          initial-percentage: u50,
          cliff-months: u0,
          vesting-months: u18,
          released-amount: (/ (* liquidity-amount u50) u100),
          last-release-block: block-height
        }
      )
      
      ;; Initialize Team vesting (0% initial, 12 month cliff, 36 month linear)
      (map-set allocation-vesting
        TEAM_PRINCIPAL
        {
          vesting-type: VESTING_TYPE_CLIFF,
          total-allocation: team-amount,
          initial-percentage: u0,
          cliff-months: u12,
          vesting-months: u36,
          released-amount: u0,
          last-release-block: block-height
        }
      )
      
      ;; Initialize Community vesting (10% initial, 48 month linear)
      (map-set allocation-vesting
        COMMUNITY_PRINCIPAL
        {
          vesting-type: VESTING_TYPE_LINEAR,
          total-allocation: community-amount,
          initial-percentage: u10,
          cliff-months: u0,
          vesting-months: u48,
          released-amount: (/ (* community-amount u10) u100),
          last-release-block: block-height
        }
      )
      
      ;; Initialize Partners vesting (10% initial, 36 month linear)
      (map-set allocation-vesting
        PARTNERS_PRINCIPAL
        {
          vesting-type: VESTING_TYPE_LINEAR,
          total-allocation: partners-amount,
          initial-percentage: u10,
          cliff-months: u0,
          vesting-months: u36,
          released-amount: (/ (* partners-amount u10) u100),
          last-release-block: block-height
        }
      )
      
      (ok true)
    )
  ))

;; Process vesting release for an allocation
(define-public (process-allocation-vesting (allocation-principal principal))
  (begin
    ;; Only admins can process vesting
    (asserts! (is-administrator tx-sender) (err ERR_UNAUTHORIZED))
    
    ;; Ensure vesting is initialized
    (asserts! (> (var-get launch-timestamp) u0) (err ERR_UNAUTHORIZED))
    
    ;; Get allocation vesting data
    (let (
      (vesting-data (unwrap! (map-get? allocation-vesting allocation-principal) (err ERR_NOT_FOUND)))
      (blocks-since-launch (- block-height (var-get launch-timestamp)))
      (months-since-launch (/ blocks-since-launch (var-get blocks-per-month)))
    )
      ;; Calculate vested amount based on vesting type
      (if (is-eq (get vesting-type vesting-data) VESTING_TYPE_CLIFF)
          (process-cliff-vesting allocation-principal vesting-data months-since-launch)
          (process-linear-vesting allocation-principal vesting-data months-since-launch))
    )
  ))

;; Process vesting for all allocations
(define-public (process-all-vestings)
  (begin
    ;; Only admins can process vesting
    (asserts! (is-administrator tx-sender) (err ERR_UNAUTHORIZED))
    
    ;; Process each allocation type
    (try! (process-allocation-vesting TREASURY_PRINCIPAL))
    (try! (process-allocation-vesting LIQUIDITY_PRINCIPAL))
    (try! (process-allocation-vesting TEAM_PRINCIPAL))
    (try! (process-allocation-vesting COMMUNITY_PRINCIPAL))
    (try! (process-allocation-vesting PARTNERS_PRINCIPAL))
    
    ;; Update tokenomics contract
    (try! (contract-call? TOKENOMICS_CONTRACT process-vesting-release))
    
    (ok true)
  ))

;; Initialize team member allocation
(define-public (initialize-team-member (member principal) (allocation-percentage uint))
  (begin
    ;; Only admins can initialize team members
    (asserts! (is-administrator tx-sender) (err ERR_UNAUTHORIZED))
    
    ;; Ensure vesting is initialized
    (asserts! (> (var-get launch-timestamp) u0) (err ERR_UNAUTHORIZED))
    
    ;; Validate percentage (must be between 5% and 40%)
    (asserts! (and (>= allocation-percentage u5) (<= allocation-percentage u40)) (err ERR_INVALID_PARAMETER))
    
    ;; Get team allocation data
    (let (
      (team-data (unwrap! (map-get? allocation-vesting TEAM_PRINCIPAL) (err ERR_NOT_FOUND)))
      (team-total (get total-allocation team-data))
      (member-amount (/ (* team-total allocation-percentage) u100))
    )
      ;; Set team member allocation
      (map-set team-allocations
        member
        {
          allocation-percentage: allocation-percentage,
          total-amount: member-amount,
          released-amount: u0,
          last-release-block: block-height,
          milestone-triggers: (list u12 u24 u36 u48 u0 u0 u0 u0 u0 u0), ;; Months for milestones
          milestone-percentages: (list u25 u25 u25 u25 u0 u0 u0 u0 u0 u0) ;; % per milestone
        }
      )
      
      (ok true)
    )
  ))

;; Process team member vesting
(define-public (process-team-member-vesting (member principal))
  (begin
    ;; Only admins can process vesting
    (asserts! (is-administrator tx-sender) (err ERR_UNAUTHORIZED))
    
    ;; Ensure vesting is initialized
    (asserts! (> (var-get launch-timestamp) u0) (err ERR_UNAUTHORIZED))
    
    ;; Get team member allocation data
    (let (
      (member-data (unwrap! (map-get? team-allocations member) (err ERR_NOT_FOUND)))
      (blocks-since-launch (- block-height (var-get launch-timestamp)))
      (months-since-launch (/ blocks-since-launch (var-get blocks-per-month)))
      (team-data (unwrap! (map-get? allocation-vesting TEAM_PRINCIPAL) (err ERR_NOT_FOUND)))
    )
      ;; Check if cliff period has passed
      (asserts! (>= months-since-launch (get cliff-months team-data)) (err ERR_INVALID_PARAMETER))
      
      ;; Calculate newly vested amount based on milestones
      (let (
        (triggers (get milestone-triggers member-data))
        (percentages (get milestone-percentages member-data))
        (total-amount (get total-amount member-data))
        (already-released (get released-amount member-data))
        (newly-vested u0)
      )
        ;; Check each milestone
        (set-newly-vested months-since-launch triggers percentages total-amount already-released)
      )
    )
  ))

;; Add a milestone for team member
(define-public (add-team-member-milestone (member principal) (trigger-month uint) (percentage uint))
  (begin
    ;; Only admins can add milestones
    (asserts! (is-administrator tx-sender) (err ERR_UNAUTHORIZED))
    
    ;; Validate percentage
    (asserts! (and (> percentage u0) (<= percentage u100)) (err ERR_INVALID_PARAMETER))
    
    ;; Get team member allocation data
    (let (
      (member-data (unwrap! (map-get? team-allocations member) (err ERR_NOT_FOUND)))
      (triggers (get milestone-triggers member-data))
      (percentages (get milestone-percentages member-data))
    )
      ;; Update with new milestone
      (map-set team-allocations
        member
        (merge member-data {
          milestone-triggers: (unwrap! (as-max-len? (append triggers trigger-month) u10) (err ERR_INVALID_PARAMETER)),
          milestone-percentages: (unwrap! (as-max-len? (append percentages percentage) u10) (err ERR_INVALID_PARAMETER))
        })
      )
      
      (ok true)
    )
  ))

;; Read-Only Functions

;; Get allocation vesting data
(define-read-only (get-allocation-vesting (allocation-principal principal))
  (map-get? allocation-vesting allocation-principal))

;; Get team member allocation
(define-read-only (get-team-member-allocation (member principal))
  (map-get? team-allocations member))

;; Calculate vested amount for an allocation
(define-read-only (calculate-vested-amount (allocation-principal principal))
  (let (
    (vesting-data (unwrap! (map-get? allocation-vesting allocation-principal) (err ERR_NOT_FOUND)))
    (blocks-since-launch (- block-height (var-get launch-timestamp)))
    (months-since-launch (/ blocks-since-launch (var-get blocks-per-month)))
    (total-allocation (get total-allocation vesting-data))
    (initial-percentage (get initial-percentage vesting-data))
    (initial-amount (/ (* total-allocation initial-percentage) u100))
    (vesting-amount (- total-allocation initial-amount))
  )
    (if (is-eq (get vesting-type vesting-data) VESTING_TYPE_CLIFF)
        ;; Cliff vesting calculation
        (if (< months-since-launch (get cliff-months vesting-data))
            ;; Before cliff, only initial amount is vested
            (ok {
              total-allocation: total-allocation,
              vested-amount: initial-amount,
              unreleased-amount: (- initial-amount (get released-amount vesting-data)),
              vesting-progress-percentage: initial-percentage
            })
            ;; After cliff, calculate linear vesting
            (let (
              (months-after-cliff (- months-since-launch (get cliff-months vesting-data)))
              (vesting-period (get vesting-months vesting-data))
              (vested-so-far (if (>= months-after-cliff vesting-period)
                               vesting-amount
                               (/ (* vesting-amount months-after-cliff) vesting-period)))
              (total-vested (+ initial-amount vested-so-far))
              (progress-percentage (/ (* total-vested u100) total-allocation))
            )
              (ok {
                total-allocation: total-allocation,
                vested-amount: total-vested,
                unreleased-amount: (- total-vested (get released-amount vesting-data)),
                vesting-progress-percentage: progress-percentage
              })
            )
        )
        ;; Linear vesting calculation
        (let (
          (vested-so-far (if (>= months-since-launch (get vesting-months vesting-data))
                           vesting-amount
                           (/ (* vesting-amount months-since-launch) (get vesting-months vesting-data))))
          (total-vested (+ initial-amount vested-so-far))
          (progress-percentage (/ (* total-vested u100) total-allocation))
        )
          (ok {
            total-allocation: total-allocation,
            vested-amount: total-vested,
            unreleased-amount: (- total-vested (get released-amount vesting-data)),
            vesting-progress-percentage: progress-percentage
          })
        )
    )
  ))

;; Get vesting schedule summary
(define-read-only (get-vesting-summary)
  (ok {
    launch-timestamp: (var-get launch-timestamp),
    current-block: block-height,
    blocks-since-launch: (if (> (var-get launch-timestamp) u0) (- block-height (var-get launch-timestamp)) u0),
    months-since-launch: (if (> (var-get launch-timestamp) u0) 
                              (/ (- block-height (var-get launch-timestamp)) (var-get blocks-per-month))
                              u0),
    treasury-percentage: TREASURY_PERCENTAGE,
    liquidity-percentage: LIQUIDITY_PERCENTAGE,
    team-percentage: TEAM_PERCENTAGE,
    community-percentage: COMMUNITY_PERCENTAGE,
    partners-percentage: PARTNERS_PERCENTAGE
  }))

;; Check if account is an administrator
(define-read-only (is-administrator (account principal))
  (default-to false (map-get? administrators account)))

;; Helper Functions

;; Process cliff-based vesting
(define-private (process-cliff-vesting 
                (allocation-principal principal) 
                (vesting-data {
                  vesting-type: uint,
                  total-allocation: uint,
                  initial-percentage: uint,
                  cliff-months: uint,
                  vesting-months: uint,
                  released-amount: uint,
                  last-release-block: uint
                }) 
                (months-since-launch uint))
  (let (
    (total-allocation (get total-allocation vesting-data))
    (initial-percentage (get initial-percentage vesting-data))
    (initial-amount (/ (* total-allocation initial-percentage) u100))
    (vesting-amount (- total-allocation initial-amount))
    (cliff-months (get cliff-months vesting-data))
    (vesting-months (get vesting-months vesting-data))
    (released-so-far (get released-amount vesting-data))
  )
    ;; Check if cliff period has passed
    (if (< months-since-launch cliff-months)
        ;; Before cliff, nothing new to release
        (ok false)
        ;; After cliff, calculate linear vesting
        (let (
          (months-after-cliff (- months-since-launch cliff-months))
          (vested-so-far (if (>= months-after-cliff vesting-months)
                           vesting-amount
                           (/ (* vesting-amount months-after-cliff) vesting-months)))
          (total-vested (+ initial-amount vested-so-far))
          (to-release (- total-vested released-so-far))
        )
          (if (> to-release u0)
              (begin
                ;; Update allocation vesting data
                (map-set allocation-vesting
                  allocation-principal
                  (merge vesting-data {
                    released-amount: total-vested,
                    last-release-block: block-height
                  })
                )
                
                ;; Transfer tokens to the allocation principal
                (try! (contract-call? TOKEN_CONTRACT transfer to-release TREASURY_PRINCIPAL allocation-principal none))
                
                (ok true)
              )
              (ok false)
          )
        )
    )
  ))

;; Process linear vesting
(define-private (process-linear-vesting 
                (allocation-principal principal) 
                (vesting-data {
                  vesting-type: uint,
                  total-allocation: uint,
                  initial-percentage: uint,
                  cliff-months: uint,
                  vesting-months: uint,
                  released-amount: uint,
                  last-release-block: uint
                }) 
                (months-since-launch uint))
  (let (
    (total-allocation (get total-allocation vesting-data))
    (initial-percentage (get initial-percentage vesting-data))
    (initial-amount (/ (* total-allocation initial-percentage) u100))
    (vesting-amount (- total-allocation initial-amount))
    (vesting-months (get vesting-months vesting-data))
    (released-so-far (get released-amount vesting-data))
    (vested-so-far (if (>= months-since-launch vesting-months)
                      vesting-amount
                      (/ (* vesting-amount months-since-launch) vesting-months)))
    (total-vested (+ initial-amount vested-so-far))
    (to-release (- total-vested released-so-far))
  )
    (if (> to-release u0)
        (begin
          ;; Update allocation vesting data
          (map-set allocation-vesting
            allocation-principal
            (merge vesting-data {
              released-amount: total-vested,
              last-release-block: block-height
            })
          )
          
          ;; Transfer tokens to the allocation principal
          (try! (contract-call? TOKEN_CONTRACT transfer to-release TREASURY_PRINCIPAL allocation-principal none))
          
          (ok true)
        )
        (ok false)
    )
  ))

;; Calculate newly vested amount for milestones (placeholder implementation)
(define-private (set-newly-vested (months uint) (triggers (list 10 uint)) (percentages (list 10 uint)) (total uint) (released uint))
  ;; Implementation would check which milestones have been triggered
  ;; and calculate the newly vested amount
  (ok u0))

;; Administrative Functions

;; Add an administrator
(define-public (add-administrator (admin principal))
  (begin
    (asserts! (is-administrator tx-sender) (err ERR_UNAUTHORIZED))
    (map-set administrators admin true)
    (ok true)))

;; Remove an administrator
(define-public (remove-administrator (admin principal))
  (begin
    (asserts! (is-administrator tx-sender) (err ERR_UNAUTHORIZED))
    (map-set administrators admin false)
    (ok true)))

;; Update blocks per month
(define-public (update-blocks-per-month (new-value uint))
  (begin
    (asserts! (is-administrator tx-sender) (err ERR_UNAUTHORIZED))
    (asserts! (> new-value u0) (err ERR_INVALID_PARAMETER))
    (var-set blocks-per-month new-value)
    (ok true))) 