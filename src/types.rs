use nom::{
    branch::alt,
    bytes::complete::tag_no_case,
    character::complete::multispace0,
    combinator::{map, opt},
    sequence::tuple,
    IResult,
};
use serde_derive::{Deserialize, Serialize};

use crate::common::{delimited_digit, len_as_u16, opt_signed};

//TODO: IMPLEMENT OTHERS TYPES
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum SqlType {
    Int(u16),
    UnsignedInt(u16),
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
        map(
            tuple((
                tag_no_case("int"),
                opt(delimited_digit),
                multispace0,
                opt_signed,
            )),
            |(_keyword, len, _, sign_info)| match sign_info {
                Some(sign_info) => match std::str::from_utf8(sign_info) {
                    Ok(signed) if signed == "signed" => {
                        SqlType::Int(len.map(len_as_u16).unwrap_or(32_u16))
                    }
                    Ok(unsigned) if unsigned == "unsigned" => {
                        SqlType::UnsignedInt(len.map(len_as_u16).unwrap_or(32_u16))
                    }
                    _ => unreachable!(),
                },
                None => SqlType::Int(len.map(len_as_u16).unwrap_or(1)),
            },
        ),
        map(tag_no_case("datetime"), |_| SqlType::DateTime),
        map(tag_no_case("date"), |_| SqlType::Date),
        map(
            tuple((tag_no_case("timestamp"), opt(delimited_digit), multispace0)),
            |_| SqlType::Timestamp,
        ),
        map(tag_no_case("text"), |_| SqlType::Text),
        map(
            tuple((tag_no_case("real"), multispace0, opt_signed)),
            |_| SqlType::Real,
        ),
        map(
            tuple((tag_no_case("float"), multispace0, opt_signed)),
            |_| SqlType::Float,
        ),
        map(tag_no_case("blob"), |_| SqlType::Blob),
        map(tag_no_case("uuid"), |_| SqlType::Uuid),
    ))(input)
}
