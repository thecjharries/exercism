pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut saddle_points: Vec<(usize, usize)> = Vec::new();
    for (row_index, row) in input.iter().enumerate() {
        if row.is_empty() {
            continue;
        }
        let row_max = row.iter().max().unwrap();
        for (col_index, col) in row.iter().enumerate() {
            let col_min = input.iter().map(|row| row[col_index]).min().unwrap();
            if col == row_max && *col == col_min {
                saddle_points.push((row_index, col_index));
            }
        }
    }
    saddle_points
}
