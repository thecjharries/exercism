if 0 < (.lines | length) then
  .lines |
    map(. / "") as $rows |
    $rows |
    map(length) |
    max |
    reduce range(.) as $index (
      [];
      . + [
        $rows[0:(
          $rows |
          to_entries |
          map(select(.value | length > $index) | .key) |
          last + 1
        )] |
        map(.[$index] // " ") |
        join("")
      ]
    )
else
  []
end
