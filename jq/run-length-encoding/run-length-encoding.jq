def encode:
  . |
    split("") |
    .[0] as $current |
    0 as $count |
    reduce .[] as $char (
      {
        current: "",
        count: 0,
        output: []
      };
      if $char == .current then
        .count += 1
      else
        if .count > 0 then
          .output += [(if 1 < .count then (.count | tostring) else "" end), .current]
        end |
        .current = $char |
        .count = 1
      end
    ) |
    .output += [(if 1 < .count then (.count | tostring) else "" end), .current] |
    .output |
    join("")
;

def decode:
  "Implement this function" | halt_error;
