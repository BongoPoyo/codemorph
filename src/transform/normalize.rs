pub fn split_words(s: &str) -> Vec<String> {
    let mut words = Vec::new();
    let mut current = String::new();

    for (i, c) in s.chars().enumerate() {
        if i > 0 && c.is_uppercase() {
            words.push(current.clone());
            current.clear();
        }
        if c != '_' && c != '-' {
            current.push(c.to_ascii_lowercase());
        } else if !current.is_empty() {
            words.push(current.clone());
            current.clear();
        }
    }

    if !current.is_empty() {
        words.push(current);
    }

    words
}
