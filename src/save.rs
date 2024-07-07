use nom::{bytes::complete::take_until, multi::many0, IResult};

use crate::soldier::{parse_soldier, Soldier, SOLDIER_START};

#[derive(Debug)]
pub struct Save<'a> {
    pub before_soldiers: &'a [u8],
    pub soldiers: Vec<Soldier<'a>>,
    pub after_soldiers: &'a [u8],
}

pub fn parse_save(input: &[u8]) -> IResult<&[u8], Save> {
    let (unparsed, before_soldiers) = take_until(SOLDIER_START)(input)?;
    let (after_soldiers, soldiers) = many0(parse_soldier)(unparsed)?;
    println!("{:x?}", soldiers);
    IResult::Ok((
        unparsed,
        Save {
            before_soldiers,
            soldiers,
            after_soldiers,
        },
    ))
}
