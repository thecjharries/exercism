(defpackage :anagram
  (:use :cl)
  (:export :anagrams-for))

(in-package :anagram)

(defun convert-to-sorted-char-list (word) (sort (every #'alpha-char-p (lower-case-p word)))

(defun anagrams-for (subject candidates)
  "Returns a sublist of candidates which are anagrams of the subject."
  )
