use tree_sitter::{Node, Parser, Query, Tree};
pub fn print_tree(node: &Node, depth: i32, source: &[u8]) {
    let mut padding = "".to_string();
    let mut i = 0;
    while i < depth {
        padding += "    ";
        i += 1;
    }

    let text = node.utf8_text(source);
    if node.child_count() == 0 {
        println!(
            "{}{}:{}   {}",
            padding,
            node.id(),
            node.kind(),
            text.unwrap()
        )
    } else {
        println!("{}{}:{}", padding, node.id(), node.kind());
    }

    for child_idx in 0..node.child_count() {
        let child = node.child(child_idx).unwrap();
        print_tree(&child, depth + 1, source);
    }
}