use crate::vfs::Vfs;
use clap::Parser as ClapParser;
use crypto::digest::Digest;
use crypto::sha1::Sha1;
use glob::glob;
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
    glob_pattern: String,
}

// for all i32 we need to really use a varint probably, eventually
// todo: InputEdit, tree.edit in tree-sitter
fn main() {
    let args = Cli::parse();

    let mut vfs = Vfs::new(PathBuf::from(".gat"));
    for entry in glob(args.glob_pattern.as_str()).unwrap() {
        let entry_unwrapped = entry.unwrap();
        if entry_unwrapped.clone().to_str().unwrap().contains("/.") {
            continue;
        }

        let mut extension = "";
        if let Some(val) = entry_unwrapped.extension() {
            extension = val.to_str().unwrap();
        }

        vfs.encode_file(
            PathBuf::from(entry_unwrapped.clone()),
            extension,
        );

        // todo: this is still pretty naive, we need to have a priority heap where more used
        // strings have a lower number, obviously
        vfs.encode_strings();
    }
}
