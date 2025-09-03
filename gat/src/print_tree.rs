use std::io::Write;
use tree_sitter::{Node, Parser, Query, Tree};

pub fn print_tree<W: Write>(node: &Node, depth: i32, source: &[u8], writer: &mut W) {
    let mut padding = "".to_string();
    let mut i = 0;
    while i < depth {
        padding += "    ";
        i += 1;
    }

    let text = node.utf8_text(source);
    let mut data = "".to_string();
    if node.child_count() == 0 {
        data = format!(
            "{}{}:{}   {}\n",
            padding,
            node.id(),
            node.kind(),
            text.unwrap()
        )
    } else {
        data = format!("{}{}:{}\n", padding, node.id(), node.kind());
    }

    writer.write_all(data.as_bytes()).unwrap();

    for child_idx in 0..node.child_count() {
        let child = node.child(child_idx).unwrap();
        print_tree(&child, depth + 1, source, writer);
    }
}
