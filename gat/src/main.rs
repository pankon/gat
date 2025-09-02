use crypto::digest::Digest;
use crypto::sha1::Sha1;
use std::any::Any;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::os::fd::AsRawFd;
use tree_sitter::{Node, Parser, Query, Tree};
use varint_rs::VarintWriter;
use clap::Parser as ClapParser;

mod print_tree;
mod compression;


#[derive(ClapParser)]
struct Cli {
    path: std::path::PathBuf,
    language: String,
}

// for all i32 we need to really use a varint probably, eventually
fn main() {
    let args = Cli::parse();

    let mut parser = Parser::new();

    let language = match args.language.as_str() {
        "rust" => &tree_sitter_rust::LANGUAGE.into(),
        _ => panic!("Unsupported language"),
    };

    parser
        .set_language(language)
        .expect("Error loading Rust grammar");

    let mut file = File::open(args.path).unwrap();
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
    print_tree::print_tree(&root_node, 0, source_code.as_bytes());

    let mut compressor = compression::Compressor::new();
    compressor.compress_tree(&root_node, 0, source_code.as_bytes());
    compressor.encode_to_filesystem(".".to_string());
}
