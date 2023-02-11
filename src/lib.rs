use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case, take_while1},
    character::{
        complete::{line_ending, multispace0, multispace1},
        is_alphanumeric,
    },
    combinator::{eof, map, not, opt, peek},
    sequence::{delimited, pair, preceded, terminated, tuple},
    IResult,
};
use serde_derive::{Deserialize, Serialize};

//TODO: IMPLEMENTS OTHERS COMMANDS
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum SqlCommandQuery {
    CreateTable(CreateTable),
    Insert,
    Select,
    Delete,
    DropTable,
    Update,
    Set,
}

// Parses a SQL identifier.
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

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct CreateTable {
    table: Table,
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct Table {
    pub name: String,
    pub schema: Option<String>,
}

pub fn table_creation(input: &[u8]) -> IResult<&[u8], CreateTable> {
    let (remaining_input, (_create_keyword, _, _table_keyword, _, table, _terminator)) =
        tuple((
            tag_no_case("create"),
            multispace1,
            tag_no_case("table"),
            multispace1,
            schema_table_reference,
            statement_terminator,
        ))(input)?;

    Ok((remaining_input, CreateTable { table }))
}

// Parse a reference to a named schema.table. TODO: ADD ALIAS!
pub fn schema_table_reference(i: &[u8]) -> IResult<&[u8], Table> {
    map(
        tuple((opt(pair(sql_identifier, tag("."))), sql_identifier)),
        |tup| Table {
            name: String::from(std::str::from_utf8(tup.1).unwrap()),
            schema: tup
                .0
                .map(|(schema, _)| String::from(std::str::from_utf8(schema).unwrap())),
        },
    )(i)
}

#[cfg(test)]
mod tests {
    use crate::{table_creation, CreateTable, Table};

    #[test]
    fn basic_table_creation() {
        let sql = "create table [test].[clients]; garbage";
        let result = CreateTable {
            table: Table {
                name: String::from("clients"),
                schema: Some(String::from("test")),
            },
        };

        assert_eq!(table_creation(sql.as_bytes()).unwrap().1, result);
    }
}
