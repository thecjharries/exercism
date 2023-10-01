(.limit + 1) as $limit |
  reduce range(2; $limit) as $index (
    [range($limit)];
    if .[$index] then
      reduce range(2 * $index; $limit; $index) as $iterator (
        .;
        .[$iterator] = false
      )
    else
      .
    end
  ) |
  .[2:] |
  map(select(. != false))

