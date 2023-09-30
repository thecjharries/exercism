# input: a floating point number
# output: the number rounded to two decimal places
def two_decimal: ((. * 100) | round) / 100;

if "Mercury" == .planet then
  .seconds / 0.2408467
elif "Venus" == .planet then
  .seconds / 0.61519726
elif "Earth" == .planet then
  .seconds
elif "Mars" == .planet then
  .seconds / 1.8808158
elif "Jupiter" == .planet then
  .seconds / 11.862615
elif "Saturn" == .planet then
  .seconds / 29.447498
elif "Uranus" == .planet then
  .seconds / 84.016846
elif "Neptune" == .planet then
  .seconds / 164.79132
else
  "not a planet" | halt_error
end |
  . / 365.25 / 24.0 / 60.0 / 60.0 |
  two_decimal
