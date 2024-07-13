use calamine::{open_workbook, Reader, Xlsx};
use roxmltree::Node;
use std::{collections::HashSet, error::Error, fs, path::PathBuf};

use rust_search::{FilterExt, SearchBuilder};

pub fn find_xenonauts_assets_folder() -> Option<PathBuf> {
    let paths_to_check: Vec<PathBuf> = [
        "C:\\Program Files (x86)\\GOG Galaxy\\Games\\Xenonauts\\assets".into(),
        "C:\\Program Files (x86)\\Steam\\steamapps\\common\\Xenonauts\\assets".into(),
        "~/.steam/steamapps/common/Xenonauts/assets".into(),
        "~/.local/share/Steam/common/Xenonauts/assets".into(),
    ]
    .to_vec();

    for path in paths_to_check {
        if let Ok(_) = fs::read_dir(path.clone()) {
            return Some(path);
        }
    }
    None
}

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct KeyedString {
    pub key: String,
    pub string: String,
}

impl std::fmt::Display for KeyedString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.key == self.string {
            write!(f, "{}", self.string,)
        } else {
            write!(f, "{} ({})", self.string, self.key,)
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct KeyedPath {
    pub key: String,
    pub path: PathBuf,
}

#[derive(Clone, Debug)]
pub struct Assets {
    pub all_strings: Vec<KeyedString>,
    pub regiment_names: Vec<KeyedString>,
    pub experience_names: Vec<KeyedString>,
    pub flags: Vec<KeyedPath>,
    pub male_faces: Vec<KeyedPath>,
    pub female_faces: Vec<KeyedPath>,
}

impl Assets {
    pub fn new(assset_folder: &PathBuf) -> Self {
        let string_files = SearchBuilder::default()
            .location(assset_folder)
            .search_input("strings.xml")
            .build()
            .collect::<Vec<String>>();

        let all_strings: Vec<KeyedString> = string_files
            .iter()
            .filter_map(|path| parse_strings(&path.into()).ok())
            .flatten()
            .collect();

        let regiment_names = all_strings
            .clone()
            .into_iter()
            .filter(|ks| ks.key.starts_with("regiment."))
            .collect();
        let experience_names = all_strings
            .clone()
            .into_iter()
            .filter(|ks| ks.key.starts_with("experience."))
            .collect();

        let flag_dirs = SearchBuilder::default()
            .location(assset_folder)
            .search_input("flags")
            .custom_filter(|dir| dir.metadata().unwrap().is_dir())
            .build()
            .collect::<Vec<String>>();
        let flags = flag_dirs
            .iter()
            .filter_map(|dir| fs::read_dir(dir).ok())
            .flatten()
            .filter_map(|r| {
                r.map(|f| KeyedPath {
                    key: f
                        .file_name()
                        .to_string_lossy()
                        .into_owned()
                        .split(".")
                        .next()
                        .unwrap()
                        .to_string(),
                    path: f.path(),
                })
                .ok()
            })
            .collect::<Vec<_>>();

        Assets {
            all_strings,
            regiment_names,
            experience_names,
            flags,
            male_faces: Vec::new(),
            female_faces: Vec::new(),
        }
    }

    pub fn string_by_key(&self, s: &str) -> Option<&KeyedString> {
        self.all_strings.iter().filter(|ks| ks.key == s).last()
    }
}

fn parse_strings(path: &PathBuf) -> Result<Vec<KeyedString>, Box<dyn Error>> {
    let f = fs::read_to_string(path)?;
    let doc = roxmltree::Document::parse(&f)?;
    let mut strings = doc
        .descendants()
        .filter(|n| n.has_tag_name("Row"))
        .filter_map(|row| parse_string(row))
        .collect::<Vec<KeyedString>>();

    strings.sort();
    strings.dedup();

    Ok(strings)
}

fn parse_string(row: Node) -> Option<KeyedString> {
    let key_cell = row.first_element_child()?;
    let key_data = key_cell.first_element_child()?;
    let key = key_data.first_child()?.text()?.to_string();

    let string_cell = row.last_element_child()?;
    let string_data = string_cell.first_element_child()?;
    let string = string_data.first_child()?.text()?.to_string();

    Some(KeyedString { key, string })
}
