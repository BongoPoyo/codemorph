use tree_sitter::Language;

pub fn resolve_language(lang_flag: &Option<String>, input: &Option<String>) -> Language {
    let lang_name = match lang_flag {
        Some(l) => l.as_str(),
        None => detect_from_input(input),
    };

    match lang_name {
        "rust" | "rs" => tree_sitter_rust::language(),
        "c" => tree_sitter_c::language(),
        "cpp" | "c++" | "cc" | "cxx" => tree_sitter_cpp::language(),
        _ => panic!("Unsupported or undetected language"),
    }
}

fn detect_from_input(input: &Option<String>) -> &str {
    let file = input
        .as_ref()
        .expect("Language not specified and cannot detect from stdin. Use --lang");

    if let Some(ext) = file.split('.').last() {
        match ext {
            "rs" => "rust",
            "c" => "c",
            "cpp" | "cc" | "cxx" => "cpp",
            _ => panic!("Unknown file extension: {}", ext),
        }
    } else {
        panic!("No file extension found. Use --lang");
    }
}
