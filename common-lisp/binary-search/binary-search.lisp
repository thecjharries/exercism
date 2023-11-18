(defpackage :binary-search
  (:use :cl)
  (:export :binary-find :value-error))

(in-package :binary-search)

(defun binary-find (arr el)
  (let ((low 0)
        (high (1- (length arr))))
    (loop while (<= low high)
          do (let ((mid (floor (+ low high) 2)))
               (cond ((= (aref arr mid) el) (return-from binary-find mid))
                     ((< (aref arr mid) el) (setf low (1+ mid)))
                     (t (setf high (1- mid))))))
    nil))

