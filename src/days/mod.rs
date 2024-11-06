use crate::io::AocRunError;
use crate::problem::ProblemSolution;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

pub fn get_solution(problem: usize) -> Result<Box<dyn ProblemSolution>, AocRunError> {
    match problem {
        01 => Ok(Box::new(day01::Solution {})),
        02 => Ok(Box::new(day02::Solution {})),
        03 => Ok(Box::new(day03::Solution {})),
        04 => Ok(Box::new(day04::Solution {})),
        05 => Ok(Box::new(day05::Solution {})),
        06 => Ok(Box::new(day06::Solution {})),
        07 => Ok(Box::new(day07::Solution {})),
        08 => Ok(Box::new(day08::Solution {})),
        09 => Ok(Box::new(day09::Solution {})),
        10 => Ok(Box::new(day10::Solution {})),
        11 => Ok(Box::new(day11::Solution {})),
        12 => Ok(Box::new(day12::Solution {})),
        13 => Ok(Box::new(day13::Solution {})),
        14 => Ok(Box::new(day14::Solution {})),
        15 => Ok(Box::new(day15::Solution {})),
        16 => Ok(Box::new(day16::Solution {})),
        17 => Ok(Box::new(day17::Solution {})),
        18 => Ok(Box::new(day18::Solution {})),
        19 => Ok(Box::new(day19::Solution {})),
        20 => Ok(Box::new(day20::Solution {})),
        21 => Ok(Box::new(day21::Solution {})),
        22 => Ok(Box::new(day22::Solution {})),
        23 => Ok(Box::new(day23::Solution {})),
        24 => Ok(Box::new(day24::Solution {})),
        25 => Ok(Box::new(day25::Solution {})),
        _ => Err(AocRunError::UnregistedProblem(problem)),
    }
}
