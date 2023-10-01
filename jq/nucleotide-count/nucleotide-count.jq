reduce (.strand / "")[] as $element (
  {A: 0, C: 0, G: 0, T: 0};
  if has($element) then .[$element] += 1
  else "Invalid nucleotide in strand" | halt_error
  end
)
