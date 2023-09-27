.heyBob | if test("^\\s*$") then
  "Fine. Be that way!"
elif test("\\?\\s*$") then
  if test("^[^a-z]*[A-Z][^a-z]*$") then
    "Calm down, I know what I'm doing!"
  else
    "Sure."
  end
elif test("^[^a-z]*[A-Z][^a-z]*$") then
  "Whoa, chill out!"
else
  "Whatever."
end
