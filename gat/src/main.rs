use crate::vfs::Vfs;
use clap::Parser as ClapParser;
use crypto::digest::Digest;
use crypto::sha1::Sha1;
use std::any::Any;
use std::fs::File;
use std::io::{Read, Seek, Write};
use std::os::fd::AsRawFd;
use std::path::PathBuf;
use tree_sitter::Parser;

mod compression;
mod print_tree;
mod vfs;

#[derive(ClapParser)]
struct Cli {
    path: std::path::PathBuf,
    language: String,
}

// for all i32 we need to really use a varint probably, eventually
// todo: InputEdit, tree.edit in tree-sitter
fn main() {
    let args = Cli::parse();

    let mut parser = Parser::new();

    let language = match args.language.as_str() {
        "rust" => &tree_sitter_rust::LANGUAGE.into(),
        "cpp" => &tree_sitter_cpp::LANGUAGE.into(),
        "python" => &tree_sitter_python::LANGUAGE.into(),
        "js" => &tree_sitter_javascript::LANGUAGE.into(),
        _ => panic!("Unsupported language"),
    };

    parser
        .set_language(language)
        .expect("Error loading Rust grammar");

    let mut vfs = Vfs::new(PathBuf::from(".gat"));
    vfs.encode_file(PathBuf::from(args.path), args.language.as_str());
    vfs.encode_strings();
}
