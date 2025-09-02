use crypto::digest::Digest;
use crypto::sha1::Sha1;
use std::any::Any;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::os::fd::AsRawFd;
use tree_sitter::{Node, Parser, Query, Tree};
use varint_rs::VarintWriter;

// for all i32 we need to really use a varint probably, eventually
struct CompressTreeDataNode {
    id: i32,
    kind: i32,
    text: i32,
}

impl CompressTreeDataNode {
    fn encode<W: Write>(&self, writer: &mut W) {
        encode_i32(self.id, writer);
        encode_i32(self.kind, writer);
    }

    fn encode_data<W: Write>(&self, writer: &mut W) {
        encode_i32(self.id, writer);
        encode_i32(self.text, writer);
    }
}

// writing to a file involves knowing the length here
struct CompressTreeNode {
    id: i32,
    kind: i32,
    children: Vec<i32>, // ids
}

impl CompressTreeNode {
    fn encode<W: Write + Seek>(&self, writer: &mut W) {
        encode_i32(self.id, writer);
        encode_i32(self.kind, writer);
        // this is a tad naive but simple enough
        encode_i32(self.children.len() as i32, writer);
        for child in self.children.iter() {
            encode_i32(*child, writer);
        }
    }
}

pub struct Compressor {
    seen_strs: HashMap<String, i32>,
    seen_types: HashMap<String, i32>,
    current_id: i32,
    current_kind_id: i32,
    current_string_id: i32,
    nodes: Vec<CompressTreeNode>,
    data_nodes: Vec<CompressTreeDataNode>,
}

fn encode_string<W: Write>(str: &str, writer: &mut W) {
    writer.write(str.as_bytes()).unwrap();
}

fn encode_i32<W: Write>(int: i32, writer: &mut W) {
    writer.write_i32_varint(int).unwrap();
}

fn encode_string_vec<W: Write>(vec: Vec<String>, writer: &mut W) {
    encode_i32(vec.len() as i32, writer);
    for key in vec {
        encode_i32(key.len() as i32, writer);
        encode_string(key.as_str(), writer);
    }
}

fn encode_string_hashmap_keys<W: Write, T>(hash_map: HashMap<String, T>, writer: &mut W) {
    encode_i32(hash_map.keys().len() as i32, writer);
    for key in hash_map.keys() {
        encode_i32(key.len() as i32, writer);
        encode_string(key.as_str(), writer);
    }
}

impl Compressor {
    pub fn new() -> Self {
        Self {
            seen_types: HashMap::new(),
            seen_strs: HashMap::new(),
            current_id: 0,
            current_kind_id: 0,
            current_string_id: 0,
            nodes: vec![],
            data_nodes: vec![],
        }
    }

    pub fn encode_to_filesystem(&self, path: String) {
        let mut node_file = File::create(path.clone() + "/node_file.gat").unwrap();
        self.encode_node_file(&mut node_file);

        let mut strings_file = File::create(path.clone() + "/strings.gat").unwrap();
        self.encode_strings(&mut strings_file);

        let mut types_file = File::create(path.clone() + "/types.gat").unwrap();
        self.encode_types(&mut types_file);

        let mut data_file = File::create(path.clone() + "/data.gat").unwrap();
        self.encode_data_file(&mut data_file);
    }

    pub fn compress_tree(&mut self, node: &Node, depth: i32, source: &[u8]) -> i32 {
        self.current_id += 1;

        self.current_kind_id += 1;
        let kind_id = self.current_string_id;
        self.seen_types
            .insert(node.kind().to_string(), self.current_kind_id);
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

    fn encode_types<W: Write + Seek>(&self, writer: &mut W) {
        encode_string("GAT0", writer);
        encode_string_hashmap_keys(self.seen_types.to_owned(), writer);
    }

    fn encode_strings<W: Write + Seek>(&self, writer: &mut W) {
        encode_string("GAT0", writer);
        encode_string_hashmap_keys(self.seen_strs.to_owned(), writer);
    }

    fn encode_data_file<W: Write + Seek>(&self, writer: &mut W) {
        encode_string("GAT0", writer);
        encode_i32(self.nodes.len() as i32, writer);
        encode_i32(self.data_nodes.len() as i32, writer);
        // for node in self.nodes.iter() {
        //     node.encode(writer);
        // }

        for node in self.data_nodes.iter() {
            node.encode_data(writer);
        }
    }

    fn encode_node_file<W: Write + Seek>(&self, writer: &mut W) {
        encode_string("GAT0", writer);
        encode_i32(self.nodes.len() as i32, writer);
        encode_i32(self.data_nodes.len() as i32, writer);
        for node in self.nodes.iter() {
            node.encode(writer);
        }

        for node in self.data_nodes.iter() {
            node.encode(writer);
        }
    }
}