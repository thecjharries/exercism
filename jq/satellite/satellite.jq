if .preorder | length == 0 then
  {}
elif (.preorder | length) != (.inorder | length) then
  "traversals must have the same length" | halt_error
elif (.preorder | sort) != (.inorder | sort) then
  "traversals must have the same elements" | halt_error
elif (.preorder | (sort != unique)) then
  "traversals must contain unique items" | halt_error
end
