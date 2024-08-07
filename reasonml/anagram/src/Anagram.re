let anagrams = (word, words) => {
  let sortString = unsorted =>
    unsorted
    |> String.lowercase_ascii
    |> Js.String.split("")
    |> Js.Array.sortInPlace
    |> Js.Array.joinWith("");

  let baseString = sortString(word);

  words
  |> List.filter(
    candidate =>
      sortString(candidate) === baseString
      && String.lowercase_ascii(candidate) !== String.lowercase_ascii(word)
  )
};
