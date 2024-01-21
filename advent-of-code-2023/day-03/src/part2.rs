use std::{collections::BTreeMap, error::Error};

use nom::{
    branch::alt,
    bytes::complete::{is_a, take_till1},
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
    let valid_parts = validate_gears(&numbers, symbols);
    Ok(valid_parts
        .iter()
        .map(|parts| parts.iter().map(|part| part.value).reduce(|acc, e| acc*e).unwrap())
        .sum::<usize>()
        .to_string())
}

fn parse<'a>(
    input: &'a str,
) -> Result<(BTreeMap<usize, Vec<Number>>, BTreeMap<usize, Vec<usize>>), Box<dyn Error + 'a>> {
    let mut numbers = BTreeMap::new();
    let mut symbols = BTreeMap::new();
    for (y, line) in input.lines().enumerate() {
        let values = parse_line(line)?.1;
        for value in values {
            match value {
                Value::Number(number) => {
                    numbers
                        .entry(y)
                        .and_modify(|e: &mut Vec<Number>| e.push(number.clone().with(y)))
                        .or_insert(vec![number.with(y)]);
                }
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
        is_a("*").map(Symbol::from).map(Value::Symbol),
        digit1.map(Number::from).map(Value::Number),
        take_till1(|c: char| c.is_ascii_digit() || c == '*').map(|_| Value::Empty),
    )))(input)
}

fn validate_gears(
    numbers: &BTreeMap<usize, Vec<Number>>,
    symbols: BTreeMap<usize, Vec<usize>>,
) -> Vec<Vec<&Number>> {
    let mut all_adj_parts = vec![];
    for (y, symbols) in symbols {
        for symobl in symbols {
            let mut lines_to_check = vec![y.saturating_sub(1), y, y + 1];
            lines_to_check.dedup();
            let adj_parts = lines_to_check
                .into_iter()
                .filter_map(|y| numbers.get(&y))
                .flat_map(|y| y)
                .filter(|number| number.x.contains(&symobl))
                .collect::<Vec<_>>();
            all_adj_parts.push(adj_parts);
        }
    }
    all_adj_parts.into_iter().filter(|adj_parts| adj_parts.len()>= 2).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!("467835", process(input)?);
        Ok(())
    }
}
