(defpackage :isogram
  (:use :cl)
  (:export :isogram-p))

(in-package :isogram)

(defun isogram-p (string)
  "Is string an Isogram?"
  (let ((comparison (remove-if-not #'alpha-char-p (string-downcase string))))
    (string= comparison (remove-duplicates comparison))))
