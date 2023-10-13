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
  else
    "Invalid codon" | halt_error
  end
;

def rnaToProtein:
  if 0 == length then
    []
  else
    (.[0:3] | codonToProtein) as $protein |
    if "STOP" == $protein then
      []
    else
      [$protein] + (.[3:] | rnaToProtein)
    end
  end
;

