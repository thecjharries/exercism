(defpackage :yacht
  (:use :cl)
  (:export :score))
(in-package :yacht)

(defun score (scores category)
  "Returns the score of the dice for the given category."
  (let* ((scores (sort (copy-list scores) #'<))
         (total (reduce #'+ scores)))
    (or (destructuring-bind (a b c d e) scores
          (case category
            (:yacht
              (when (= a e) 50))
            (:choice
              total)
            (:big-straight
              (when (equal scores '(2 3 4 5 6)) 30))
            (:little-straight
              (when (equal scores '(1 2 3 4 5)) 30))
            (:four-of-a-kind
              (when (or (= a d) (= b e))
                (* 4 c)))
            (:full-house
              (when (and (= a b) (= d e) (< a e) (or (= b c) (= c d)))
                total))
            (otherwise
              (let ((p (position category
                  '(nil :ones :twos :threes :fours :fives :sixes))))
                  (* p (count p scores))))))
          0)))
