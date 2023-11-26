(defpackage :space-age
  (:use :cl)
  (:export :on-mercury
           :on-venus
           :on-earth
           :on-mars
           :on-jupiter
           :on-saturn
           :on-uranus
           :on-neptune))

(in-package :space-age)

(defconstant EARTH_SECONDS (* 365.25 24 60 60))

(defun rounds-to (number)
  (/ (round (* number 100)) 100.0))

(defun on-mercury (seconds)
  (rounds-to (/ seconds (* EARTH_SECONDS 0.2408467))))

(defun on-venus (seconds)
  (rounds-to (/ seconds (* EARTH_SECONDS 0.61519726))))

(defun on-earth (seconds)
  (rounds-to (/ seconds EARTH_SECONDS)))

(defun on-mars (seconds)
  (rounds-to (/ seconds (* EARTH_SECONDS 1.8808158))))

(defun on-jupiter (seconds)
  (rounds-to (/ seconds (* EARTH_SECONDS 11.862615))))

(defun on-saturn (seconds)
  (rounds-to (/ seconds (* EARTH_SECONDS 29.447498))))

(defun on-uranus (seconds)
  (rounds-to (/ seconds (* EARTH_SECONDS 84.016846))))

(defun on-neptune (seconds)
  (rounds-to (/ seconds (* EARTH_SECONDS 164.79132))))
