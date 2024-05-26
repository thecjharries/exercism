let annotate grid =
    let grid = Array.of_list grid in
    let row_count = Array.length grid in
    let get_cell x y =
        if x < 0 || x >= row_count then ' '
        else
            let row = grid.(x) in
            if y < 0 || y >= String.length row then ' '
            else row.[y]
        in
    let count_mines x y = [
        -1, -1; -1, 0; -1, 1;
        0, -1; 0, 1;
        1, -1; 1, 0; 1, 1
    ] |> List.map (fun (dx, dy) -> get_cell (x + dx) (y + dy))
      |> List.filter ((<>) ' ')
      |> List.length
    in
    Array.mapi (fun x row ->
        String.mapi (fun y cell ->
            if cell = '*' then '*'
            else
                let count = count_mines x y in
                if count = 0 then ' '
                else Char.chr (int_of_char '0' + count)
        ) row
    ) grid |> Array.to_list
