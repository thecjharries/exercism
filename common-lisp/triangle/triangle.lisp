(defpackage :triangle
  (:use :cl)
  (:export :triangle-type-p))

(in-package :triangle)

(defun is-triangle-p (a b c)
  (and (<= (+ a b) c)
       (<= (+ a c) b)
       (<= (+ b c) a)))

(defun triangle-type-p (type a b c)
  "Deterimines if a triangle (given by side lengths A, B, C) is of the given TYPE"
  )
