if .preorder | length == 0 then
  {}
elif (.preorder | length) != (.inorder | length) then
  "traversals must have the same length" | halt_error
elif (.preorder | sort) != (.inorder | sort) then
  "traversals must have the same elements" | halt_error
end
