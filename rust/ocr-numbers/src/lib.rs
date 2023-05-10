// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

pub fn convert(input: &str) -> Result<String, Error> {
    let mut result = String::new();
    let lines = input.lines().collect::<Vec<&str>>();
    if 0 != lines.len() % 4 {
        return Err(Error::InvalidRowCount(lines.len()));
    }
    let column_count = lines[0].len();
    if 0 != column_count % 3 {
        return Err(Error::InvalidColumnCount(column_count));
    }
    for numbers in lines.chunks(4) {
        if result.len() > 0 {
            result.push(',');
        }
        for index in (0..numbers[0].len()).step_by(3) {
            let mut digit = String::new();
            for line in numbers.iter().take(3) {
                digit.push_str(&line[index..index + 3]);
            }
            println!("'{}'", digit);
            match digit.as_str() {
                " _ | ||_|" => result.push('0'),
                "     |  |" => result.push('1'),
                " _  _||_ " => result.push('2'),
                " _  _| _|" => result.push('3'),
                "   |_|  |" => result.push('4'),
                " _ |_  _|" => result.push('5'),
                " _ |_ |_|" => result.push('6'),
                " _   |  |" => result.push('7'),
                " _ |_||_|" => result.push('8'),
                " _ |_| _|" => result.push('9'),
                _ => result.push('?'),
            }
        }
    }
    Ok(result)
}
