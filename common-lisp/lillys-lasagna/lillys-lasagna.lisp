(defpackage :lillys-lasagna
  (:use :cl)
  (:export :expected-time-in-oven
           :remaining-minutes-in-oven
           :preparation-time-in-minutes
           :elapsed-time-in-minutes))

(in-package :lillys-lasagna)

;; Define function expected-time-in-oven
(defun expected-time-in-oven ()
  "how many minutes the lasagna should be in the oven"
  337)

;; Define function remaining-minutes-in-oven
(defun remaining-minutes-in-oven (time-in-oven)
  "how many minutes the lasagna still has to remain in the oven"
  (- (expected-time-in-oven) time-in-oven))

;; Define function preparation-time-in-minutes
(defun preparation-time-in-minutes (number-of-layers)
  "how many minutes it takes to prepare a lasagna with a certain number of layers"
  (* number-of-layers 19))

;; Define function elapsed-time-in-minutes
(defun elapsed-time-in-minutes (number-of-layers time-in-oven)
  "how many minutes have elapsed since the process was started"
  (+ (preparation-time-in-minutes number-of-layers) time-in-oven))
