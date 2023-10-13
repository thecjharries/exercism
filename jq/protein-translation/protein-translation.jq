def codonToProtein:
  . | ascii_upcase |
  if "AUG" == . then
    "Methionine"
  elif "UUU" == . or "UUC" == . then
    "Phenylalanine"
  elif "UUA" == . or "UUG" == . then
    "Leucine"
  elif "UCU" == . or "UCC" == . or "UCA" == . or "UCG" == . then
    "Serine"
  elif "UAU" == . or "UAC" == . then
    "Tyrosine"
  elif "UGU" == . or "UGC" == . then
    "Cysteine"
  elif "UGG" == . then
    "Tryptophan"
  elif "UAA" == . or "UAG" == . or "UGA" == . then
    "STOP"
  end
;
