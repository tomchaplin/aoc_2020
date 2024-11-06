use std::collections::HashSet;

use crate::ProblemSolution;
#[allow(unused_imports)]
use aoc_parse::{parser, prelude::*};
pub struct Solution {}

struct PersonAnswers(Vec<char>);
struct GroupAnswers(Vec<PersonAnswers>);

impl GroupAnswers {
    fn union_size(&self) -> usize {
        let union: HashSet<_> = self
            .0
            .iter()
            .map(|person| person.0.iter())
            .flatten()
            .collect();
        union.len()
    }

    fn intersection_size(&self) -> usize {
        let person_sets: Vec<HashSet<&char>> = self
            .0
            .iter()
            .map(|person| person.0.iter().collect())
            .collect();
        person_sets[0]
            .iter()
            .filter(|&answer| person_sets[1..].iter().all(|s| s.contains(answer)))
            .count()
    }
}

fn parse_input(input: &str) -> Vec<GroupAnswers> {
    let person_parser = parser!(yeses:lower+ => PersonAnswers(yeses));
    let group_parser = parser!(people:lines(person_parser) => GroupAnswers(people));
    let p = parser!(sections(group_parser));
    p.parse(input).unwrap()
}

impl ProblemSolution for Solution {
    fn solve_a(&self, input: &str) -> Option<String> {
        let groups = parse_input(input);
        let total: usize = groups.into_iter().map(|grp| grp.union_size()).sum();
        Some(total.to_string())
    }

    fn solve_b(&self, input: &str) -> Option<String> {
        let groups = parse_input(input);
        let total: usize = groups.into_iter().map(|grp| grp.intersection_size()).sum();
        Some(total.to_string())
    }
}
