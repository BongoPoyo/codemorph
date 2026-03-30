use tree_sitter::{Node, Parser, Language};
use crate::transform::{cases, normalize};
use std::collections::HashSet;

// Reserved identifiers per language
fn reserved_identifiers(lang: &str) -> HashSet<&'static str> {
    match lang {
        "c" | "cpp" => [
            "printf", "scanf", "main", "sizeof", "exit", "malloc", "free", "NULL",
            "int", "float", "double", "char", "void", "long", "short",
            "if", "else", "while", "for", "switch", "case", "break", "continue", "return",
        ]
        .iter()
        .copied()
        .collect(),

        "rust" => [
            "println!", "print!", "eprintln!", "Vec", "String", "Box", "Option", "Some", "None",
            "Result", "Ok", "Err", "fn", "let", "mut", "pub", "struct", "enum", "impl",
            "use", "mod", "crate", "self", "super", "const", "static", "as", "trait", "type",
        ]
        .iter()
        .copied()
        .collect(),

        _ => HashSet::new(),
    }
}

/// Process code and convert identifiers to `to_case`.
/// `_from` is currently unused (future-proofing for explicit from-case).
pub fn process(source: &str, lang: Language, _from: String, to: String) -> String {
    let mut parser = Parser::new();
    parser.set_language(&lang).unwrap();

    let tree = parser.parse(source, None).unwrap();
    let root = tree.root_node();

    // Detect language name string for reserved identifiers
    let lang_name = if lang == tree_sitter_c::language() {
        "c"
    } else if lang == tree_sitter_cpp::language() {
        "cpp"
    } else if lang == tree_sitter_rust::language() {
        "rust"
    } else {
        ""
    };

    let reserved = reserved_identifiers(lang_name);

    let mut edits = Vec::new();
    walk(root, source, &mut edits, &to, &reserved);

    apply_edits(source, edits)
}

fn walk(node: Node, source: &str, edits: &mut Vec<(usize, usize, String)>, to: &str, reserved: &HashSet<&str>) {
    if is_identifier(node) {
        let text = &source[node.start_byte()..node.end_byte()];

        // Skip reserved words
        if reserved.contains(text) {
            // println!("Skipping reserved: {}", text); // optional debug
        } else {
            let words = normalize::split_words(text);
            let new_name = cases::apply_case(&words, to);

            if new_name != text {
                edits.push((node.start_byte(), node.end_byte(), new_name));
            }
        }
    }

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        walk(child, source, edits, to, reserved);
    }
}

fn is_identifier(node: Node) -> bool {
    matches!(
        node.kind(),
        "identifier" | "field_identifier" | "type_identifier"
    )
}

fn apply_edits(source: &str, mut edits: Vec<(usize, usize, String)>) -> String {
    edits.sort_by_key(|e| e.0);
    edits.reverse();

    let mut result = source.to_string();
    for (start, end, replacement) in edits {
        result.replace_range(start..end, &replacement);
    }
    result
}