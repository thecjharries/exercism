(defpackage :hamming
  (:use :cl)
  (:export :distance))

(in-package :hamming)

(defun distance (dna1 dna2)
  "Number of positional differences in two equal length dna strands."
  (cond ((not (equal (length dna1) (length dna2))) nil)
        ((equal dna1 dna2) 0)
        (t (loop for i from 0 below (length dna1)
                 count (not (equal (char dna1 i) (char dna2 i))))))
  )
