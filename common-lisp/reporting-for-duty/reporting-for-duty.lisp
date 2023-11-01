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
(defun format-two-quarters (stream first-quarter first-value second-quarter second-value) (
  format stream "~%The value ~a quarter: ~a~%The value ~a quarter: ~a~%" first-quarter first-value second-quarter second-value
))

;; Define format-two-quarters-for-reading function.
