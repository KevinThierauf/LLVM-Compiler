#![allow(non_snake_case)]

extern crate core;

use crate::source::filepos::SourceFile;
use crate::source::parser::parse;

pub mod source;

fn main() {
    const EXAMPLE_PATH: &'static str = "examples/source.txt";

    match parse(SourceFile::new(EXAMPLE_PATH.into()).expect("failed to create SourceFile from path")) {
        Ok(symbols) => {
            println!("{:?}", symbols);
        }
        Err(err) => {
            println!("{}", err.getDisplayMessage())
        }
    }
}