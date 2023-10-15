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
