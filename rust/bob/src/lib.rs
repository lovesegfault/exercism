pub fn reply(message: &str) -> &str {
    let message = message.trim();
    if message.is_empty() {
        return "Fine. Be that way!";
    }

    let is_question = message.ends_with("?");
    let is_yelled = {
        let alphas: Vec<char> = message.chars().filter(|c| c.is_alphabetic()).collect();
        if alphas.is_empty() {
            false
        } else {
            alphas.into_iter().all(|c| c.is_uppercase())
        }
    };

    match (is_question, is_yelled) {
        (true, false) => "Sure.",
        (true, true) => "Calm down, I know what I'm doing!",
        (false, true) => "Whoa, chill out!",
        (false, false) => "Whatever.",
    }
}
