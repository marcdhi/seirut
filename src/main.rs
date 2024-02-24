use std::{fs, process::exit};

fn main() {
    let file_path = "docs.gl/gl4/glClear.xhtml";
    let contents = fs::read_to_string(file_path).unwrap_or_else(|err| {
        eprintln!("Error: could not read file {file_path}: {err}");
        exit(1)
    });
    println!("File contents: {}", contents);
    println!("Length of the {file_path} is {length}", length = contents.len());
}
