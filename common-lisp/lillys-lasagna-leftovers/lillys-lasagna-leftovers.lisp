(defpackage :lillys-lasagna-leftovers
  (:use :cl)
  (:export
   :preparation-time
   :remaining-minutes-in-oven
   :split-leftovers))

(in-package :lillys-lasagna-leftovers)

;; Define function preparation-time
(defun preparation-time (&rest layers) (
  * 19 (length layers)
))

;; Define function remaining-minutes-in-oven
(defun remaining-minutes-in-oven (&optional (change :normal)) (
  + 337 (cond ((eq change :shorter) -100)
              ((eq change :very-short) -200)
              ((eq change :longer) 100)
              ((eq change :very-long) 200)
              ((eq change nil) -337)
              (t 0))
))

;; Define function split-leftovers
(defun split-leftovers (&key (alien 10) (human 10) (weight nil weight-p))
  (cond
    ((and (not weight) weight-p) :looks-like-someone-was-hungry)
    ((not weight) :just-split-it)
    (t (- weight alien human))))
