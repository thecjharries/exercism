const BRACKET_OPENERS: [char; 3] = ['(', '[', '{'];
const BRACKET_CLOSERS: [char; 3] = [')', ']', '}'];

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = Vec::new();
    for c in string.chars() {
        if BRACKET_OPENERS.contains(&c) {
            stack.push(c);
        } else if BRACKET_CLOSERS.contains(&c) {
            if let Some(last) = stack.pop() {
                if BRACKET_OPENERS.iter().position(|&x| x == last) != BRACKET_CLOSERS.iter().position(|&x| x == c) {
                    return false;
                }
            } else {
                return false;
            }
        }
    }
    stack.is_empty()
}
