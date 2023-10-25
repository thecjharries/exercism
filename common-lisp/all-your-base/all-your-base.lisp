(defpackage :all-your-base
  (:use :cl)
  (:export :rebase))

(in-package :all-your-base)

(defun to-base-ten (list-digits base) (
  loop for i from 0 to (1- (length list-digits)) sum
    (* (nth i list-digits) (expt base (- (1- (length list-digits)) i)))
  )
)

(defun from-base-ten-to-list (input base)
  (let (quotient remainder)
    (setf (values quotient remainder) (floor input base))
    (if (zerop quotient)
      (list remainder)
      (append (from-base-ten-to-list quotient base) (list remainder))
    )
  )
)


(defun rebase (list-digits in-base out-base) (
  cond
    ((< in-base 2) nil)
    ((< out-base 2) nil)
    ((null list-digits) '(0))
    ((some (lambda (x) (or (< x 0) (>= x in-base))) list-digits) nil)
    (T (from-base-ten-to-list (to-base-ten list-digits in-base) out-base))
  )
)
