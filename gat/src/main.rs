use crypto::digest::Digest;
use crypto::sha1::Sha1;
use std::any::Any;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::os::fd::AsRawFd;
use tree_sitter::{Node, Parser, Query, Tree};

fn print_tree(node: &Node, depth: i32, source: &[u8]) {
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

// for all i32 we need to really use a varint probably, eventually
struct CompressTreeDataNode {
    id: i32,
    kind: i32,
    text: i32,
}

impl CompressTreeDataNode {
    fn encode(&self, out: &mut Vec<u8>) {
        encode_i32(self.id, out);
        encode_i32(self.kind, out);
        encode_i32(self.text, out);
    }
}

// writing to a file involves knowing the length here
struct CompressTreeNode {
    id: i32,
    kind: i32,
    children: Vec<i32>, // ids
}

impl CompressTreeNode {
    fn encode(&self, out: &mut Vec<u8>) {
        encode_i32(self.id, out);
        encode_i32(self.kind, out);
        // this is a tad naive but simple enough
        encode_i32(self.children.len() as i32, out);
        for child in self.children.iter() {
            encode_i32(*child, out);
        }
    }
}

struct Compressor {
    seen_strs: HashMap<String, i32>,
    current_id: i32,
    current_string_id: i32,
    nodes: Vec<CompressTreeNode>,
    data_nodes: Vec<CompressTreeDataNode>,
}

fn encode_string(str: &str, out: &mut Vec<u8>) {
    for char in str.chars() {
        out.push(char as u8);
    }
}

fn encode_i32(int: i32, out: &mut Vec<u8>) {
    let mut copy = int;

    for _ in 0..2 {
        out.push((copy & 0xff).try_into().unwrap());
        copy >>= 8;
    }
}

impl Compressor {
    fn new() -> Self {
        Self {
            seen_strs: HashMap::new(),
            current_id: 0,
            current_string_id: 0,
            nodes: vec![],
            data_nodes: vec![],
        }
    }

    fn encode(&self) -> (Vec<u8>, Vec<u8>, String) {
        let mut out = Vec::new();
        let mut strings = Vec::new();

        encode_string("GAT0", &mut strings);
        encode_i32(self.seen_strs.keys().len() as i32, &mut strings);
        for key in self.seen_strs.keys() {
            encode_i32(key.len() as i32, &mut strings);
            encode_string(key.as_str(), &mut strings);
        }

        encode_string("GAT0", &mut out);
        encode_i32(self.nodes.len() as i32, &mut out);
        encode_i32(self.data_nodes.len() as i32, &mut out);
        for node in self.nodes.iter() {
            node.encode(&mut out);
        }

        for node in self.data_nodes.iter() {
            node.encode(&mut out);
        }

        let mut hasher = Sha1::new();
        hasher.input(&out);
        let result_str = hasher.result_str();
        // hasher.result(&mut out);

        (out, strings, result_str)
    }

    fn compress_tree(&mut self, node: &Node, depth: i32, source: &[u8]) -> i32 {
        self.current_id += 1;

        self.current_string_id += 1;
        let kind_id = self.current_string_id;
        /*self.seen_strs
        .insert(node.kind().to_string(), self.current_string_id);*/
        let text = node.utf8_text(source);
        if node.child_count() == 0 {
            self.current_string_id += 1;
            let text_id = self.current_string_id;
            self.seen_strs
                .insert(text.unwrap().to_string(), self.current_string_id);

            let new_root = CompressTreeDataNode {
                id: self.current_id,
                kind: kind_id,
                text: text_id,
            };

            self.data_nodes.push(new_root);

            return self.current_id.clone();
        }

        let mut current_root = CompressTreeNode {
            id: self.current_id,
            kind: kind_id,
            children: vec![],
        };

        for child_idx in 0..node.child_count() {
            let child = node.child(child_idx).unwrap();
            let child_id = self.compress_tree(&child, depth + 1, source);
            current_root.children.push(child_id);
        }

        self.nodes.push(current_root);

        self.current_id.clone()
    }
}

fn main() {
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_rust::LANGUAGE.into())
        .expect("Error loading Rust grammar");

    let mut file = File::open("../fake_repo/main.rs").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let source_code = contents.as_str(); // "let x = 1;";
    let tree = parser.parse(source_code, None).unwrap();
    let root_node = tree.root_node();

    // let example = File::create("tmp.txt").unwrap();
    //let raw_fd = example.as_raw_fd();
    //let cursor = tree.walk();

    // basically compress types at start of file, creating a generic ast filesystem
    // which will be used by the diff mechanism
    print_tree(&root_node, 0, source_code.as_bytes());

    let mut compressor = Compressor::new();
    compressor.compress_tree(&root_node, 0, source_code.as_bytes());
    let (result, strings, name) = compressor.encode();
    let mut file = File::create(name + ".gat").unwrap();
    file.write_all(&result).unwrap();

    let mut file = File::create("strings.gat").unwrap();
    file.write_all(&strings).unwrap();
}
