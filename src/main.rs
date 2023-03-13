#![allow(non_snake_case)]

use std::process::exit;
use std::time::SystemTime;

use compiler::Compiler;

pub mod module;
pub mod ast;
pub mod resolver;
pub mod compiler;
pub mod backend;

fn main() {
    let sourcePathVec = vec!["examples/source.txt".to_owned()];
    let start = SystemTime::now();
    let compiler = Compiler::new(None, sourcePathVec);

    if let Some(result) = compiler.getCompiledResult() {
        let end = SystemTime::now();
        println!("Compilation completed in {}ms", end.duration_since(start).unwrap().as_millis());

        // todo - handle compiled module
    } else {
        exit(-1);
    }
}
