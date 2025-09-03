use std::fs;
use crate::compression;
use crate::compression::Compressor;
use std::fs::File;
use std::io::{Read, Seek, Write};
use std::os::fd::AsRawFd;
use std::path::PathBuf;
use tree_sitter::{Language, Parser};

pub struct Vfs {
    path_buf: PathBuf,
    objects_path: PathBuf,
    compressor: Compressor,
}

impl Vfs {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path_buf: path.clone(),
            objects_path: path.clone().join("objects"),
            compressor: Compressor::new(),
        }
    }

    pub fn encode_strings(&self) {
        fs::create_dir_all(self.path_buf.clone()).unwrap();
        self.compressor.encode_strings_only(self.path_buf.clone())
    }

    pub fn encode_file(&mut self, full_file_path: PathBuf, extension: &str) {
        let mut parser = Parser::new();

        let def_lang = match extension {
            "rs" => "rust",
            "cpp" => "cpp",
            "cc" => "cpp",
            "h" => "h",
            "hpp" => "h",
            "py" => "python",
            "js" => "js",
            _ => "",
        };

        if def_lang == "" {
            // skip
            // println!("skipping: {}", full_file_path.display());
            return;
        }

        println!("encoding: {}", full_file_path.display());

        let language = match def_lang {
            "rust" => &tree_sitter_rust::LANGUAGE.into(),
            "cpp" => &tree_sitter_cpp::LANGUAGE.into(),
            "python" => &tree_sitter_python::LANGUAGE.into(),
            "js" => &tree_sitter_javascript::LANGUAGE.into(),
            _ => panic!("Unsupported language"),
        };

        parser
            .set_language(language)
            .expect("Error loading Rust grammar");

        let mut file = File::open(full_file_path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let source_code = contents.as_str(); // "let x = 1;";
        let tree = parser.parse(source_code, None).unwrap();
        let root_node = tree.root_node();

        fs::create_dir_all(self.objects_path.clone()).unwrap();

        self.compressor.encode_file_only(
            self.objects_path.clone(),
            &root_node,
            source_code.as_bytes(),
        );
    }
}
