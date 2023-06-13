use crate::day02::Input;

const INPUT: &str = include_str!("../../input/02/input.txt");

pub fn read() -> Input {
    INPUT
        .trim()
        .split("\n\n")
        .map(try_parse_elf_calories)
        .collect::<Result<_,_>>()
        .expect("Could not parse input!")
}

fn try_parse_elf_calories(value: &str) -> Result<u32, std::num::ParseIntError> {
    let mut total = 0;
    for line in value.lines() {
        total += line.parse::<u32>()?;
    }
    Ok(total)
}
