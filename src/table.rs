use nom::{
    bytes::complete::{tag, tag_no_case},
    character::{
        complete::{multispace1},
    },
    combinator::{map, opt},
    sequence::{pair, tuple},
    IResult,
};
use serde_derive::{Deserialize, Serialize};

use crate::common::{sql_identifier, statement_terminator};

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
    use crate::table::{table_creation, CreateTable, Table};

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
