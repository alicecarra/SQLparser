use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case, take_while1},
    character::{
        complete::{digit1, line_ending, multispace0, multispace1},
        is_alphanumeric,
    },
    combinator::{eof, map, not, opt, peek},
    error::context,
    sequence::{delimited, pair, preceded, terminated, tuple},
    IResult,
};

#[inline]
pub fn valid_identifier(input: &[u8]) -> IResult<&[u8], &[u8]> {
    context(
        "valid identifier",
        preceded(not(peek(sql_keyword)), take_while1(is_sql_identifier)),
    )(input)
}

pub fn sql_identifier(input: &[u8]) -> IResult<&[u8], &[u8]> {
    context(
        "sql identifier",
        alt((
            valid_identifier,
            delimited(tag("`"), take_while1(is_sql_identifier), tag("`")),
            delimited(tag("["), take_while1(is_sql_identifier), tag("]")),
        )),
    )(input)
}
#[inline]
pub fn is_sql_identifier(chr: u8) -> bool {
    is_alphanumeric(chr) || chr == b'_' || chr == b'@'
}

#[inline]
pub fn opt_signed(input: &[u8]) -> IResult<&[u8], Option<&[u8]>> {
    opt(alt((tag_no_case("unsigned"), tag_no_case("signed"))))(input)
}

#[inline]
pub fn delimited_digit(i: &[u8]) -> IResult<&[u8], &[u8]> {
    delimited(tag("("), digit1, tag(")"))(i)
}

pub(crate) fn comma_separator(i: &[u8]) -> IResult<&[u8], &[u8]> {
    delimited(multispace0, tag(","), multispace0)(i)
}

#[inline]
pub fn len_as_u16(len: &[u8]) -> u16 {
    match std::str::from_utf8(len) {
        Ok(s) => match s.parse::<u16>() {
            Ok(v) => v,
            Err(e) => std::panic::panic_any(e),
        },
        Err(e) => std::panic::panic_any(e),
    }
}

// TODO: complete the list :p (maybe doind that while adding new keywords?)
pub fn sql_keyword(input: &[u8]) -> IResult<&[u8], &[u8]> {
    alt((
        terminated(tag_no_case("create"), keyword_follow_char),
        terminated(tag_no_case("table"), keyword_follow_char),
        terminated(tag_no_case("as"), keyword_follow_char),
        terminated(tag_no_case("not"), keyword_follow_char),
        terminated(tag_no_case("null"), keyword_follow_char),
        terminated(tag_no_case("check"), keyword_follow_char),
        terminated(tag_no_case("default"), keyword_follow_char),
        terminated(tag_no_case("autoincrement"), keyword_follow_char),
        terminated(tag_no_case("primary"), keyword_follow_char),
        terminated(tag_no_case("unique"), keyword_follow_char),
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
