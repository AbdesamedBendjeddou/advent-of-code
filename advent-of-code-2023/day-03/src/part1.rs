use std::{collections::BTreeMap, error::Error};

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take_till, take_till1},
    character::complete::{digit1, none_of},
    combinator::iterator,
    error::ParseError,
    multi::{many0, many1},
    IResult, Offset, Parser,
};
use nom_locate::LocatedSpan;

use crate::custom_error::AocError;
type Span<'a> = LocatedSpan<&'a str>;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Number {
    value: usize,
    x: Vec<usize>,
    y: Option<usize>,
}

impl Number {
    fn from(span: Span) -> Self {
        let value = span.fragment();
        let offset = span.location_offset();
        dbg!("fragmnet,offsset", (value, span.location_offset()));
        let x = (offset.saturating_sub(1)..offset + value.len() + 1).collect();
        Number {
            value: value.parse().expect("shloud be a number"),
            x,
            y: None,
        }
    }
    fn with(mut self, y: usize) -> Self {
        self.y = Some(y);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Symbol {
    x: usize,
}
impl Symbol {
    fn from(span: Span) -> Self {
        Symbol {
            x: span.location_offset(),
        }
    }
    /*fn with(mut self, y: usize) -> Self {
        self.y = Some(y);
        self
    }*/
}

#[derive(Clone, PartialEq, Debug, Eq)]
enum Value {
    Number(Number),
    Symbol(Symbol),
    Empty,
}

fn validate_parts(numbers: Vec<Number>, symbols: BTreeMap<usize, Vec<usize>>) -> Vec<Number> {
    numbers
        .into_iter()
        .filter(|number| {
            let y = number.y.unwrap();
            let mut lines_to_check = vec![y.saturating_sub(1), y, y + 1];
            lines_to_check.dedup();
            lines_to_check.iter().any(|line| {
                if let Some(positions) = symbols.get(line) {
                    positions.into_iter().any(|pos| number.x.contains(pos))
                } else {
                    false
                }
            })
        })
        .collect()
}

fn parse<'a>(
    input: &'a str,
) -> Result<(Vec<Number>, BTreeMap<usize, Vec<usize>>), Box<dyn Error + 'a>> {
    let mut numbers = vec![];
    let mut symbols = BTreeMap::new();
    for (y, line) in input.lines().enumerate().inspect(|line| {
        dbg!(line);
    }) {
        let values = parse_line(line)?.1;
        for value in values {
            match value {
                Value::Number(number) => numbers.push(number.with(y)),
                Value::Symbol(symbol) => {
                    symbols
                        .entry(y)
                        .and_modify(|e: &mut Vec<usize>| e.push(symbol.x))
                        .or_insert(vec![symbol.x]);
                }
                Value::Empty => (),
            }
        }
    }

    Ok((numbers, symbols))
}

fn parse_line(input: &str) -> IResult<Span, Vec<Value>> {
    let input = Span::new(input.trim());
    dbg!("entered parse line");
    let values /*Result<(LocatedSpan<&str>, Vec<Value>), nom::Err<_>>*/ = many1(alt((
        is_not(".0123456789")
            .map(|res| {
                dbg!("is not:",&res);
                Symbol::from(res)})
            .map(Value::Symbol),
        digit1.map(|res: Span| Number::from(res)).map( |res|{
            dbg!(("digit1:",&res));
            Value::Number(res)}),
        take_till1(|c: char| c.is_ascii_digit() || c != '.').map(|res|{
            dbg!("take_till:",&res);
            Value::Empty}),
    )))(input);
    values
    /*let mut it = iterator(
            Span::new(input),
            alt((
                take_till(|c: char| c.is_ascii_digit() || c != '.').map(|_| Value::Empty),
                digit1.map(|res: Span| Number::from(res)).map(Value::Number),
                is_not(".123456789")
                    .map(|res| Symbol::from(res))
                    .map(Value::Symbol),
            )),
        );
        let values = it
        .filter(|v| *v != Value::Empty)
        .inspect(|v| {
            dbg!(v);
        })
        .collect::<Vec<_>>();
    dbg!(&values);
    let res = it.finish();
    dbg!("exited parse line");
    res.map(|(span, _)| (span, values))*/
    // dbg!("after iter");
}

#[tracing::instrument]
pub fn process<'a>(input: &'a str) -> miette::Result<String, AocError> {
    let (numbers, symbols) = parse(input).unwrap();
    dbg!(&numbers, &symbols);
    let valid_parts = validate_parts(numbers, symbols);
    Ok(valid_parts
        .iter()
        .map(|part| part.value)
        .sum::<usize>()
        .to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_line() {
        let input = "300.400..500%..";
        assert_eq!(
            vec![
                Value::Number(Number {
                    value: 300,
                    x: vec![0, 1, 2],
                    y: None
                }),
                Value::Empty,
                Value::Number(Number {
                    value: 400,
                    x: vec![4, 5, 6],
                    y: None
                }),
                Value::Empty,
                Value::Number(Number {
                    value: 500,
                    x: vec![9, 10, 11],
                    y: None
                }),
                Value::Symbol(Symbol { x: 12 }),
                Value::Empty
            ],
            parse_line(input).unwrap().1
        )
    }

    #[test]
    fn test_validate_numbers() {
        let _input = "..123%..22..*
                            /.32.....$.09";
        let mut symbols = BTreeMap::new();
        symbols.insert(0, vec![5, 12]);
        symbols.insert(1, vec![0, 9]);
        let numbers = vec![
            Number {
                value: 123,
                x: vec![2, 3, 4],
                y: Some(0),
            },
            Number {
                value: 22,
                x: vec![8, 9, 10],
                y: Some(0),
            },
            Number {
                value: 32,
                x: vec![11, 12],
                y: Some(1),
            },
            Number {
                value: 09,
                x: vec![11, 12],
                y: Some(1),
            },
        ];
        assert_eq!(
            validate_parts(numbers, symbols),
            vec![
                Number {
                    value: 123,
                    x: vec![2, 3, 4],
                    y: Some(0)
                },
                Number {
                    value: 22,
                    x: vec![8, 9, 10],
                    y: Some(0)
                },
                Number {
                    value: 09,
                    x: vec![11, 12],
                    y: Some(1)
                }
            ]
        );
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..";
        assert_eq!("4361", process(input)?);
        Ok(())
    }
}
/*input
.lines()
.enumerate()
.map(|(y, line)| parse_line(y, line))
.for_each(|res| {
    res.and_then(|(_, values)| {
        values.into_iter().for_each(|value| match value {
            Value::Number(number) => numbers.push(number),
            Value::Symbol(symbol) => {
                symbols.insert((symbol.x, symbol.y), symbol.x);
            }
            Value::Empty => (),
        });
        Ok(())
    });
});*/
