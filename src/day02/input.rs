use crate::day02::Input;
use nom::{
    character::complete::{anychar, space1},
    error::Error as NomError,
    sequence::separated_pair,
    Finish, IResult,
};

const INPUT: &str = include_str!("../../input/02/input.txt");

fn parse_line(line: &str) -> Result<(char, char), NomError<&str>> {
    let (_, char_pair) = separated_pair(anychar, space1, anychar)(line).finish()?;
    Ok(char_pair)
}

pub fn read() -> Input {
    INPUT
        .lines()
        .flat_map(parse_line)
        .collect()
}
