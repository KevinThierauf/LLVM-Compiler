#![allow(non_snake_case)]

use std::time::SystemTime;

use crate::ast::AbstractSyntaxTree;
use crate::module::{Module, SourceFile};

pub mod module;
pub mod logger;
pub mod ast;
pub mod resolver;

fn main() {
    const EXAMPLE_PATH: &'static str = "examples/source.txt";

    let start = SystemTime::now();
    let result = Module::new(SourceFile::new(EXAMPLE_PATH.into()).expect("failed to create SourceFile from path"));
    let end = SystemTime::now();
    match result {
        Ok(module) => {
            match AbstractSyntaxTree::new(module) {
                Ok(ast) => {
                    println!("{:#?}", ast);
                }
                Err(err) => println!("{}", err.getDisplayMessage()),
            }
        }
        Err(err) => {
            println!("{}", err.getDisplayMessage())
        }
    }

    println!("Token parsing completed in {}ms", end.duration_since(start).unwrap().as_millis());
}
