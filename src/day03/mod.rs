pub mod input;
pub mod part1;
pub mod part2;

use crate::{Output, Part};
use input::{Item, ItemSet, Rucksack};

pub type Input = Vec<Rucksack>;

pub fn run(part: Part) -> Output {
    let input = input::read();
    match part {
        Part::One => part1::solve(&input),
        Part::Two => part2::solve(&input),
    }
}

pub fn run_both() -> (Output, Output) {
    let input = input::read();
    (part1::solve(&input), part2::solve(&input))
}
