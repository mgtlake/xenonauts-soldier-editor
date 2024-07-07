use hex_literal::hex;
use nom::{bytes::complete::tag, IResult};

pub struct Soldier {}

pub fn parse_soldier(input: &[u8]) -> IResult<&[u8], Soldier> {
    // M A R K 7 NULL NULL NULL S o l d i e r
    let solider_start_mark = hex!("4D 41 52 4B 07 00 00 00 53 6F 6C 64 69 65 72");
    let (unparsed, _parsed) = tag(solider_start_mark)(input)?;
    IResult::Ok((unparsed, Soldier {}))
}
