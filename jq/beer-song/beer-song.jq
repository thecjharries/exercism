def capitalize:
  (.[:1] | ascii_upcase) + .[1:]
;

def bottles:
  if . == 0 then
    "no more bottles"
  elif . == 1 then
    "1 bottle"
  else
    "\(. | tostring) bottles"
  end
;

[range(.startBottles; .startBottles - .takeDown; -1)] |
  map(
    [
      "\(. | bottles | capitalize) of beer on the wall, \(. | bottles) of beer.",
      if . == 0 then
        "Go to the store and buy some more, 99 bottles of beer on the wall."
      else
        "Take \(if . == 1 then "it" else "one" end) down and pass it around, \(. - 1 | bottles) of beer on the wall."
      end,
      ""
    ]
  ) |
  add[:-1]
