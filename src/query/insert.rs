use nom::{
    branch::alt,
    bytes::{
        complete::{is_not, tag, tag_no_case, take_till, take_while},
        streaming::tag,
    },
    character::complete::{char, digit1, multispace0, multispace1},
    combinator::{map, not, opt},
    error::context,
    multi::{many0, many1},
    sequence::{delimited, pair, preceded, tuple},
    IResult,
};
use serde::{Deserialize, Serialize};

use crate::{
    column::{column_specification_list, field_specification, Column},
    common::{comma_separator, statement_terminator, valid_identifier},
    table::Table,
    types::{Literal, Real},
};

use super::create::schema_table_reference;

pub type InsertColumns = Vec<Column>;
pub type InsertValues = Vec<Literal>;

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct InsertTable {
    pub table: Table,
    pub fields: InsertColumns,
    pub values: Vec<InsertValues>,
}

pub fn table_creation(input: &[u8]) -> IResult<&[u8], InsertTable> {
    let foo = context(
        "Table insertion",
        tuple((
            tag_no_case("insert"),
            multispace1,
            tag_no_case("into"),
            multispace1,
            schema_table_reference,
            multispace0,
            opt(fields),
            tag_no_case("values"),
            multispace0,
            many1(data),
            statement_terminator,
        )),
    )(input)?;

    Ok((
        input,
        InsertTable {
            table: todo!(),
            fields: todo!(),
            values: todo!(),
        },
    ))
}

fn fields(input: &[u8]) -> IResult<&[u8], Vec<Column>> {
    delimited(
        preceded(tag("("), multispace0),
        many1(map(valid_identifier, |column_name| Column {
            name: std::str::from_utf8(column_name).unwrap().to_string(),
        })),
        delimited(multispace0, tag(")"), multispace1),
    )(input)
}

fn data(input: &[u8]) -> IResult<&[u8], Vec<Literal>> {
    delimited(
        tag("("),
        value_list,
        preceded(tag(")"), opt(comma_separator)),
    )(input)
}

pub fn value_list(input: &[u8]) -> IResult<&[u8], Vec<Literal>> {
    many0(delimited(multispace0, literal, opt(comma_separator)))(input)
}

pub fn literal(input: &[u8]) -> IResult<&[u8], Literal> {
    alt((
        map(
            tuple((opt(tag("-")), digit1, tag("."), digit1)),
            |(_signal, integral, _dot, fractional)| {
                Literal::FixedPoint(Real {
                    integral: if (integral).is_some() {
                        -std::str::from_utf8(integral)
                            .unwrap()
                            .parse::<i32>()
                            .unwrap()
                    } else {
                        std::str::from_utf8(integral)
                            .unwrap()
                            .parse::<i32>()
                            .unwrap()
                    },
                    fractional: std::str::from_utf8(fractional)
                        .unwrap()
                        .parse::<i32>()
                        .unwrap(),
                })
            },
        ),
        map(pair(opt(tag("-")), digit1), |(signal, value)| {
            let mut integer = std::str::from_utf8(value).unwrap().parse::<i64>().unwrap();
            if (signal).is_some() {
                integer *= -1;
            }
            Literal::Integer(integer)
        }),
        map(
            tuple((
                tag("\'"),
                take_till(is_not(tag("\'"))),
                preceded(tag("\'"), opt(comma_separator)),
            )),
            |bytes| match String::from_utf8(bytes) {
                Ok(string) => Literal::String(string),
                Err(err) => Literal::Blob(err.into_bytes()),
            },
        ),
        map(tag_no_case("null"), |_| Literal::Null),
        map(tag_no_case("current_timestamp"), |_| {
            Literal::CurrentTimestamp
        }),
        map(tag_no_case("current_date"), |_| Literal::CurrentDate),
        map(tag_no_case("current_time"), |_| Literal::CurrentTime),
    ))(input)
}
