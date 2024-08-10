
let find input target =
    let rec search left right =
      if left > right then
        Error "value not in array"
      else
        let mid = (left + right) / 2 in
        match input.(mid) with
        | x when x = target -> Ok mid
        | x when x < target -> search (mid + 1) right
        | _ -> search left (mid - 1)
    in
    search 0 (Array.length input - 1)
