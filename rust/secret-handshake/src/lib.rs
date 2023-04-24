pub fn actions(n: u8) -> Vec<&'static str> {
    let actions = format!("{:05b}", n).chars().rev().collect::<Vec<_>>();
    let mut result = vec![];
    if '1' == actions[0] {
        result.push("wink");
    }
    if '1' == actions[1] {
        result.push("double blink");
    }
    if '1' == actions[2] {
        result.push("close your eyes");
    }
    if '1' == actions[3] {
        result.push("jump");
    }
    if '1' == actions[4] {
        result.reverse();
    }
    result
}
