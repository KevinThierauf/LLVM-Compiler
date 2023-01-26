#![allow(non_snake_case)]

extern crate core;

use std::time::SystemTime;
use crate::source::filepos::SourceFile;
use crate::source::parser::parse;

pub mod source;

fn main() {
    const EXAMPLE_PATH: &'static str = "examples/source.txt";

    let start = SystemTime::now();
    let result = parse(SourceFile::new(EXAMPLE_PATH.into()).expect("failed to create SourceFile from path"));
    let end = SystemTime::now();
    match result {
        Ok(symbols) => {
            println!("{:?}", symbols);
        }
        Err(err) => {
            println!("{}", err.getDisplayMessage())
        }
    }

    println!("Parsing completed in {}ms", end.duration_since(start).unwrap().as_millis());
}