use std::{collections::HashMap, fs::{self, read, File}, hash::Hash, io, path::Path, process::exit};
use xml::{EventReader, reader::XmlEvent};

fn index_document(_doc_content: &str) -> HashMap<String, usize> {
    todo!("not done yet!")
}

fn read_entire_xml_file<P: AsRef<Path>>(file_path: P) -> io::Result<String> {
    let file = File::open(file_path)?;
    let er = EventReader::new(file);
    let mut content = String::new();
    
    for event in er.into_iter(){
        if let XmlEvent::Characters(text) = event.unwrap() {
            content.push_str(&text);
        };
    }
    Ok(content) 
}

fn main() -> io::Result<()>{
    let dir_path = "docs.gl/gl4";

    let dir = fs::read_dir(dir_path);

    for file in dir.unwrap() {
        let file_path = file.unwrap().path();
        let content = read_entire_xml_file(&file_path).unwrap();
        println!("{file_path:?} => {size}", size = content.len());
    }

    Ok(())
    // println!("{content}", content = read_entire_xml_file(file_path).unwrap());
}
 