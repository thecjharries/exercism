#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    if 0 == span {
        return Ok(1);
    }
    if string_digits.len() < span {
        return Err(Error::SpanTooLong);
    }
    let digits = string_digits
        .chars()
        .map(|c| c.to_digit(10).ok_or(Error::InvalidDigit(c)))
        .collect::<Result<Vec<_>, _>>()?;
    let mut max = 0;
    for window in digits.windows(span) {
        let product = window.iter().product();
        if product > max {
            max = product;
        }
    }
    Ok(max as u64)
}
