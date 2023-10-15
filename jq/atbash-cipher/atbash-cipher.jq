def encode:
  reduce explode[] as $char (
    "";
    . + (
      if $char >= 97 and $char <= 122 then
        [ (122 - $char) + 97 ]
      elif $char >= 65 and $char <= 90 then
        [ (90 - $char) + 65 ]
      elif $char >= 48 and $char <= 57 then
        [ $char ]
      else
        []
      end |
        implode |
        ascii_downcase
    )
  ) |
    [scan(".{1,5}")] |
    join(" ")
;

if "encode" == .property then
  .input.phrase | encode
else
  "Implement" | halt_error
end
