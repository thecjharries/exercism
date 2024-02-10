(ns raindrops)

(defn convert [n]
  (let [factors (filter (fn [[k v]] (zero? (rem n k))) {3 "Pling", 5 "Plang", 7 "Plong"})]
    (if (empty? factors)
      (str n)
      (apply str (map second factors)))))
