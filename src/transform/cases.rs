pub fn apply_case(words: &[String], case: &str) -> String {
    match case {
        "snake_case" => words.join("_"),
        "camelCase" => to_camel(words),
        "PascalCase" => to_pascal(words),
        "kebab-case" => words.join("-"),
        "SCREAMING_SNAKE_CASE" => words.join("_").to_uppercase(),
        "SCREAMING-KEBAB-CASE" => words.join("-").to_uppercase(),
        _ => panic!("Unsupported case: {}", case),
    }
}

fn to_camel(words: &[String]) -> String {
    let mut result = String::new();
    for (i, w) in words.iter().enumerate() {
        if i == 0 {
            result.push_str(w);
        } else {
            result.push_str(&capitalize(w));
        }
    }
    result
}

fn to_pascal(words: &[String]) -> String {
    words.iter().map(|w| capitalize(w)).collect::<String>()
}

fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}
