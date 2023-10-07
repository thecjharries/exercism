def colorToNumber:
  if "brown" == . then
    1
  elif "red" == . then
    2
  elif "orange" == . then
    3
  elif "yellow" == . then
    4
  elif "green" == . then
    5
  elif "blue" == . then
    6
  elif "violet" == . then
    7
  elif "grey" == . then
    8
  elif "white" == . then
    9
  else
    0
  end
;

((.colors[0] | colorToNumber) * 10 + (.colors[1] | colorToNumber)) * pow(10; .colors[2] | colorToNumber) |
  if 1000000000 < . then
    {
      "value": (. / 1000000000),
      "unit": "gigaohms"
    }
  elif 1000000 < . then
    {
      "value": (. / 1000000),
      "unit": "megaohms"
    }
  elif 1000 < . then
    {
      "value": (. / 1000),
      "unit": "kiloohms"
    }
  else
    {
      "value": .,
      "unit": "ohms"
    }
  end
