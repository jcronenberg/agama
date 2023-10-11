use crate::interface::{Interface, ParentKind};
use quick_xml::de::from_str;
use regex::Regex;
use std::collections::HashMap;
use std::fs::{self, read_dir};
use std::path::{Path, PathBuf};

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

// https://stackoverflow.com/a/76820878
fn recurse_files(path: impl AsRef<Path>) -> std::io::Result<Vec<PathBuf>> {
    let mut buf = vec![];
    let entries = read_dir(path)?;

    for entry in entries {
        let entry = entry?;
        let meta = entry.metadata()?;

        if meta.is_dir() {
            let mut subdir = recurse_files(entry.path())?;
            buf.append(&mut subdir);
        }

        if meta.is_file() {
            buf.push(entry.path());
        }
    }

    Ok(buf)
}

pub fn read(paths: Vec<String>) -> Result<Vec<Interface>, anyhow::Error> {
    let mut interfaces: Vec<Interface> = vec![];
    for path in paths {
        let path: PathBuf = path.into();
        if path.is_dir() {
            let files = recurse_files(path)?;
            for file in files {
                interfaces.append(&mut read_xml_file(file)?);
            }
        } else {
            interfaces.append(&mut read_xml_file(path)?);
        }
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
