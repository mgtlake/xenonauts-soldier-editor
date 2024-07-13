use hex_literal::hex;
use nom::{
    bytes::complete::{tag, take, take_until},
    combinator::map_res,
    multi::length_data,
    number::complete::{float, le_f32, le_u16, le_u32, le_u8},
    sequence::{delimited, tuple},
    IResult,
};

// M A R K 7 NULL NULL NULL S o l d i e r
pub const SOLDIER_START: &[u8] = hex!("4D 41 52 4B 07 00 00 00 53 6F 6C 64 69 65 72").as_slice();

// M A R K 8 NULL NULL NULL S o l d i e r 2
const SOLDIER_END: &[u8] = hex!("4D 41 52 4B 08 00 00 00 53 6F 6C 64 69 65 72 32").as_slice();

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Gender {
    Female = 0,
    Male = 1,
}

impl std::fmt::Display for Gender {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Gender::Female => "Female",
                Gender::Male => "Male",
            }
        )
    }
}

#[derive(Debug)]
pub struct Soldier {
    pub id: u32,
    pub nationality: String,
    pub name: String,
    pub race: Vec<u8>,
    pub face_number: u32,
    pub nation: Vec<u8>,
    pub stats: SoldierStats,
    pub xp: u32,
    pub age: f32,
    pub regiment: Vec<u8>,
    pub experience: Vec<u8>,
    pub carrier: Vec<u8>,
    unknown_number: u32,
    another_unknown_number: u32,
    pub gender: Gender,
    remaining_bytes: Vec<u8>,
}

impl Soldier {
    pub fn serialise(&self) -> Vec<u8> {
        [
            SOLDIER_START,
            &self.id.to_le_bytes(),
            &(self.nationality.len() as u32).to_le_bytes(),
            &self.nationality.clone().into_bytes(),
            &(self.name.len() as u32).to_le_bytes(),
            &self.name.clone().into_bytes(),
            &(self.race.len() as u32).to_le_bytes(),
            &self.race,
            &self.face_number.to_le_bytes(),
            &(self.nation.len() as u32).to_le_bytes(),
            &self.nation,
            &self.stats.serialise(),
            &self.xp.to_le_bytes(),
            &[b'\0'; 36], // TODO replace with parsed data
            &self.age.to_le_bytes(),
            &(self.regiment.len() as u32).to_le_bytes(),
            &self.regiment,
            &(self.experience.len() as u32).to_le_bytes(),
            &self.experience,
            &[b'\0'; 4], // TODO replace with parsed data
            &(self.carrier.len() as u32).to_le_bytes(),
            &self.carrier,
            &self.unknown_number.to_le_bytes(),
            &self.another_unknown_number.to_le_bytes(),
            &[self.gender as u8],
            &self.remaining_bytes,
            SOLDIER_END,
        ]
        .concat()
    }
}

pub fn parse_soldier(input: &[u8]) -> IResult<&[u8], Soldier> {
    let parse_string = |x: &[u8]| String::from_utf8(x.to_vec());
    let parse_gender = |x: u8| match x {
        0 => Ok(Gender::Female),
        1 => Ok(Gender::Male),
        _ => Err(()),
    };

    let (
        unparsed,
        (
            id,
            nationality,
            name,
            race,
            face_number,
            nation,
            stats,
            xp,
            _unknown, // TODO figure this out
            age,
            regiment,
            experience,
            _unknown_part_two, // TODO figure this out
            carrier,
            unknown_number,         // TODO figure this out
            another_unknown_number, // TODO figure this out
            gender,
            remaining_bytes,
        ),
    ) = delimited(
        tag(SOLDIER_START),
        tuple((
            le_u32,
            map_res(length_data(le_u32), parse_string),
            map_res(length_data(le_u32), parse_string),
            length_data(le_u32),
            le_u32,
            length_data(le_u32),
            parse_soldier_stats,
            le_u32,
            take(36 as u32),
            le_f32,
            length_data(le_u32),
            length_data(le_u32),
            take(4 as u32),
            length_data(le_u32),
            le_u32,
            le_u32,
            map_res(le_u8, parse_gender),
            take_until(SOLDIER_END),
        )),
        tag(SOLDIER_END),
    )(input)?;
    IResult::Ok((
        unparsed,
        Soldier {
            id,
            nationality,
            name,
            race: race.to_vec(),
            face_number,
            nation: nation.to_vec(),
            stats,
            xp,
            age,
            regiment: regiment.to_vec(),
            experience: experience.to_vec(),
            carrier: carrier.to_vec(),
            unknown_number,
            another_unknown_number,
            gender,
            remaining_bytes: remaining_bytes.to_vec(),
        },
    ))
}

#[derive(Debug, Clone)]
pub struct SoldierStats {
    pub time_units_current: u32,
    pub health_current: u32,
    pub strength_current: u32,
    pub accuracy_current: u32,
    pub reflexes_current: u32,
    pub bravery_current: u32,
    pub time_units_original: u32,
    pub health_original: u32,
    pub strength_original: u32,
    pub accuracy_original: u32,
    pub reflexes_original: u32,
    pub bravery_original: u32,
}

impl SoldierStats {
    fn serialise(&self) -> Vec<u8> {
        [
            self.time_units_current.to_le_bytes(),
            self.health_current.to_le_bytes(),
            self.strength_current.to_le_bytes(),
            self.accuracy_current.to_le_bytes(),
            self.reflexes_current.to_le_bytes(),
            self.bravery_current.to_le_bytes(),
            self.time_units_original.to_le_bytes(),
            self.health_original.to_le_bytes(),
            self.strength_original.to_le_bytes(),
            self.accuracy_original.to_le_bytes(),
            self.reflexes_original.to_le_bytes(),
            self.bravery_original.to_le_bytes(),
        ]
        .concat()
    }
}

// fn parse_soldier_stats(input: &[u8]) -> IResult<&[u8], SoldierStats, VerboseError<&[u8]>> {
fn parse_soldier_stats(input: &[u8]) -> IResult<&[u8], SoldierStats> {
    let (
        unparsed,
        (
            time_units_current,
            health_current,
            strength_current,
            accuracy_current,
            reflexes_current,
            bravery_current,
            time_units_original,
            health_original,
            strength_original,
            accuracy_original,
            reflexes_original,
            bravery_original,
        ),
    ) = tuple((
        le_u32, le_u32, le_u32, le_u32, le_u32, le_u32, le_u32, le_u32, le_u32, le_u32, le_u32,
        le_u32,
    ))(input)?;
    IResult::Ok((
        unparsed,
        SoldierStats {
            time_units_current,
            health_current,
            strength_current,
            accuracy_current,
            reflexes_current,
            bravery_current,
            time_units_original,
            health_original,
            strength_original,
            accuracy_original,
            reflexes_original,
            bravery_original,
        },
    ))
}

#[cfg(test)]
mod tests {
    use std::{fs, path::PathBuf};

    use super::*;

    #[test]
    fn it_parses_stats() {
        let input = [
            hex!("36 00 00 00 37 00 00 00 31 00 00 00 43 00 00 00"),
            hex!("3F 00 00 00 3B 00 00 00 36 00 00 00 37 00 00 00"),
            hex!("31 00 00 00 43 00 00 00 3F 00 00 00 3B 00 00 00"),
        ]
        .concat();

        let (_, stats) = parse_soldier_stats(&input).unwrap();
        assert_eq!(stats.time_units_current, 54);
        assert_eq!(stats.health_current, 55);
        assert_eq!(stats.strength_current, 49);
        assert_eq!(stats.accuracy_current, 67);
        assert_eq!(stats.reflexes_current, 63);
        assert_eq!(stats.bravery_current, 59);
        assert_eq!(stats.time_units_original, 54);
        assert_eq!(stats.health_original, 55);
        assert_eq!(stats.strength_original, 49);
        assert_eq!(stats.accuracy_original, 67);
        assert_eq!(stats.reflexes_original, 63);
        assert_eq!(stats.bravery_original, 59);
    }

    #[test]
    fn it_parses_stats_round_trip() {
        let input = [
            hex!("36 00 00 00 37 00 00 00 31 00 00 00 43 00 00 00"),
            hex!("3F 00 00 00 3B 00 00 00 36 00 00 00 37 00 00 00"),
            hex!("31 00 00 00 43 00 00 00 3F 00 00 00 3B 00 00 00"),
        ]
        .concat();

        let (_, stats) = parse_soldier_stats(&input).unwrap();
        let output = stats.serialise();
        assert_eq!(input, output);
    }

    #[test]
    fn it_parses_soldier() {
        let filepath: PathBuf = [env!("CARGO_MANIFEST_DIR"), "tests", "single_soldier.sav"]
            .iter()
            .collect();
        let file = fs::read(filepath).unwrap();

        let (_, soldier) = parse_soldier(&file).unwrap();
        assert_eq!(soldier.id, 23);
        assert_eq!(soldier.nationality, "Japanese");
        assert_eq!(soldier.name, "Ruri Yasuda");
        assert_eq!(soldier.race, b"asi");
        assert_eq!(soldier.face_number, 3);
        assert_eq!(soldier.nation, b"japan");
        assert_eq!(soldier.xp, 9);
        assert_eq!(soldier.regiment, b"regiment.japan1");
        assert_eq!(soldier.experience, b"experience.none");
        assert_eq!(soldier.carrier, b"Charlie - 1/13");
        assert_eq!(soldier.gender, Gender::Female);

        assert_eq!(soldier.stats.time_units_current, 54);
        assert_eq!(soldier.stats.health_current, 55);
        assert_eq!(soldier.stats.strength_current, 49);
        assert_eq!(soldier.stats.accuracy_current, 67);
        assert_eq!(soldier.stats.reflexes_current, 63);
        assert_eq!(soldier.stats.bravery_current, 59);
        assert_eq!(soldier.stats.time_units_original, 54);
        assert_eq!(soldier.stats.health_original, 55);
        assert_eq!(soldier.stats.strength_original, 49);
        assert_eq!(soldier.stats.accuracy_original, 67);
        assert_eq!(soldier.stats.reflexes_original, 63);
        assert_eq!(soldier.stats.bravery_original, 59);
    }

    #[test]
    fn it_parses_soldier_round_trip() {
        let filepath: PathBuf = [env!("CARGO_MANIFEST_DIR"), "tests", "single_soldier.sav"]
            .iter()
            .collect();
        let file = fs::read(filepath).unwrap();

        let (_, soldier) = parse_soldier(&file).unwrap();
        let output = soldier.serialise();
        assert_eq!(file, output);
    }
}
