(defpackage :lucys-magnificent-mapper
  (:use :cl)
  (:export :make-magnificent-maybe :only-the-best))

(in-package :lucys-magnificent-mapper)

;; Define make-magnificent-maybe function
(defun make-magnificent-maybe (function list) (
  mapcar function list
))

;; Define only-the-best function
(defun only-the-best (function list)
  (remove 1 (remove-if function list)))
