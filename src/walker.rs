use tree_sitter::{Node, Parser};

use crate::transform::{cases, normalize};

pub fn process(source: &str, lang: tree_sitter::Language, from: String, to: String) -> String {
    let mut parser = Parser::new();
    parser.set_language(&lang).unwrap();

    let tree = parser.parse(source, None).unwrap();
    let root = tree.root_node();

    let mut edits = Vec::new();

    walk(root, source, &mut edits, &to);

    apply_edits(source, edits)
}

fn walk(node: Node, source: &str, edits: &mut Vec<(usize, usize, String)>, to: &str) {
    if is_identifier(node) {
        let text = &source[node.start_byte()..node.end_byte()];

        let words = normalize::split_words(text);
        let new_name = cases::apply_case(&words, to);

        if new_name != text {
            edits.push((node.start_byte(), node.end_byte(), new_name));
        }
    }

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        walk(child, source, edits, to);
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
