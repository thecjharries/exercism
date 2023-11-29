(defpackage :triangle
  (:use :cl)
  (:export :triangle-type-p))

(in-package :triangle)

(defun is-triangle-p (a b c)
  (and (< 0 a)
       (< 0 b)
       (< 0 c)
       (>= (+ a b) c)
       (>= (+ a c) b)
       (>= (+ b c) a)))

(defun is-isosceles-p (a b c)
  (or (= a b)
      (= b c)
      (= a c)))

(defun is-equilateral-p (a b c)
  (= a b c))

(defun is-scalene-p (a b c)
  (not (is-isosceles-p a b c)))

(defun triangle-type-p (type a b c)
  "Deterimines if a triangle (given by side lengths A, B, C) is of the given TYPE"
  (and (is-triangle-p a b c)
       (case type
         (:equilateral (is-equilateral-p a b c))
         (:isosceles (is-isosceles-p a b c))
         (:scalene (is-scalene-p a b c))
         (otherwise nil))))
