use crate::interface::{Interface, ParentKind};
use quick_xml::de::from_str;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::PathBuf;

pub fn read_xml(str: &str) -> Result<Vec<Interface>, quick_xml::DeError> {
    from_str(replace_colons(str).as_str())
}

pub fn read_xml_file(path: PathBuf) -> Result<Vec<Interface>, quick_xml::DeError> {
    let contents = fs::read_to_string(path).expect("Should have been able to read the file");
    read_xml(contents.as_str())
}

fn replace_colons(colon_string: &str) -> String {
    let re = Regex::new(r"<([\/]?)(\w+):(\w+)\b").unwrap();
    let replaced = re.replace_all(colon_string, "<$1$2-$3").to_string();
    replaced
}

pub fn post_process_interface(interfaces: &mut [Interface]) {
    let mut helper = HashMap::new();
    for (idx, i) in interfaces.iter().enumerate() {
        if let Some(parent) = &i.link.parent {
            for j in interfaces.iter() {
                if j.name == *parent && j.bond.is_some() {
                    helper.insert(idx, Some(ParentKind::Bond));
                }
            }
        }
    }
    for (_, (k, v)) in helper.iter().enumerate() {
        if let Some(ifc) = interfaces.get_mut(*k) {
            ifc.link.kind = v.clone();
        }
    }
}

pub async fn read(paths: Vec<String>) -> Result<Vec<Interface>, io::Error> {
    let mut interfaces: Vec<Interface> = vec![];
    for path in paths {
        let path: PathBuf = path.into();
        let mut new_interfaces: Vec<Interface> = if path.is_dir() {
            fs::read_dir(path)?
                .filter(|r| !r.as_ref().unwrap().path().is_dir())
                .flat_map(|res| res.map(|e| read_xml_file(e.path()).unwrap()).unwrap())
                .collect::<Vec<_>>()
        } else {
            read_xml_file(path).unwrap()
        };
        interfaces.append(&mut new_interfaces);
    }
    post_process_interface(&mut interfaces);
    Ok(interfaces)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bond_option_fail_over_mac() {
        let xml = r##"
            <interface>
                <name>bond0</name>
                <bond>
                    <mode>active-backup</mode>
                    <fail-over-mac>none</fail-over-mac>
                    <slaves>
                        <slave><device>en0</device></slave>
                    </slaves>
                </bond>
            </interface>
            "##;
        let ifc = read_xml(xml).unwrap().pop().unwrap();
        assert!(ifc.bond.is_some());
        let bond = ifc.bond.unwrap();
        assert_eq!(
            bond.fail_over_mac,
            Some(crate::interface::FailOverMac::None)
        );
    }
}
