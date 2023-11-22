(defpackage :perfect-numbers
  (:use :cl)
  (:export :classify))

(in-package :perfect-numbers)

(defun find-divisors (n)
  (loop for i from 1 to (/ n 2) when (zerop (mod n i)) collect i))

(defun classify (n))
