(defpackage :two-fer
  (:use :cl)
  (:export :twofer))
(in-package :two-fer)

(defun twofer (name)
  (let ((used-name (if (null name) "you" name)))
  (format nil "One for ~a, one for me." used-name))
)
