use std::{collections::BTreeMap, error::Error};

use nom::{
    branch::alt,
    bytes::complete::{is_not, take_till1},
    character::complete::digit1,
    multi::many1,
    IResult, Parser,
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
}

#[derive(Clone, PartialEq, Debug, Eq)]
enum Value {
    Number(Number),
    Symbol(Symbol),
    Empty,
}

#[tracing::instrument]
pub fn process<'a>(input: &'a str) -> miette::Result<String, AocError> {
    let (numbers, symbols) = parse(input).unwrap();
    let valid_parts = validate_parts(numbers, symbols);
    Ok(valid_parts
        .iter()
        .map(|part| part.value)
        .sum::<usize>()
        .to_string())
}

fn parse<'a>(
    input: &'a str,
) -> Result<(Vec<Number>, BTreeMap<usize, Vec<usize>>), Box<dyn Error + 'a>> {
    let mut numbers = vec![];
    let mut symbols = BTreeMap::new();
    for (y, line) in input.lines().enumerate() {
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
    many1(alt((
        is_not(".0123456789").map(Symbol::from).map(Value::Symbol),
        digit1.map(Number::from).map(Value::Number),
        take_till1(|c: char| c.is_ascii_digit() || c != '.').map(|_| Value::Empty),
    )))(input)
}

fn validate_parts(numbers: Vec<Number>, symbols: BTreeMap<usize, Vec<usize>>) -> Vec<Number> {
    numbers
        .into_iter()
        .filter(|number| {
            let y = number.y.unwrap();
            let mut lines_to_check = vec![y.saturating_sub(1), y, y + 1];
            lines_to_check.dedup();
            lines_to_check.iter().any(|y| {
                if let Some(positions) = symbols.get(y) {
                    positions.iter().any(|pos| number.x.contains(pos))
                } else {
                    false
                }
            })
        })
        .collect()
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
