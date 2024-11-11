use std::collections::HashSet;

use crate::ProblemSolution;
#[allow(unused_imports)]
use aoc_parse::{parser, prelude::*};
pub struct Solution {}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct ValidRange(usize, usize);

impl ValidRange {
    fn contains(&self, value: &usize) -> bool {
        *value >= self.0 && *value <= self.1
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct FieldRule {
    name: String,
    ranges: [ValidRange; 2],
}

impl FieldRule {
    fn contains(&self, value: &usize) -> bool {
        self.ranges[0].contains(value) || self.ranges[1].contains(value)
    }
}

#[derive(Clone, Debug)]
struct Ticket(Vec<usize>);

impl Ticket {
    fn get_invalid_fields<'a>(
        &'a self,
        rules: &'a Vec<FieldRule>,
    ) -> impl Iterator<Item = usize> + 'a {
        self.0
            .iter()
            .filter(|num| !rules.iter().any(|r| r.contains(num)))
            .copied()
    }
}

fn parse_input(input: &str) -> (Vec<FieldRule>, Ticket, Vec<Ticket>) {
    let range_parser = parser!(min:usize "-" max:usize => ValidRange(min, max));
    let rule_parser = parser!(
        name:string(any_char+) ": " r1:range_parser " or " r2:range_parser
        => FieldRule{ name, ranges: [r1, r2] }
    );

    let ticket_parser = parser!(nums:repeat_sep(usize, ",") => Ticket(nums));

    let p = parser!(
        section(lines(rule_parser))
        section(
            line("your ticket:")
            ticket:line(ticket_parser)
            => ticket
        )
        section(
            line("nearby tickets:")
            tickets:lines(ticket_parser)
            => tickets
        )
    );

    p.parse(input).unwrap()
}

// Surely one of the fields must now be fixed
// Remove this from any other possibilities and keep iterating
// Eventually we arrive at the only valid permutation
fn find_only_permutation(mut possibilities: Vec<HashSet<FieldRule>>) -> Vec<FieldRule> {
    let mut fixed: HashSet<usize> = HashSet::new();
    while possibilities.iter().any(|s| s.len() > 1) {
        let newly_fixed: Vec<_> = possibilities
            .iter()
            .enumerate()
            .filter(|(idx, p_set)| (!fixed.contains(idx)) && p_set.len() == 1)
            .map(|(idx, p_set)| (idx, p_set.iter().next().cloned().unwrap()))
            .collect();
        for (fixed_idx, fixed_field) in newly_fixed {
            fixed.insert(fixed_idx);
            for (other_idx, p_set) in possibilities.iter_mut().enumerate() {
                if other_idx == fixed_idx {
                    continue;
                }
                p_set.remove(&fixed_field);
            }
        }
    }

    possibilities
        .into_iter()
        .map(|mut p_set| p_set.drain().next().unwrap())
        .collect()
}

fn build_possible_field_names<'a>(
    rules: &Vec<FieldRule>,
    tickets: impl Iterator<Item = &'a Ticket>,
    n_fields: usize,
) -> Vec<HashSet<FieldRule>> {
    // possibilities[i] represents the possible fields that could be in position i
    let mut possibilities: Vec<HashSet<FieldRule>> =
        vec![rules.iter().cloned().collect(); n_fields];

    // Filter out which rules are possible based on ranges
    for ticket in tickets {
        for (idx, num) in ticket.0.iter().enumerate() {
            possibilities[idx].retain(|rule| rule.contains(num));
        }
    }
    possibilities
}

impl ProblemSolution for Solution {
    fn solve_a(&self, input: &str) -> Option<String> {
        let (rules, _my_ticket, nearby_tickets) = parse_input(input);
        let error_rate: usize = nearby_tickets
            .iter()
            .flat_map(|ticket| ticket.get_invalid_fields(&rules))
            .sum();
        Some(error_rate.to_string())
    }

    fn solve_b(&self, input: &str) -> Option<String> {
        let (rules, my_ticket, nearby_tickets) = parse_input(input);
        let n_fields = my_ticket.0.len();
        let valid_nearby = nearby_tickets
            .iter()
            .filter(|ticket| ticket.get_invalid_fields(&rules).next().is_none());

        // Use rules to filter out which field name could be in each position
        let possibilities = build_possible_field_names(&rules, valid_nearby, n_fields);
        // Find the only possible combination of names based on the above
        let field_names = find_only_permutation(possibilities);

        let departure_idxs = field_names
            .iter()
            .enumerate()
            .filter(|(_idx, rule)| rule.name.starts_with("departure"))
            .map(|(idx, _rule)| idx);

        let departure_values = departure_idxs.map(|idx| my_ticket.0[idx]);
        let answer: usize = departure_values.product();

        Some(answer.to_string())
    }
}
