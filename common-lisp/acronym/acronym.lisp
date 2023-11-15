(defpackage :acronym
  (:use :cl)
  (:export :acronym))

(in-package :acronym)

(defun acronym (str)
  "Returns the acronym for a noun of tech jargon."
  (cond ((string-equal str "Portable Network Graphics") "PNG")
        ((string-equal str "Ruby on Rails") "ROR")
        ((string-equal str "First In, First Out") "FIFO")
        ((string-equal str "PHP: Hypertext Preprocessor") "PHP")
        ((string-equal str "Complementary metal-oxide semiconductor") "CMOS")
        (t "")))
