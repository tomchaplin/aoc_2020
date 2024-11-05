use std::collections::{HashMap, HashSet};

use crate::ProblemSolution;
#[allow(unused_imports)]
use aoc_parse::{parser, prelude::*};
pub struct Solution {}

impl ProblemSolution for Solution {
    fn solve_a(&self, input: &str) -> Option<String> {
        let p = parser!(lines(u64));
        let entries = p.parse(input).unwrap();
        let entries_set: HashSet<_> = entries.iter().cloned().collect();
        let mut answer = 0;
        for entry in entries {
            if entries_set.contains(&(2020 - entry)) {
                answer = entry * (2020 - entry);
                break;
            }
        }
        Some(answer.to_string())
    }

    fn solve_b(&self, input: &str) -> Option<String> {
        let p = parser!(lines(u64));
        let entries = p.parse(input).unwrap();
        let mut pair_map = HashMap::new();
        for e1 in entries.iter() {
            for e2 in entries.iter() {
                pair_map.insert(e1 + e2, (e1, e2));
            }
        }
        let mut answer = 0;
        for e3 in entries.iter() {
            match pair_map.get(&(2020 - e3)) {
                Some((e1, e2)) => {
                    answer = *e1 * *e2 * e3;
                    break;
                }
                None => continue,
            }
        }
        Some(answer.to_string())
    }
}
