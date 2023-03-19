pub fn reply(message: &str) -> &str {
    let message = message.trim();
    if message.is_empty() {
        return "Fine. Be that way!";
    }
    let mut all_caps = true;
    let mut has_letters = false;
    for c in message.chars() {
        if c.is_alphabetic() {
            has_letters = true;
            if c.is_lowercase() {
                all_caps = false;
                break;
            }
        }
    }
    let mut question = false;
    if message.ends_with('?') {
        question = true;
    }
    if has_letters && all_caps && question {
        "Calm down, I know what I'm doing!"
    } else if has_letters && all_caps {
        "Whoa, chill out!"
    } else if question {
        "Sure."
    } else {
        "Whatever."
    }
}
