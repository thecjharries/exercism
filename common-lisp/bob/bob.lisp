(defpackage :bob
  (:use :cl)
  (:export :response))
(in-package :bob)

(defun response (hey-bob)
  (setf hey-bob (remove #\Space (remove #\Newline (remove #\Tab hey-bob))))
  (cond ((zerop (length hey-bob)) "Fine. Be that way")
    (t "Whatever.")))
