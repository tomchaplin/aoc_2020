use aoc_parse::{parser, prelude::*};
use colored::Colorize;
use std::fmt::Display;
use std::fs;
use std::str::FromStr;

use crate::days;

pub enum AocRunError {
    NoFile(String),
    UnregistedProblem(usize),
    BadRunCode(String),
}

impl Display for AocRunError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let explanation = match self {
            AocRunError::NoFile(path) => format!("Could not find problem input at path {path}"),
            AocRunError::UnregistedProblem(problem) => {
                format!("No solution registered for problem {problem}")
            }
            AocRunError::BadRunCode(code) => format!("Could not parse run code \"{code}\""),
        };
        explanation.bold().red().fmt(f)
    }
}

#[derive(Debug)]
pub struct RunCode<const SOLVED: bool> {
    problem: usize,
    run_a: bool,
    run_b: bool,
    as_example: bool,
    result_a: Option<String>,
    result_b: Option<String>,
}

impl RunCode<false> {
    pub fn init_run_all(problem: usize) -> Self {
        Self {
            problem,
            run_a: true,
            run_b: true,
            as_example: false,
            result_a: None,
            result_b: None,
        }
    }
}

impl FromStr for RunCode<false> {
    type Err = AocRunError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ab_parser = parser!({
            "a" => (true, false),
             "b" => (false, true),
             "ab" => (true, true),
             "" => (true, true)
        });
        let example_parser = parser!({"e" => true, "" => false});
        let code_parser = parser!(example_parser usize ab_parser);
        let (as_example, problem, (run_a, run_b)) = code_parser
            .parse(s)
            .map_err(|_err| AocRunError::BadRunCode(s.to_string()))?;
        Ok(RunCode {
            problem,
            run_a,
            run_b,
            as_example,
            result_a: None,
            result_b: None,
        })
    }
}

impl RunCode<false> {
    fn get_input(&self) -> Result<String, AocRunError> {
        let foldername = if self.as_example {
            "examples"
        } else {
            "inputs"
        };
        let problem = self.problem;
        let path = format!("./data/{foldername}/{problem:02}.txt");
        fs::read_to_string(&path).map_err(|_e| AocRunError::NoFile(path))
    }

    pub fn run(self) -> Result<RunCode<true>, AocRunError> {
        let solution = days::get_solution(self.problem)?;
        let input = self.get_input()?;
        let result_a = self.run_a.then(|| solution.solve_a(&input)).flatten();
        let result_b = self.run_b.then(|| solution.solve_b(&input)).flatten();
        Ok(RunCode {
            problem: self.problem,
            run_a: self.run_a,
            run_b: self.run_b,
            as_example: self.as_example,
            result_a,
            result_b,
        })
    }
}

impl Display for RunCode<true> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.run_a {
            let solution_str = match self.result_a.as_ref() {
                Some(s) => s.to_string().bold().cyan(),
                None => "TODO".to_string().dimmed(),
            };
            writeln!(f, "A : {solution_str}")?
        }
        if self.run_b {
            let solution_str = match self.result_b.as_ref() {
                Some(s) => s.to_string().bold().cyan(),
                None => "TODO".to_string().dimmed(),
            };
            writeln!(f, "B : {solution_str}")?
        }
        Ok(())
    }
}
