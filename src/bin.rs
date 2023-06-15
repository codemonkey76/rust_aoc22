use advent22lib::*;
use clap::Parser;
use std::time::{Instant, Duration};
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    day: Option<u8>,

    #[arg(short, long)]
    all: bool,

    #[arg(short, long)]
    timed: bool
}

fn main() {
    let args = Args::parse();
    let all = args.all;
    let timed = args.timed;
    let timer = Instant::now();

    if !all {
        if let None = args.day {
            println!("Didn't do anything. Run with --help to see flags.");
            return;
        }

        let day = args.day.unwrap();
        if timed {
            println!("Run timed: {}", run_timed(day, &timer));
            return;
        }

        println!("Run day {}", run_day(day));
        return;
    }

    if timed {
        let mut results = Vec::with_capacity(25);
        for day in 1..=25 {
            results.push(run_timed(day, &timer));
        }
        let total = timer.elapsed();
        results.iter().for_each(|result| println!("{}", result));
        println!("Total Runtime: {total:?}");
        return;
    }

    (1..=25).for_each(|day| println!("{}", run_day(day)));
}

struct TimedResult {
    day: u8,
    answer_one: Output,
    answer_two: Output,
    duration: Duration
}

impl Display for TimedResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let TimedResult { day, answer_one, answer_two, duration } = self;
        writeln!(f, "****************************************************")?;
        writeln!(f, "*  Advent of Code: 2022, Day {day}")?;
        writeln!(f, "*      Solution for ...")?;
        writeln!(f, "*          Part One: {answer_one}")?;
        writeln!(f, "*          Part Two: {answer_two}")?;
        writeln!(f, "*  Run Time: {duration:?}")?;
        write!(f, "")
    }
}

struct RunResult {
    day: u8,
    answer_one: Output,
    answer_two: Output,
}

impl Display for RunResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let RunResult { day, answer_one, answer_two } = self;
        writeln!(f, "************************************************************")?;
        writeln!(f, "* Advent of Code: 2022, Day {day}")?;
        writeln!(f, "*   Solution for...")?;
        writeln!(f, "*     Part One: {answer_one}")?;
        writeln!(f, "*     Part Two: {answer_two}")?;
        writeln!(f, "************************************************************")?;
        write!(f, "")
    }
}

fn run_timed(day: u8, timer: &Instant) -> TimedResult {
    let run = match day {
        1  => day01::run_both,
        2  => day02::run_both,
        3  => day03::run_both,
        _ => panic!("There's no day {day} on the Advent Calendar!"),
    };

    let start = timer.elapsed();
    let (answer_one, answer_two) = run();
    let duration = timer.elapsed() - start;
    TimedResult { day, answer_one, answer_two, duration }
}

fn run_day(day: u8) -> RunResult {
    let run = match day {
        1 => day01::run,
        2 => day02::run,
        3 => day03::run,
        _ => panic!("There's no day {day} on the Advent Calendar!"),
    };

    let answer_one = run(Part::One);
    let answer_two = run(Part::Two);
    RunResult { day, answer_one, answer_two }
}
