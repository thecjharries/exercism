pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut result = Vec::new();
    for (row_index, row) in minefield.iter().enumerate() {
        let mut annotated_row = String::new();
        for (col_index, ch) in row.chars().enumerate() {
            if ch == '*' {
                annotated_row.push('*');
            } else {
                let mut count = 0;
                for (i, j) in &[
                    (row_index as i32 - 1, col_index as i32 - 1),
                    (row_index as i32 - 1, col_index as i32),
                    (row_index as i32 - 1, col_index as i32 + 1),
                    (row_index as i32, col_index as i32 - 1),
                    (row_index as i32, col_index as i32 + 1),
                    (row_index as i32 + 1, col_index as i32 - 1),
                    (row_index as i32 + 1, col_index as i32),
                    (row_index as i32 + 1, col_index as i32 + 1),
                ] {
                    if *i >= 0
                        && *i < minefield.len() as i32
                        && *j >= 0
                        && *j < row.len() as i32
                        && minefield[*i as usize].chars().nth(*j as usize).unwrap() == '*'
                    {
                        count += 1;
                    }
                }
                if count == 0 {
                    annotated_row.push(' ');
                } else {
                    annotated_row.push_str(&count.to_string());
                }
            }
        }
        result.push(annotated_row);
    }
    result
}
