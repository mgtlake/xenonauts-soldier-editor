use nom::{bytes::complete::take_until, multi::many0, IResult};

use crate::soldier::{Soldier, SOLDIER_START, parse_soldier};

#[derive(Debug)]
pub struct Save<'a> {
    pub before_soldiers: &'a [u8],
    pub soldiers: Vec<Soldier>,
    pub after_soldiers: &'a [u8],
}

pub fn parse_save(input: &[u8]) -> IResult<&[u8], Save> {
    let (unparsed, parsed) = take_until(SOLDIER_START)(input)?;
    let (still_unparsed, soldiers) = many0(parse_soldier)(unparsed)?;
    IResult::Ok((
        unparsed,
        Save {
            before_soldiers: parsed,
            soldiers: soldiers,
            after_soldiers: still_unparsed,
        },
    ))
}
