// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

pub fn convert(input: &str) -> Result<String, Error> {
    let mut result = String::new();
    let mut lines = input.lines().collect::<Vec<&str>>();
    if 0 != lines.len() % 4 {
        return Err(Error::InvalidRowCount(lines.len()));
    }
    let mut line = lines[0];
    let mut column_count = line.len();
    if 0 != column_count % 3 {
        return Err(Error::InvalidColumnCount(column_count));
    }
    Ok(result)
}
