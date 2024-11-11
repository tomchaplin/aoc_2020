use std::collections::HashMap;

use crate::ProblemSolution;
#[allow(unused_imports)]
use aoc_parse::{parser, prelude::*};
pub struct Solution {}

#[derive(Debug)]
struct MemoryGame {
    current_number: usize,
    step_number: usize,
    // Records the last time a given number was spoken
    memory: HashMap<usize, usize>,
}

impl MemoryGame {
    fn init(mut numbers: Vec<usize>) -> Self {
        let step_number = numbers.len() - 1;
        let current_number = numbers.pop().unwrap();
        let memory = numbers
            .into_iter()
            .enumerate()
            .map(|(idx, number)| (number, idx))
            .collect();
        Self {
            step_number,
            memory,
            current_number,
        }
    }

    fn advance(&mut self) {
        let next_number = if let Some(last_seen_idx) = self.memory.get(&self.current_number) {
            self.step_number - last_seen_idx
        } else {
            0
        };

        self.memory.insert(self.current_number, self.step_number);
        self.current_number = next_number;
        self.step_number += 1;
    }

    fn get_nth_number(&mut self, n: usize) -> usize {
        while self.step_number != (n - 1) {
            self.advance();
        }
        self.current_number
    }
}

impl ProblemSolution for Solution {
    fn solve_a(&self, input: &str) -> Option<String> {
        let numbers = parser!(line(repeat_sep(usize, ","))).parse(input).unwrap();
        let mut game = MemoryGame::init(numbers);
        Some(game.get_nth_number(2020).to_string())
    }

    fn solve_b(&self, input: &str) -> Option<String> {
        let numbers = parser!(line(repeat_sep(usize, ","))).parse(input).unwrap();
        let mut game = MemoryGame::init(numbers);
        Some(game.get_nth_number(30000000).to_string())
    }
}
