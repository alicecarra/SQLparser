// Parses a SQL identifier.

use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case, take_while1},
    character::{
        complete::{line_ending, multispace0},
        is_alphanumeric,
    },
    combinator::{eof, not, peek},
    sequence::{delimited, preceded, terminated},
    IResult,
};
pub fn sql_identifier(i: &[u8]) -> IResult<&[u8], &[u8]> {
    alt((
        preceded(not(peek(sql_keyword)), take_while1(is_sql_identifier)),
        delimited(tag("`"), take_while1(is_sql_identifier), tag("`")),
        delimited(tag("["), take_while1(is_sql_identifier), tag("]")),
    ))(i)
}
#[inline]
pub fn is_sql_identifier(chr: u8) -> bool {
    is_alphanumeric(chr) || chr == b'_' || chr == b'@'
}

// TODO: complete the list :p
pub fn sql_keyword(i: &[u8]) -> IResult<&[u8], &[u8]> {
    alt((
        terminated(tag_no_case("create"), keyword_follow_char),
        terminated(tag_no_case("table"), keyword_follow_char),
    ))(i)
}

fn keyword_follow_char(i: &[u8]) -> IResult<&[u8], &[u8]> {
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
    )))(i)
}

// Parse a terminator that ends a SQL statement.
pub fn statement_terminator(i: &[u8]) -> IResult<&[u8], ()> {
    let (remaining_input, _) =
        delimited(multispace0, alt((tag(";"), line_ending, eof)), multispace0)(i)?;

    Ok((remaining_input, ()))
}
