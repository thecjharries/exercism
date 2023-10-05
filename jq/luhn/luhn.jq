test("^[0-9 ]+$") and (
  gsub(" "; "") |
  split("") |
  reverse |
  map(tonumber) |
  to_entries |
  map(
    if 1 == .key % 2 then
      if 9 < .value * 2 then
        .value * 2 - 9
      else
        .value * 2
      end
    else
      .value
    end
  ) |
  length > 1 and add % 10 == 0
)
