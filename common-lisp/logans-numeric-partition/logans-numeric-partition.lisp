(defpackage :logans-numeric-partition
  (:use :cl)
  (:export :categorize-number :partition-numbers))

(in-package :logans-numeric-partition)

;; Define categorize-number function
(defun categorize-number (list number)
  (if (evenp number)
    (cons (car list) (cons number (cdr list)))
    (cons (cons number (car list)) (cdr list))))

;; Define partition-numbers function
(defun partition-numbers (list)
  (reduce #'categorize-number list :initial-value '(() . ())))
