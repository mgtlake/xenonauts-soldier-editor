use nom::{
    bytes::complete::{take, take_until},
    combinator::map_res,
    multi::{length_data, many0},
    number::complete::{le_u32, le_u8},
    sequence::tuple,
    IResult,
};

use crate::soldier::{self, Soldier, SOLDIER_START};

const FEATURE_GUARDS_END: &[u8] = "FeatureGuards2".as_bytes();

#[derive(Debug)]
pub struct Save {
    file_start: Vec<u8>,
    pub save_name: String,
    feature_guards: Vec<u8>,
    save_time: Vec<u8>,
    unknown: u32,
    pub iron_man: bool,
    pub before_soldiers: Vec<u8>,
    pub soldiers: Vec<Soldier>,
    pub after_soldiers: Vec<u8>,
}

impl Save {
    pub fn serialise(&self) -> Vec<u8> {
        [
            self.file_start.clone(),
            (self.save_name.len() as u32).to_le_bytes().to_vec(),
            self.save_name.clone().into_bytes(),
            self.feature_guards.clone(),
            FEATURE_GUARDS_END.to_vec(),
            (self.save_time.len() as u32).to_le_bytes().to_vec(),
            self.save_time.clone(),
            self.unknown.to_le_bytes().to_vec(),
            [self.iron_man as u8].to_vec(),
            self.before_soldiers.clone(),
            self.soldiers
                .iter()
                .flat_map(|soldier| soldier.serialise())
                .collect(),
            self.after_soldiers.clone(),
        ]
        .concat()
    }

    pub fn get_soldier(&self, id: u32) -> Option<&Soldier> {
        self.soldiers
            .iter()
            .filter(|soldier| soldier.id == id)
            .last()
    }

    pub fn get_soldier_mut(&mut self, id: u32) -> Option<&mut Soldier> {
        self.soldiers
            .iter_mut()
            .filter(|soldier| soldier.id == id)
            .last()
    }
}

pub fn parse_save(input: &[u8]) -> IResult<&[u8], Save> {
    let parse_string = |x: &[u8]| String::from_utf8(x.to_vec());
    let parse_iron_man_status = |x: u8| match x {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(()),
    };

    let (unparsed, (file_start, save_name, feature_guards, _, save_time, unknown, iron_man)) =
        tuple((
            take(8_u8),
            map_res(length_data(le_u32), parse_string),
            take_until(FEATURE_GUARDS_END),
            take(FEATURE_GUARDS_END.len()),
            length_data(le_u32),
            le_u32,
            map_res(le_u8, parse_iron_man_status),
        ))(input)?;
    let (unparsed, before_soldiers) = take_until(SOLDIER_START)(unparsed)?;
    let (after_soldiers, soldiers) = many0(soldier::parse_soldier)(unparsed)?;
    IResult::Ok((
        unparsed,
        Save {
            file_start: file_start.to_vec(),
            save_name,
            feature_guards: feature_guards.to_vec(),
            save_time: save_time.to_vec(),
            unknown,
            iron_man,
            before_soldiers: before_soldiers.to_vec(),
            soldiers,
            after_soldiers: after_soldiers.to_vec(),
        },
    ))
}

#[cfg(test)]
mod tests {
    use std::{fs, path::PathBuf};

    use super::*;

    #[test]
    fn it_parses_full_save() {
        let filepath: PathBuf = [env!("CARGO_MANIFEST_DIR"), "tests", "full_save.sav"]
            .iter()
            .collect();
        let file = fs::read(filepath).unwrap();

        let (_, save) = parse_save(&file).unwrap();
        assert_eq!(save.file_start.len(), 8);
        assert_eq!(save.save_name, "Iron Man (2024-07-06_20.46.00)");
        assert_eq!(save.feature_guards.len(), 416);
        assert_eq!(save.save_time, "2024-07-06_20.46.00".as_bytes());
        assert_eq!(save.iron_man, true);
        assert_eq!(save.before_soldiers.len(), 1581);
        assert_eq!(save.soldiers.len(), 22);
        assert_eq!(save.after_soldiers.len(), 23740);
    }

    #[test]
    fn it_parses_full_save_round_trip() {
        let filepath: PathBuf = [env!("CARGO_MANIFEST_DIR"), "tests", "full_save.sav"]
            .iter()
            .collect();
        let file = fs::read(filepath).unwrap();

        let (_, save) = parse_save(&file).unwrap();
        let output = save.serialise();
        assert_eq!(file, output);
    }
}
