pub fn encrypt(input: &str) -> String {
    let mut input = input.to_lowercase();
    input.retain(|c| c.is_alphanumeric());
    let len = input.len();
    let cols = (len as f64).sqrt().ceil() as usize;
    let rows = (len as f64 / cols as f64).ceil() as usize;
    let mut output = String::new();
    for i in 0..cols {
        for j in 0..rows {
            let index = i + j * cols;
            if index < len {
                output.push(input.chars().nth(index).unwrap());
            } else {
                output.push(' ');
            }
        }
        if i < cols - 1 {
            output.push(' ');
        }
    }
    output
}
