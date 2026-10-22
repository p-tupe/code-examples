pub fn reply(message: &str) -> &str {
    match (
        message.trim().is_empty(),
        message.trim().ends_with("?"),
        message.contains(char::is_alphabetic) && !message.contains(char::is_lowercase),
    ) {
        (true, _, _) => "Fine. Be that way!",   // Silent
        (_, true, false) => "Sure.",            // Question
        (_, false, true) => "Whoa, chill out!", // Yell
        (_, true, true) => "Calm down, I know what I'm doing!", // Yelling Question
        _ => "Whatever.",                       // Whatever
    }
}
