pub fn count(lines: &[&str]) -> u32 {
    let grid: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();
    let mut rectangle_count = 0;
    for (y, row) in grid.iter().enumerate() {
        for (x, character) in row.iter().enumerate() {
            if '+' != *character {
                continue;
            }
            let mut x_corners = Vec::new();
            for next_x in x + 1..row.len() {
                if '+' == row[next_x] {
                    x_corners.push(next_x);
                }
            }
            let mut y_corners = Vec::new();
            for next_y in y + 1..grid.len() {
                if '+' == grid[next_y][x] {
                    y_corners.push(next_y);
                }
            }
            for &next_x in &x_corners {
                for &next_y in &y_corners {
                    if '+' == grid[next_y][next_x] {
                        let mut okay_to_add = true;
                        for x in x + 1..next_x {
                            match grid[next_y][x] {
                                '-' | '+' => {}
                                _ => {
                                    okay_to_add = false;
                                    break;
                                }
                            }
                        }
                        if okay_to_add {
                            for y in y + 1..next_y {
                                match grid[y][x] {
                                    '|' | '+' => {}
                                    _ => {
                                        okay_to_add = false;
                                        break;
                                    }
                                }
                                match grid[y][next_x] {
                                    '|' | '+' => {}
                                    _ => {
                                        okay_to_add = false;
                                        break;
                                    }
                                }
                            }
                            if okay_to_add {
                                rectangle_count += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    rectangle_count
}
