use nom::{bytes::complete::take_until, multi::many0, IResult};

use crate::soldier::{self, Soldier, SOLDIER_START};

#[derive(Debug)]
pub struct Save {
    pub before_soldiers: Vec<u8>,
    pub soldiers: Vec<Soldier>,
    pub after_soldiers: Vec<u8>,
}

impl Save {
    fn serialise(self) -> Vec<u8> {
        [
            self.before_soldiers,
            self.soldiers
                .iter()
                .flat_map(|soldier| soldier.serialise())
                .collect(),
            self.after_soldiers,
        ]
        .concat()
    }
}

pub fn parse_save(input: &[u8]) -> IResult<&[u8], Save> {
    let (unparsed, before_soldiers) = take_until(SOLDIER_START)(input)?;
    let (after_soldiers, soldiers) = many0(soldier::parse_soldier)(unparsed)?;
    IResult::Ok((
        unparsed,
        Save {
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
    fn it_parses_just_soldier() {
        let filepath: PathBuf = [env!("CARGO_MANIFEST_DIR"), "tests", "single_soldier.sav"]
            .iter()
            .collect();
        let file = fs::read(filepath).unwrap();

        let (_, save) = parse_save(&file).unwrap();
        assert_eq!(save.before_soldiers.len(), 0);
        assert_eq!(save.soldiers.len(), 1);
        assert_eq!(save.after_soldiers.len(), 0);
    }

    #[test]
    fn it_parses_just_soldier_round_trip() {
        let filepath: PathBuf = [env!("CARGO_MANIFEST_DIR"), "tests", "single_soldier.sav"]
            .iter()
            .collect();
        let file = fs::read(filepath).unwrap();

        let (_, save) = parse_save(&file).unwrap();
        let output = save.serialise();
        assert_eq!(file, output);
    }

    #[test]
    fn it_parses_full_save() {
        let filepath: PathBuf = [env!("CARGO_MANIFEST_DIR"), "tests", "full_save.sav"]
            .iter()
            .collect();
        let file = fs::read(filepath).unwrap();

        let (_, save) = parse_save(&file).unwrap();
        assert_eq!(save.before_soldiers.len(), 2081);
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
