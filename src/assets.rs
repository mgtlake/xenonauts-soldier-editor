use std::{collections::HashSet, error::Error, fs, path::PathBuf};

use regex::Regex;
use roxmltree::Node;
use rust_search::{FilterExt, SearchBuilder};

use crate::soldier::Gender;

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

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct FacePath {
    pub race: String,
    pub id: u32,
    pub path: PathBuf,
}

#[derive(Clone, Debug)]
pub struct Assets {
    pub asset_path: PathBuf,
    pub all_strings: Vec<KeyedString>,
    pub regiment_names: Vec<KeyedString>,
    pub experience_names: Vec<KeyedString>,
    pub flags: Vec<KeyedPath>,
    pub male_faces: Vec<FacePath>,
    pub female_faces: Vec<FacePath>,
}

impl Assets {
    pub fn new(asset_path: PathBuf) -> Self {
        let string_files = SearchBuilder::default()
            .location(asset_path.clone())
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

        let flags = SearchBuilder::default()
            .location(asset_path.clone())
            .search_input("flags")
            .custom_filter(|dir| dir.metadata().unwrap().is_dir())
            .build()
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
            .collect();

        let male_faces = fetch_faces(&asset_path, "soldierimages");
        let female_faces = fetch_faces(&asset_path, "soldierimagesfemale");

        Assets {
            asset_path,
            all_strings,
            regiment_names,
            experience_names,
            flags,
            male_faces,
            female_faces,
        }
    }

    pub fn string_by_key(&self, s: &str) -> Option<&KeyedString> {
        self.all_strings.iter().filter(|ks| ks.key == s).last()
    }

    pub fn get_face(&self, race: &Vec<u8>, id: u32, gender: Gender) -> Option<&FacePath> {
        let faces = match gender {
            Gender::Male => &self.male_faces,
            Gender::Female => &self.female_faces,
        };
        faces
            .iter()
            .filter(|fp| fp.race.as_bytes() == race && fp.id == id)
            .last()
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

fn fetch_faces(asset_path: &PathBuf, image_folder_name: &str) -> Vec<FacePath> {
    SearchBuilder::default()
        .location(asset_path)
        .search_input(image_folder_name)
        .custom_filter(|dir| dir.metadata().unwrap().is_dir())
        .build()
        .flat_map(|image_dir| {
            SearchBuilder::default()
                .location(image_dir)
                .search_input("faces")
                .custom_filter(|dir| dir.metadata().unwrap().is_dir())
                .build()
        })
        .filter_map(|face_dir| fs::read_dir(face_dir).ok())
        .flatten()
        .filter_map(|res| res.ok())
        .filter_map(|file| {
            let filename = file.file_name().into_string().unwrap();
            let re = Regex::new(r"(?<race>[A-Za-z]+)(?<id>\d+).*\.png$").unwrap();
            re.captures(&filename).map(|caps| FacePath {
                race: caps["race"].to_string(),
                id: caps["id"].parse::<u32>().unwrap(),
                path: file.path(),
            })
        })
        .collect()
}
