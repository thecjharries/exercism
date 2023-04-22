pub fn answer(command: &str) -> Option<i32> {
    let command = command.replace("by", "");
    let command = command
        .trim_start_matches("What is ")
        .trim_end_matches('?')
        .split_whitespace()
        .collect::<Vec<&str>>();
    if 1 == command.len() {
        return command[0].parse::<i32>().ok();
    }
    let mut result = command[0].parse::<i32>().ok()?;
    let mut iter = command.iter().skip(1);
    while let Some(operator) = iter.next() {
        let operand = iter.next()?.parse::<i32>().ok()?;
        match operator {
            &"plus" => result += operand,
            &"minus" => result -= operand,
            &"multiplied" => {
                result *= operand;
            }
            &"divided" => {
                result /= operand;
            }
            _ => return None,
        }
    }
    Some(result)
}
