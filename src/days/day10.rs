use std::collections::HashMap;

use crate::ProblemSolution;
#[allow(unused_imports)]
use aoc_parse::{parser, prelude::*};
pub struct Solution {}

fn parse_input(input: &str) -> Vec<usize> {
    parser!(lines(usize)).parse(input).unwrap()
}

struct ChargerArray<'a> {
    source: usize,
    array: &'a Vec<usize>,
    target: usize,
}

impl<'a> ChargerArray<'a> {
    fn init(array: &'a Vec<usize>) -> Self {
        ChargerArray {
            source: *array.first().unwrap(),
            array,
            target: *array.last().unwrap(),
        }
    }

    fn key(&self) -> (usize, usize) {
        (self.source, self.target)
    }

    fn n_paths(&self, cache: &mut HashMap<(usize, usize), usize>) -> usize {
        if self.source == self.target {
            return 1;
        }

        if let Some(answer) = cache.get(&self.key()) {
            return *answer;
        }

        let answer = [self.source + 1, self.source + 2, self.source + 3]
            .iter()
            .filter(|node| self.array.contains(node))
            .map(|new_source| {
                let downstream_arr = ChargerArray {
                    source: *new_source,
                    array: self.array,
                    target: self.target,
                };
                downstream_arr.n_paths(cache)
            })
            .sum();

        cache.insert(self.key(), answer);

        answer
    }
}

impl ProblemSolution for Solution {
    fn solve_a(&self, input: &str) -> Option<String> {
        let mut outputs = parse_input(input);
        outputs.push(0);
        outputs.sort_unstable();
        outputs.push(outputs.last().unwrap() + 3);

        let differences = outputs.windows(2).map(|pair| pair[1] - pair[0]);

        let (diff_1s, diff_3s) = differences.fold((0, 0), |acc, diff| match diff {
            1 => (acc.0 + 1, acc.1),
            3 => (acc.0, acc.1 + 1),
            _ => acc,
        });

        let answer = (diff_1s, diff_3s, diff_1s * diff_3s);

        Some(answer.2.to_string())
    }

    fn solve_b(&self, input: &str) -> Option<String> {
        let mut outputs = parse_input(input);
        outputs.push(0);
        outputs.sort_unstable();
        outputs.push(outputs.last().unwrap() + 3);

        let c_arr = ChargerArray::init(&outputs);
        let mut cache = HashMap::default();

        Some(c_arr.n_paths(&mut cache).to_string())
    }
}
