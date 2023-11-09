(defpackage :bob
  (:use :cl)
  (:export :response))
(in-package :bob)

(defun response (hey-bob)
  (setf hey-bob (remove #\Space (remove #\Newline (remove #\Tab hey-bob))))
  (cond ((zerop (length hey-bob)) "Fine. Be that way!")
    ((and (and (string= hey-bob (string-upcase hey-bob)) (some #'alpha-char-p hey-bob)) (char= #\? (uiop:last-char hey-bob))) "Calm down, I know what I'm doing!")
    ((and (string= hey-bob (string-upcase hey-bob)) (some #'alpha-char-p hey-bob)) "Whoa, chill out!")
    ((char= #\? (uiop:last-char hey-bob)) "Sure.")
    (t "Whatever.")))
