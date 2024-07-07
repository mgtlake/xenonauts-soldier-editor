use hex_literal::hex;
use nom::{bytes::complete::take_until, IResult};

pub struct Soldier {}

// M A R K 7 NULL NULL NULL S o l d i e r
pub const SOLDIER_START: &[u8] = hex!("4D 41 52 4B 07 00 00 00 53 6F 6C 64 69 65 72").as_slice();

pub fn parse_soldier(input: &[u8]) -> IResult<&[u8], Soldier> {
    let (unparsed, _parsed) = take_until(SOLDIER_START)(input)?;
    IResult::Ok((unparsed, Soldier {}))
}
