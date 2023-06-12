pub fn abbreviate(phrase: &str) -> String {
    let (_, result) = phrase.chars().fold(
        (None, String::from("")),
        |(option, mut result): (Option<char>, _), c| {
            if let Some(last) = option {
                if last != '\''
                    && c.is_alphabetic()
                    && (!last.is_alphabetic()
                        || (last.is_alphabetic() && last.is_lowercase() && c.is_uppercase()))
                {
                    result.push(c);
                    return (Some(c), result);
                }
            } else {
                result.push(c);
            }
            return (Some(c), result);
        },
    );
    result.to_uppercase()
}
