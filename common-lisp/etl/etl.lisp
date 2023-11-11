(defpackage :etl
  (:use :cl)
  (:export :transform))

(in-package :etl)

(defun transform (data)
  "Transforms hash values into keys with their keys as their values."
  (let ((result (make-hash-table)))
    (maphash #'(lambda (key value)
               (mapcar #'(lambda (letter)
                         (setf (gethash (char-downcase letter) result) key))
                       value))
             data)
    result
  ))
