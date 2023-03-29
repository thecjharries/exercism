pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let mut matrix = vec![vec![0; size as usize]; size as usize];
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut dx: i32 = 1;
    let mut dy: i32 = 0;
    for i in 1..=size * size {
        matrix[y as usize][x as usize] = i;
        if x + dx == size as i32
            || y + dy == size as i32
            || x + dx == -1
            || y + dy == -1
            || matrix[(y + dy) as usize][(x + dx) as usize] != 0
        {
            let temp = dx;
            dx = -dy;
            dy = temp;
        }
        x += dx;
        y += dy;
    }
    matrix
}
