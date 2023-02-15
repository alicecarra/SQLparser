use nom::{
    bytes::complete::{tag, tag_no_case},
    character::complete::{multispace0, multispace1},
    combinator::{map, opt},
    error::context,
    sequence::{pair, tuple},
    IResult,
};
use serde::{Deserialize, Serialize};

use crate::{
    column::{column_specification_list, ColumnSpecification},
    common::{as_alias, sql_identifier, statement_terminator},
    table::Table,
};

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct CreateTable {
    pub table: Table,
    pub fields: Vec<ColumnSpecification>,
}

pub fn table_creation(input: &[u8]) -> IResult<&[u8], CreateTable> {
    let (
        remaining_input,
        (_create_keyword, _, _table_keyword, _, table, _, _, fields, _, _terminator),
    ) = context(
        "Table Creation",
        tuple((
            tag_no_case("create"),
            multispace1,
            tag_no_case("table"),
            multispace1,
            schema_table_reference,
            multispace0,
            tag("("),
            column_specification_list,
            tag(")"),
            statement_terminator,
        )),
    )(input)?;

    Ok((remaining_input, CreateTable { table, fields }))
}

// Parse a reference to a named schema.table. TODO: ADD ALIAS!
pub fn schema_table_reference(input: &[u8]) -> IResult<&[u8], Table> {
    context(
        "Table schema naming",
        map(
            tuple((
                opt(pair(sql_identifier, tag("."))),
                sql_identifier,
                opt(as_alias),
            )),
            |(schema_with_dot, identifier, alias)| Table {
                name: String::from(std::str::from_utf8(identifier).unwrap()),
                schema: schema_with_dot
                    .map(|(schema, _dot)| String::from(std::str::from_utf8(schema).unwrap())),
                alias: alias.map(String::from),
            },
        ),
    )(input)
}

#[cfg(test)]
mod tests {
    use crate::{
        column::{Column, ColumnConstraint, ColumnSpecification},
        query::create::{table_creation, CreateTable},
        table::Table,
        types::SqlType,
    };

    #[test]
    fn basic_table_creation() {
        let sql = r#"create table [test].[clients] (FirstName varchar(255) not null, SecondName varchar(255) not null, isActive bool not null);"#;

        let result = CreateTable {
            table: Table {
                name: String::from("clients"),
                schema: Some(String::from("test")),
                alias: None,
            },
            fields: vec![
                ColumnSpecification {
                    column: Column {
                        name: String::from("FirstName"),
                    },
                    sql_type: SqlType::VarChar(255),
                    constraints: vec![ColumnConstraint::NotNull],
                },
                ColumnSpecification {
                    column: Column {
                        name: String::from("SecondName"),
                    },
                    sql_type: SqlType::VarChar(255),
                    constraints: vec![ColumnConstraint::NotNull],
                },
                ColumnSpecification {
                    column: Column {
                        name: String::from("isActive"),
                    },
                    sql_type: SqlType::Bool,
                    constraints: vec![ColumnConstraint::NotNull],
                },
            ],
        };

        assert_eq!(table_creation(sql.as_bytes()).unwrap().1, result);
    }
}
