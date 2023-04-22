pub fn answer(command: &str) -> Option<i32> {
    let command = command
        .trim_start_matches("What is ")
        .trim_end_matches('?')
        .split_whitespace()
        .collect::<Vec<&str>>();
    if 1 == command.len() {
        return command[0].parse::<i32>().ok();
    }
    unimplemented!("Return the result of the command '' or None, if the command is invalid.");
}
