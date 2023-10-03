if .preorder | length == 0 then
  {}
elif (.preorder | length) != (.inorder | length) then
  "traversals must have the same length" | halt_error

end
