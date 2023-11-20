(defpackage :collatz-conjecture
  (:use :cl)
  (:export :collatz))

(in-package :collatz-conjecture)

(defun collatz (n &optional (steps 0)) (
  cond ((< n 1) nil)
       ((= n 1) steps)
       ((evenp n) (collatz (/ n 2) (1+ steps)))
       (t (collatz (+ (* n 3) 1) (1+ steps)))))
