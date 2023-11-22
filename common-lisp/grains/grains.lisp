(defpackage :grains
  (:use :cl)
  (:export :square :total))
(in-package :grains)

(defun square (n)
  (expt 2 (1- n)))

(defun total () )
