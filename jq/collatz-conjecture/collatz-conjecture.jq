def steps:
  if 0 >= . then
    "Only positive integers are allowed" | halt_error
  elif 1 == . then
    0
  elif 0 == . % 2 then
    1 + ( . / 2 | steps )
  else
    1 + ( 3 * . + 1 | steps )
  end;
