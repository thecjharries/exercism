(.x * .x + .y * .y) as $circle |
  if $circle <= 1.0 then
    10
  elif $circle <= 25.0 then
    5
  elif $circle <= 100.0 then
    1
  else
    0
  end
