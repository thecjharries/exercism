(defpackage :character-study
  (:use :cl)
  (:export
   :compare-chars
   :size-of-char
   :change-size-of-char
   :type-of-char))
(in-package :character-study)

(defun compare-chars (char1 char2)
  (if (char= char1 char2)
      :equal-to
      (if (char< char1 char2)
          :less-than
          :greater-than)))

(defun size-of-char (char)
  (if (alpha-char-p char)
    (if (upper-case-p char)
        :big
        :small)
    :no-size))

(defun change-size-of-char (char wanted-size)
  (if (eq wanted-size :big)
      (char-upcase char)
      (char-downcase char)))

(defun type-of-char (char))
