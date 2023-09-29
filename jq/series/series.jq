. as  {series: $series, sliceLength: $sliceLength} |
if "" == $series then
  "series cannot be empty" | halt_error
elif 0 > $sliceLength then
  "slice length cannot be negative" | halt_error
elif 0 == $sliceLength then
  "slice length cannot be zero" | halt_error
elif ($series | length) < $sliceLength  then
  "slice length cannot be greater than series length" | halt_error
else . end |
[ range(0; ($series | length) - $sliceLength + 1) ] |
map($series[. : . + $sliceLength])
