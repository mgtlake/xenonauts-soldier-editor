use hex_literal::hex;
use nom::{
    bytes::complete::{is_a, tag, take, take_until, take_while},
    multi::{length_data, length_value},
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
            _unknown_number, // TODO figure this out
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
    // println!("{:x?}", Soldier {
    //     id,
    //     nationality,
    //     name,
    //     race,
    //     face_number,
    //     nation,
    //     stats: stats.clone(),
    //     xp,
    //     age,
    //     regiment,
    //     experience,
    //     carrier,
    //     gender
    // });
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
            gender
        },
    ))
}

fn parse_soldier_stats(input: &[u8]) -> IResult<&[u8], SoldierStats> {
    println!("Parsing soldier stats");
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
    println!("Parsed soldier stats");
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
