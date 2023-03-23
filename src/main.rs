#![allow(non_snake_case)]

use std::process::exit;
use std::time::SystemTime;
use log::info;

use compiler::Compiler;
use crate::backend::link::{checkLinkerPath, getExecutableExtension};

pub mod module;
pub mod ast;
pub mod resolver;
pub mod compiler;
pub mod backend;

fn main() {
    simple_logger::init().unwrap();
    
    checkLinkerPath();
    
    let sourcePathVec = vec!["examples/source.txt".to_owned()];
    let start = SystemTime::now();
    let compiler = Compiler::new(None, sourcePathVec);

    if let Some(module) = compiler.getCompiledResult() {
        std::fs::create_dir_all("output").expect("failed to create output directory");
        let outputPath = "output/output".to_owned() + getExecutableExtension();
        module.writeExecutable(&outputPath);

        let end = SystemTime::now();
        info!("Compilation completed in {}ms", end.duration_since(start).unwrap().as_millis());
        info!("Output written to {outputPath}.");
    } else {
        let end = SystemTime::now();
        info!("Compilation failed in {}ms", end.duration_since(start).unwrap().as_millis());
        exit(1);
    }
}
