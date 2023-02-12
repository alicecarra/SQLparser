use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case, take_while1},
    character::{
        complete::{digit1, line_ending, multispace0, multispace1},
        is_alphanumeric,
    },
    combinator::{eof, map, not, opt, peek},
    sequence::{delimited, pair, preceded, terminated, tuple},
    IResult,
};
use serde_derive::{Deserialize, Serialize};

#[inline]
pub fn valid_identifier(input: &[u8]) -> IResult<&[u8], &[u8]> {
    preceded(not(peek(sql_keyword)), take_while1(is_sql_identifier))(input)
}

pub fn sql_identifier(input: &[u8]) -> IResult<&[u8], &[u8]> {
    alt((
        valid_identifier,
        delimited(tag("`"), take_while1(is_sql_identifier), tag("`")),
        delimited(tag("["), take_while1(is_sql_identifier), tag("]")),
    ))(input)
}
#[inline]
pub fn is_sql_identifier(chr: u8) -> bool {
    is_alphanumeric(chr) || chr == b'_' || chr == b'@'
}

#[inline]
fn opt_signed(input: &[u8]) -> IResult<&[u8], Option<&[u8]>> {
    opt(alt((tag_no_case("unsigned"), tag_no_case("signed"))))(input)
}

#[inline]
fn delimited_digit(i: &[u8]) -> IResult<&[u8], &[u8]> {
    delimited(tag("("), digit1, tag(")"))(i)
}

pub(crate) fn comma_separator(i: &[u8]) -> IResult<&[u8], &[u8]> {
    delimited(multispace0, tag(","), multispace0)(i)
}

#[inline]
fn len_as_u16(len: &[u8]) -> u16 {
    match std::str::from_utf8(len) {
        Ok(s) => match s.parse::<u16>() {
            Ok(v) => v,
            Err(e) => std::panic::panic_any(e),
        },
        Err(e) => std::panic::panic_any(e),
    }
}

// TODO: Aadd the rest0
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

// TODO: complete the list :p (maybe doind that while adding new keywords?)
pub fn sql_keyword(input: &[u8]) -> IResult<&[u8], &[u8]> {
    alt((
        terminated(tag_no_case("create"), keyword_follow_char),
        terminated(tag_no_case("table"), keyword_follow_char),
        terminated(tag_no_case("as"), keyword_follow_char),
    ))(input)
}

fn keyword_follow_char(input: &[u8]) -> IResult<&[u8], &[u8]> {
    peek(alt((
        tag(" "),
        tag("\n"),
        tag(";"),
        tag("("),
        tag(")"),
        tag("\t"),
        tag(","),
        tag("="),
        eof,
    )))(input)
}

// Parse a byte that ends a SQL statement.
pub fn statement_terminator(input: &[u8]) -> IResult<&[u8], ()> {
    let (remaining_input, _) =
        delimited(multispace0, alt((tag(";"), line_ending, eof)), multispace0)(input)?;

    Ok((remaining_input, ()))
}
// Parse AS ALIAS
pub fn as_alias(input: &[u8]) -> IResult<&[u8], &str> {
    map(
        tuple((
            multispace1,
            opt(pair(tag_no_case("as"), multispace1)),
            sql_identifier,
        )),
        |(_space, _as, alias)| std::str::from_utf8(alias).unwrap(),
    )(input)
}
