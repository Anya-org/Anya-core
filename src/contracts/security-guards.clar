;; Reentrancy protection
(define-data-var reentrancy-lock bool false)

;; Enhanced reentrancy protection with context tracking
(define-data-var reentrancy-context (optional principal) none)

(define-private (non-reentrant)
  (asserts! (is-none (var-get reentrancy-context)) (err u501))
  (var-set reentrancy-context (some (contract-caller)))
)

(define-private (release-reentrancy)
  (var-set reentrancy-context none)
)

;; Enhanced arithmetic guards with context tracking
(define-data-var arithmetic-context (optional principal) none)

(define-private (with-arithmetic-guard (fn (function (uint uint) (response uint uint))))
  (lambda (a uint b uint)
    (let (
      (ctx (var-get arithmetic-context))
      (caller (contract-caller))
    )
    (asserts! (is-none ctx) (err u504))
    (var-set arithmetic-context (some caller))
    (let ((result (fn a b)))
      (var-set arithmetic-context none)
      result
    )
  )
))

;; Wrap core arithmetic operations
(define-public safe-add (with-arithmetic-guard 
  (lambda (a uint b uint) 
    (let ((sum (+ a b)))
      (asserts! (>= sum a) (err u502))
      (ok sum)
    )
  )
))

(define-public safe-mul (with-arithmetic-guard
  (lambda (a uint b uint)
    (let ((product (* a b)))
      (asserts! (or (eq a 0) (eq (div product a) b)) (err u503))
      (ok product)
    )
  )
))

;; Time-locked arithmetic operations
(define-private (time-locked-add (a uint) (b uint) (lock-period uint))
  (let (
    (sum (safe-add a b))
    (block-height (at-block-height (unwrap! sum (err u502))))
  )
  (asserts! (> block-height (+ (var-get last-operation-height) lock-period)) (err u503))
  (ok sum)
  )
) 