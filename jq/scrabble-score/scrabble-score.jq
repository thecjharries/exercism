def letterScore($letter):
  if (["A", "E", "I", "O", "U", "L", "N", "R", "S", "T"] | index($letter)) != null then
    1
  elif (["D", "G"] | index($letter)) != null then
    2
  elif (["B", "C", "M", "P"] | index($letter)) != null then
    3
  elif (["F", "H", "V", "W", "Y"] | index($letter)) != null then
    4
  elif (["K"] | index($letter)) != null then
    5
  elif (["J", "X"] | index($letter)) != null then
    8
  elif (["Q", "Z"] | index($letter)) != null then
    10
  else
    0
  end
;

if "" == .word then
  0
else
  .word | ascii_upcase | split("") | map(letterScore(.)) | add
end

