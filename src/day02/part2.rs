use super::shared::{Outcome, Shape};
use crate::day02::{Input, Output};

pub fn solve(input: &Input) -> Output {
    input
        .iter()
        .flat_map(|pair| pair.try_into_outcome())
        .map(|outcome| outcome.score())
        .sum::<u32>()
        .into()
}

trait TryIntoShape {
    type Error;
    fn try_into_shape(&self) -> Result<Shape, Self::Error>;
}

impl TryIntoShape for char {
    type Error = &'static str;
    fn try_into_shape(&self) -> Result<Shape, Self::Error> {
        match self {
            'A' => Ok(Shape::Rock),
            'B' => Ok(Shape::Paper),
            'C' => Ok(Shape::Scissors),
            _ => Err("Character cannot be converted to `Shape`!"),
        }
    }
}

trait TryIntoOutcome {
    type Error;
    fn try_into_outcome(&self) -> Result<Outcome, Self::Error>;
}

impl TryIntoOutcome for (char, char) {
    type Error = &'static str;

    fn try_into_outcome(&self) -> Result<Outcome, Self::Error> {
        let (ch1, result) = self;
        let opponent = ch1.try_into_shape()?;

        use Shape::*;
        match (opponent, result) {
            (Rock, 'Y') => Ok(Outcome::Draw(Rock)),
            (Rock, 'Z') => Ok(Outcome::Win(Paper)),
            (Rock, 'X') => Ok(Outcome::Lose(Scissors)),
            (Paper, 'X') => Ok(Outcome::Lose(Rock)),
            (Paper, 'Y') => Ok(Outcome::Draw(Paper)),
            (Paper, 'Z') => Ok(Outcome::Win(Scissors)),
            (Scissors, 'Z') => Ok(Outcome::Win(Rock)),
            (Scissors, 'X') => Ok(Outcome::Lose(Paper)),
            (Scissors, 'Y') => Ok(Outcome::Draw(Scissors)),
            (_, _) => Err("Cannot convert character pair to an Outcome!"),
        }
    }
}
