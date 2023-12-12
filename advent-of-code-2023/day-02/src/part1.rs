
use nom::{
    bytes::complete::{tag, take_while1},
    character::complete::digit1,
    combinator::map_res,
    multi::separated_list1,
    sequence::preceded, Finish, IResult,
};

use crate::custom_error::AocError;

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

struct Game {
    rounds: Vec<Round>,
}

impl Game {
    fn from(rounds: Vec<Round>) -> Self {
        Self { rounds }
    }
    fn is_possible(&self) -> bool {
        self.rounds.iter().all(|round| round.is_possible())
    }
}

struct Round {
    cubes: Vec<Cube>,
}

impl Round {
    fn from(cubes: Vec<Cube>) -> Self {
        Self { cubes }
    }
    fn is_possible(&self) -> bool {
        self.cubes.iter().all(|cube| cube.le_max_cube())
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
enum Cube {
    Red(u32),
    Green(u32),
    Blue(u32),
}
impl Cube {
    fn from(str: &str) -> Self {
        let input: Vec<&str> = str.trim().split(' ').collect();
        let (revealed, color) = (input[0], input[1]);
        match color {
            "red" => Cube::Red(revealed.parse::<u32>().unwrap()),
            "green" => Cube::Green(revealed.parse::<u32>().unwrap()),
            "blue" => Cube::Blue(revealed.parse::<u32>().unwrap()),
            _ => panic!("")
        }
    }
    fn le_max_cube(&self) -> bool {
        match self {
            Cube::Red(revealed) => *revealed <= MAX_RED,
            Cube::Green(revealed) => *revealed <= MAX_GREEN,
            Cube::Blue(revealed) => *revealed <= MAX_BLUE,
        }
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let output = input
        .lines()
        .filter_map(|line| process_line(line).finish().ok())
        .map(|output| output.1)
        .sum::<u32>();
    Ok(output.to_string())
}

fn process_line(line: &str) -> IResult<&str, u32> {
    let mut parser = preceded(tag("Game "), map_res(digit1, str::parse::<u32>));
    let (game_input, id) = parser(line)?;
    let game = game(game_input);
    let res = game.and_then(|(_, game)| {
        if game.is_possible() {
            Ok((game_input, id))
        } else {
            Err(nom::Err::Error(nom::error::Error::new(
                game_input,
                nom::error::ErrorKind::Verify,
            )))
        }
    });
    res
}

fn game(input: &str) -> IResult<&str, Game> {
    let mut parser = preceded(
        tag(": "),
        separated_list1(tag(";"), take_while1(|c| c != ';')),
    );
    let rounds_input = parser(input)?.1;
    let rounds = rounds_input
        .into_iter()
        .map(round)
        .filter_map(|res| res.ok())
        .map(|res| res.1)
        .collect::<Vec<Round>>();
    Ok((input, Game::from(rounds)))
}

fn round(input: &str) -> IResult<&str, Round> {
    let mut parser = preceded(
        tag(""),
        separated_list1(tag(", "), take_while1(|c| c != ',')),
    );
    let cubes_input = parser(input)?.1;
    let cubes = cubes_input
        .into_iter()
        .map(Cube::from)
        .collect::<Vec<Cube>>();
    Ok((input, Round::from(cubes)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cube_from() {
        let input = "3 blue";
        assert_eq!(Cube::from(input), Cube::Blue(3));
    }
    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!("8", process(input)?);
        Ok(())
    }
}
