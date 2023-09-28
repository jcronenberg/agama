use crate::interface::Interface;
use quick_xml::de::from_str;
use regex::Regex;
use std::fs;
use std::io;
use std::path::PathBuf;

pub fn read_xml(path: PathBuf) -> Result<Vec<Interface>, quick_xml::DeError> {
    let contents = fs::read_to_string(path).expect("Should have been able to read the file");
    // TODO better error handling when xml parsing failed
    from_str(replace_colons(contents).as_str())
}

fn replace_colons(colon_string: String) -> String {
    let re = Regex::new(r"<([\/]?)(\w+):(\w+)\b").unwrap();
    let replaced = re
        .replace_all(colon_string.as_str(), "<$1$2-$3")
        .to_string();
    replaced
}

pub async fn read(path: PathBuf) -> Result<Vec<Interface>, io::Error> {
    let interfaces: Vec<Interface> = if path.is_dir() {
        fs::read_dir(path)?
            .flat_map(|res| res.map(|e| read_xml(e.path()).unwrap()).unwrap())
            .collect::<Vec<_>>()
    } else {
        read_xml(path).unwrap()
    };
    Ok(interfaces)
}