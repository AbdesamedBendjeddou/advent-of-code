use nom::{
    bytes::complete::{tag, take_while1},
    character::complete::digit1,
    combinator::map_res,
    multi::separated_list1,
    sequence::preceded,
    Finish, IResult,
};

use crate::custom_error::AocError;

struct Game {
    rounds: Vec<Round>,
}

impl Game {
    fn from(rounds: Vec<Round>) -> Self {
        Self { rounds }
    }

    fn min_cubes(&self, color: &str) -> u32 {
        self.rounds
            .iter()
            .map(|round| round.revealed(color))
            .reduce(|min_cubes, cubes_round| {
                if min_cubes > cubes_round {
                    min_cubes
                } else {
                    cubes_round
                }
            })
            .unwrap()
    }

    fn power(&self) -> u32 {
        let min_red = self.min_cubes("red");
        let min_green = self.min_cubes("green");
        let min_blue = self.min_cubes("blue");
        min_red * min_green * min_blue
    }
}

struct Round {
    cubes: Vec<Cube>,
}

impl Round {
    fn from(cubes: Vec<Cube>) -> Self {
        Self { cubes }
    }

    fn revealed(&self, color: &str) -> u32 {
        self.cubes
            .iter()
            .filter_map(|cube| match (cube, color) {
                (Cube::Red(revealed), "red")
                | (Cube::Green(revealed), "green")
                | (Cube::Blue(revealed), "blue") => Some(*revealed),
                _ => None,
            })
            .last()
            .unwrap_or_default()
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
            _ => panic!("wrong input"),
        }
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let output = input
        .lines()
        .filter_map(|line| process_line(line.trim()).finish().ok())
        .map(|output| output.1)
        .sum::<u32>();
    Ok(output.to_string())
}

fn process_line(line: &str) -> IResult<&str, u32> {
    let mut parser = preceded(tag("Game "), map_res(digit1, str::parse::<u32>));
    let (game_input, _) = parser(line)?;
    let game = game(game_input);
    if let Ok((_, game)) = game {
        Ok((game_input, game.power()))
    } else {
        Err(nom::Err::Error(nom::error::Error::new(
            game_input,
            nom::error::ErrorKind::Verify,
        )))
    }
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
    fn test_process() -> miette::Result<()> {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!("2286", process(input)?);
        Ok(())
    }
}
