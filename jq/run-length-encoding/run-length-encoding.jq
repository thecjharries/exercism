# These solutions only use reduce and split
# They completely avoid regex

def encode:
  reduce split("")[] as $char (
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
  reduce split("")[] as $char (
      {
        count: "",
        output: ""
      };
      if (try ($char | tonumber) catch false) then
        .count += $char
      else
        if .count == "" then
          .output += $char
        else
          .output += ($char * (.count | tonumber))
        end |
        .count = ""
      end
    ) |
    .output
  ;
