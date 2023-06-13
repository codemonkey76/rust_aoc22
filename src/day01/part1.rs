use crate::day01::{Input, Output};

pub fn solve(input: &Input) -> Output {
    input.iter().copied().max().unwrap_or_default().into()
}
