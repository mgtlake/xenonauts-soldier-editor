use hex_literal::hex;
use nom::{
    bytes::complete::{tag, take, take_until},
    multi::length_data,
    number::complete::{le_u16, le_u32, le_u8},
    sequence::{delimited, tuple},
    IResult,
};

#[derive(Debug)]
pub struct Soldier<'a> {
    id: u32,
    nationality: &'a [u8],
    name: &'a [u8],
    race: &'a [u8],
    face_number: u32,
    nation: &'a [u8],
    stats: SoldierStats,
    xp: u32,
    age: u16,
    regiment: &'a [u8],
    experience: &'a [u8],
    carrier: &'a [u8],
    gender: u8,
}

#[derive(Debug, Clone)]
pub struct SoldierStats {
    time_units_current: u32,
    health_current: u32,
    strength_current: u32,
    accuracy_current: u32,
    reflexes_current: u32,
    bravery_current: u32,
    time_units_original: u32,
    health_original: u32,
    strength_original: u32,
    accuracy_original: u32,
    reflexes_original: u32,
    bravery_original: u32,
}

// M A R K 7 NULL NULL NULL S o l d i e r
pub const SOLDIER_START: &[u8] = hex!("4D 41 52 4B 07 00 00 00 53 6F 6C 64 69 65 72").as_slice();

// M A R K 8 NULL NULL NULL S o l d i e r 2
const SOLDIER_END: &[u8] = hex!("4D 41 52 4B 08 00 00 00 53 6F 6C 64 69 65 72 32").as_slice();

pub fn parse_soldier(input: &[u8]) -> IResult<&[u8], Soldier> {
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
            _unknown_number,         // TODO figure this out
            _another_unknown_number, // TODO figure this out
            gender,
            _,
        ),
    ) = delimited(
        tag(SOLDIER_START),
        tuple((
            le_u32,
            length_data(le_u32),
            length_data(le_u32),
            length_data(le_u32),
            le_u32,
            length_data(le_u32),
            parse_soldier_stats,
            le_u32,
            take(38 as u32),
            le_u16, // TODO find correct format - this is right length
            length_data(le_u32),
            length_data(le_u32),
            take(4 as u32),
            length_data(le_u32),
            le_u32,
            le_u32,
            le_u8,
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
            race,
            face_number,
            nation,
            stats,
            xp,
            age,
            regiment,
            experience,
            carrier,
            gender,
        },
    ))
}

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
    fn it_parses_soldier() {
        let filepath: PathBuf = [env!("CARGO_MANIFEST_DIR"), "tests", "single_soldier.sav"]
            .iter()
            .collect();
        let file = fs::read(filepath).unwrap();

        let (_, soldier) = parse_soldier(&file).unwrap();
        assert_eq!(soldier.id, 23);
        assert_eq!(soldier.nationality, b"Japanese");
        assert_eq!(soldier.name, b"Ruri Yasuda");
        assert_eq!(soldier.race, b"asi");
        assert_eq!(soldier.face_number, 3);
        assert_eq!(soldier.nation, b"japan");
        assert_eq!(soldier.xp, 9);
        assert_eq!(soldier.regiment, b"regiment.japan1");
        assert_eq!(soldier.experience, b"experience.none");
        assert_eq!(soldier.carrier, b"Charlie - 1/13");
        assert_eq!(soldier.gender, 0);

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
}
