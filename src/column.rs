use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case, take_until},
    character::complete::{digit1, multispace0, multispace1},
    combinator::{map, opt},
    error::context,
    multi::{many0, many1},
    sequence::{delimited, tuple},
    IResult,
};
use serde::{Deserialize, Serialize};

use crate::{
    common::{comma_separator, valid_identifier},
    types::{type_identifier, Literal, Real, SqlType},
};

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct ColumnSpecification {
    pub column: Column,
    pub sql_type: SqlType,
    pub constraints: Vec<ColumnConstraint>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct Column {
    pub name: String,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum ColumnConstraint {
    NotNull,
    Check,                 //todo
    DefaultValue(Literal), //todo
    AutoIncrement,
    PrimaryKey,
    Unique,
}

pub fn field_specification(input: &[u8]) -> IResult<&[u8], ColumnSpecification> {
    let (remaining_input, (column_name, column_type, constraints, _terminator)) = tuple((
        map(valid_identifier, |column_name| {
            std::str::from_utf8(column_name).unwrap()
        }),
        opt(delimited(multispace1, type_identifier, multispace0)),
        many0(column_constraint),
        opt(comma_separator),
    ))(input)?;

    let column_type = match column_type {
        Some(column_type) => column_type,
        None => SqlType::Text,
    };

    Ok((
        remaining_input,
        ColumnSpecification {
            column: Column {
                name: column_name.to_string(),
            },
            sql_type: column_type,
            constraints: constraints.into_iter().flatten().collect(),
        },
    ))
}

pub fn column_specification_list(input: &[u8]) -> IResult<&[u8], Vec<ColumnSpecification>> {
    context("column specification list", many1(field_specification))(input)
}

// Parse rule for a column definition constraint.
pub fn column_constraint(input: &[u8]) -> IResult<&[u8], Option<ColumnConstraint>> {
    let not_null = map(
        delimited(multispace0, tag_no_case("not null"), multispace0),
        |_| Some(ColumnConstraint::NotNull),
    );
    let null = map(
        delimited(multispace0, tag_no_case("null"), multispace0),
        |_| None,
    );
    let auto_increment = map(
        delimited(multispace0, tag_no_case("auto_increment"), multispace0),
        |_| Some(ColumnConstraint::AutoIncrement),
    );
    let primary_key = map(
        delimited(multispace0, tag_no_case("primary key"), multispace0),
        |_| Some(ColumnConstraint::PrimaryKey),
    );
    let unique = map(
        delimited(multispace0, tag_no_case("unique"), multispace0),
        |_| Some(ColumnConstraint::Unique),
    );

    alt((not_null, null, auto_increment, default, primary_key, unique))(input)
}

fn fixed_point(input: &[u8]) -> IResult<&[u8], Literal> {
    let (remaining_input, (integer, _, fractional)) = tuple((digit1, tag("."), digit1))(input)?;

    Ok((
        remaining_input,
        Literal::FixedPoint(Real {
            integral: std::str::from_utf8(integer)
                .unwrap()
                .parse::<i32>()
                .unwrap(),
            fractional: std::str::from_utf8(fractional)
                .unwrap()
                .parse::<i32>()
                .unwrap(),
        }),
    ))
}

fn default(input: &[u8]) -> IResult<&[u8], Option<ColumnConstraint>> {
    let (remaining_input, (_, _keyword, _, default_value, _)) = tuple((
        multispace0,
        tag_no_case("default"),
        multispace1,
        alt((
            map(
                delimited(tag("'"), take_until("'"), tag("'")),
                |s: &[u8]| Literal::String(String::from_utf8(s.to_vec()).unwrap()),
            ),
            fixed_point,
            map(digit1, |d| {
                let d_i64 = std::str::from_utf8(d).unwrap().parse::<i64>().unwrap();
                Literal::Integer(d_i64)
            }),
            map(tag("''"), |_| Literal::String(String::from(""))),
            map(tag_no_case("null"), |_| Literal::Null),
            map(tag_no_case("current_timestamp"), |_| {
                Literal::CurrentTimestamp
            }),
        )),
        multispace0,
    ))(input)?;

    Ok((
        remaining_input,
        Some(ColumnConstraint::DefaultValue(default_value)),
    ))
}
