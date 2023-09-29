. as  {series: $series, sliceLength: $sliceLength} |
[ range(0; ($series | length) - $sliceLength + 1) ] |
map($series[. : . + $sliceLength])
