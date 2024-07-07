use hex_literal::hex;
use nom::{
    bytes::complete::{tag, take_until},
    sequence::delimited,
    IResult,
};

#[derive(Debug)]
pub struct Soldier {}

// M A R K 7 NULL NULL NULL S o l d i e r
pub const SOLDIER_START: &[u8] = hex!("4D 41 52 4B 07 00 00 00 53 6F 6C 64 69 65 72").as_slice();

// M A R K 8 NULL NULL NULL S o l d i e r 2
const SOLDIER_END: &[u8] = hex!("4D 41 52 4B 08 00 00 00 53 6F 6C 64 69 65 72 32").as_slice();

pub fn parse_soldier(input: &[u8]) -> IResult<&[u8], Soldier> {
    let (unparsed, _parsed) = delimited(
        tag(SOLDIER_START),
        take_until(SOLDIER_END),
        tag(SOLDIER_END),
    )(input)?;
    println!("Soldier parsed len {}", _parsed.len());
    IResult::Ok((unparsed, Soldier {}))
}
