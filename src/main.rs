use std::env;
use std::path::PathBuf;
use std::path;
use std::io::BufWriter;
use std::fs::File;
use pdf_extract::*;

fn main() {
    let bytes = std::fs::read("./test.pdf").unwrap();
    let out = pdf_extract::extract_text_from_mem(&bytes).unwrap();
    // assert!(out.contains("This is a small demonstration"));
    println!("{}", out);
}
