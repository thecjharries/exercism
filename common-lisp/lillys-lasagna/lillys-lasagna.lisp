(defpackage :lillys-lasagna
  (:use :cl)
  (:export :expected-time-in-oven
           :remaining-minutes-in-oven
           :preparation-time-in-minutes
           :elapsed-time-in-minutes))

(in-package :lillys-lasagna)

;; Define function expected-time-in-oven
(defun expected-time-in-oven () 337)

;; Define function remaining-minutes-in-oven
(defun remaining-minutes-in-oven (time-in-oven)
  (- (expected-time-in-oven) time-in-oven))

;; Define function preparation-time-in-minutes
(defun preparation-time-in-minutes (number-of-layers)
  (* number-of-layers 19))

;; Define function elapsed-time-in-minutes
