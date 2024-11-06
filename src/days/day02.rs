use crate::ProblemSolution;
#[allow(unused_imports)]
use aoc_parse::{parser, prelude::*};
pub struct Solution {}

struct PasswordLine {
    min: usize,
    max: usize,
    character: char,
    password: Vec<char>,
}

impl PasswordLine {
    fn is_valid_a(&self) -> bool {
        let mut count = 0;
        for c in self.password.iter() {
            if *c == self.character {
                count += 1;
            }
        }
        if count < self.min {
            false
        } else if count > self.max {
            false
        } else {
            true
        }
    }

    fn is_valid_b(&self) -> bool {
        let mut matches = 0;
        if self.password[self.min - 1] == self.character {
            matches += 1;
        }
        if self.password[self.max - 1] == self.character {
            matches += 1;
        }
        matches == 1
    }
}

fn get_lines(input: &str) -> impl Iterator<Item = PasswordLine> {
    let p = parser!(lines(
        min:usize "-" max:usize " " character:lower ": " password:lower+
        => PasswordLine { min , max, character, password }
    ));
    p.parse(input).unwrap().into_iter()
}

impl ProblemSolution for Solution {
    fn solve_a(&self, input: &str) -> Option<String> {
        let password_lines = get_lines(&input);
        let n_valid = password_lines.filter(|line| line.is_valid_a()).count();
        Some(n_valid.to_string())
    }

    fn solve_b(&self, input: &str) -> Option<String> {
        let password_lines = get_lines(&input);
        let n_valid = password_lines.filter(|line| line.is_valid_b()).count();
        Some(n_valid.to_string())
    }
}
