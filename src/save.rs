use nom::{bytes::complete::take_until, IResult};

use crate::soldier::{Soldier, SOLDIER_START};

pub struct Save<'a> {
    before_soldiers: &'a [u8],
    soldiers: Vec<Soldier>,
}

pub fn parse_save(input: &[u8]) -> IResult<&[u8], Save> {
    let (unparsed, parsed) = take_until(SOLDIER_START)(input)?;
    IResult::Ok((
        unparsed,
        Save {
            before_soldiers: parsed,
            soldiers: Vec::new(),
        },
    ))
}
