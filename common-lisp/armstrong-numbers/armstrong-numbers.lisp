(defpackage :armstrong-numbers
  (:use :cl)
  (:export :armstrong-number-p))
(in-package :armstrong-numbers)

(defun armstrong-number-p (number)
  (let ((digits (map 'list #'digit-char-p (prin1-to-string number))))
    (= number (reduce #'+ (mapcar (lambda (digit) (expt digit (length digits))) digits)))))
