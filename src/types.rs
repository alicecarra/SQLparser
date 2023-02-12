use nom::{
    branch::alt, bytes::complete::tag_no_case, character::complete::multispace0, combinator::map,
    sequence::tuple, IResult,
};
use serde_derive::{Deserialize, Serialize};

use crate::common::{delimited_digit, len_as_u16};

//TODO: IMPLEMENT OTHERS TYPES
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum SqlType {
    Int(Option<u8>),
    UnsignedInt(Option<u8>),
    Float,
    Real,
    Char(u16),
    VarChar(u16),
    Text,
    Bool,
    Blob,
    Uuid,
    Date,
    DateTime,
    Timestamp,
    Enum(Vec<Literal>),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum Literal {
    Null,
    Integer(i64),
    UnsignedInteger(u64),
    FixedPoint(Real),
    String(String),
    Blob(Vec<u8>),
    CurrentTime,
    CurrentDate,
    CurrentTimestamp,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct Real {
    pub integral: i32,
    pub fractional: i32,
}

// TODO: Aadd the rest
pub fn type_identifier(input: &[u8]) -> IResult<&[u8], SqlType> {
    alt((
        map(tag_no_case("bool"), |_| SqlType::Bool),
        map(
            tuple((tag_no_case("char"), delimited_digit, multispace0)),
            |(_keyword, len, _)| SqlType::Char(len_as_u16(len)),
        ),
        map(
            tuple((tag_no_case("varchar"), delimited_digit, multispace0)),
            |(_keyword, len, _)| SqlType::VarChar(len_as_u16(len)),
        ),
        map(tag_no_case("datetime"), |_| SqlType::DateTime),
        map(tag_no_case("date"), |_| SqlType::Date),
    ))(input)
}
