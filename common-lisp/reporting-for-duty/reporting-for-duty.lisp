(defpackage :reporting-for-duty
  (:use :cl)
  (:export :format-quarter-value :format-two-quarters
           :format-two-quarters-for-reading))

(in-package :reporting-for-duty)

;; Define format-quarter-value function.
(defun format-quarter-value (quarter value) (
  format nil "The value ~a quarter: ~a" quarter value
))

;; Define format-two-quarters function.

;; Define format-two-quarters-for-reading function.
