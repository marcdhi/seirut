use std::{fs::File, process::exit};
use xml::{EventReader, reader::XmlEvent};

fn main() {
    let file_path = "docs.gl/gl4/glClear.xhtml";

    let file = File::open(file_path).unwrap_or_else(|err| {
        eprintln!("Error: could not read file {file_path}: {err}");
        exit(1)
    });

    let er = EventReader::new(file);
    let mut content = String::new();

    for event in er.into_iter(){

        let event = event.unwrap_or_else(|err| {
            eprintln!("ERROR: Could not read the next event : {err}");
            exit(1);
        });

        if let XmlEvent::Characters(text) = event {
            content.push_str(&text);
            // println!("{text:?}");
        };


    }

    println!("{content:?}");


}
 