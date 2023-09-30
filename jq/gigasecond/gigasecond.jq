.moment as $moment |
  try(
    .moment | strptime("%Y-%m-%d") | todateiso8601
  ) catch(
    "\($moment)Z"
  ) |
  fromdateiso8601 |
  . + 1000000000 |
  todateiso8601 |
  sub("Z$"; "")
