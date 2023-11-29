(defpackage :anagram
  (:use :cl)
  (:export :anagrams-for))

(in-package :anagram)

(defun convert-to-sorted-char-list (word) (sort (coerce (string-upcase word) 'list) #'char-lessp))

(defun anagrams-for (subject candidates)
  "Returns a sublist of candidates which are anagrams of the subject."
  (let ((possible (copy-list candidates)))
    (remove-if-not (lambda (candidate)
                     (and (not (equal (string-upcase candidate) (string-upcase subject)))
                          (equal (convert-to-sorted-char-list candidate)
                                 (convert-to-sorted-char-list subject))))
                   possible)))
