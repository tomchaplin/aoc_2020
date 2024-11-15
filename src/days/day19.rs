use std::collections::{HashMap, HashSet};

use crate::ProblemSolution;
#[allow(unused_imports)]
use aoc_parse::{parser, prelude::*};
use itertools::Itertools;
pub struct Solution {}

#[derive(Debug, PartialEq, Eq)]
enum ABChar {
    A,
    B,
}

#[derive(Debug)]
enum RawRule {
    MatchChar(ABChar),
    MatchRuleSequences(Vec<Vec<usize>>),
}

#[derive(Debug)]
struct RuleSet {
    rules: HashMap<usize, RawRule>,
}

impl RuleSet {
    fn rule(&self, idx: usize) -> Rule<'_> {
        Rule {
            rule_set: self,
            idx,
        }
    }
}

struct Rule<'a> {
    rule_set: &'a RuleSet,
    idx: usize,
}

impl<'a> Rule<'a> {
    // Each element of the returned set is a possible number of characters that could be matched
    // by the rule
    fn match_lengths(&self, word: &[ABChar]) -> HashSet<usize> {
        if word.len() == 0 {
            return HashSet::default();
        }
        let raw_rule = self.rule_set.rules.get(&self.idx).unwrap();
        match raw_rule {
            RawRule::MatchChar(abchar) => {
                // Base case
                if word.len() >= 1 && word[0] == *abchar {
                    return HashSet::from([1]);
                } else {
                    return HashSet::default();
                }
            }
            RawRule::MatchRuleSequences(rule_sequences) => rule_sequences
                .iter()
                .flat_map(|rule_sequence| {
                    // Each rule sequence is checked individually and then merged
                    let rule_sequence = RuleSequence {
                        rule_set: &self.rule_set,
                        sequence: &rule_sequence,
                    };
                    rule_sequence.match_lengths(word).into_iter()
                })
                .collect(),
        }
    }

    fn fully_matches(&self, word: &[ABChar]) -> bool {
        self.match_lengths(word).contains(&word.len())
    }
}

// Quick wrapper to enable recursive checking of a sequence of rules
// that must match one after the other
struct RuleSequence<'a> {
    rule_set: &'a RuleSet,
    sequence: &'a [usize],
}

impl<'a> RuleSequence<'a> {
    // Each element of the returned set is a possible number of characters that could be matched
    // by the rule sequence
    fn match_lengths(&self, word: &[ABChar]) -> HashSet<usize> {
        if self.sequence.len() == 1 {
            self.rule_set.rule(self.sequence[0]).match_lengths(word)
        } else {
            let first_rule = self.rule_set.rule(self.sequence[0]);
            let rest_of_seq = RuleSequence {
                rule_set: self.rule_set,
                sequence: &self.sequence[1..],
            };
            first_rule
                .match_lengths(word)
                .into_iter()
                .flat_map(|n_first_matches| {
                    rest_of_seq
                        .match_lengths(&word[n_first_matches..])
                        .into_iter()
                        .map(move |n_rest_matches| n_first_matches + n_rest_matches)
                })
                .collect()
        }
    }
}

fn parse_input(input: &str) -> (RuleSet, Vec<Vec<ABChar>>) {
    let abchar = parser!({'a' => ABChar::A, 'b' => ABChar::B});
    let rule_parser = parser!(
        idx:usize ": " raw_rule:{
            any_char c:abchar any_char  => RawRule::MatchChar(c),
            rules:repeat_sep(repeat_sep(usize, " "), " | ")
            => RawRule::MatchRuleSequences(rules),
        }
    );
    let word_parser = parser!(abchar+);

    let p = parser!(
        section(lines(rule_parser))
        section(lines(word_parser))
    );

    let (rules, words) = p.parse(input).unwrap();
    let rule_set = RuleSet {
        rules: rules.into_iter().collect(),
    };
    (rule_set, words)
}

impl ProblemSolution for Solution {
    fn solve_a(&self, input: &str) -> Option<String> {
        let (rule_set, words) = parse_input(input);
        let rule_0 = rule_set.rule(0);
        let words_matching_0 = words
            .into_iter()
            .filter(|word| rule_0.fully_matches(word))
            .count();
        Some(words_matching_0.to_string())
    }

    fn solve_b(&self, input: &str) -> Option<String> {
        let new_lines = input.lines().map(|line| {
            if line.starts_with("8:") {
                "8: 42 | 42 8"
            } else if line.starts_with("11:") {
                "11: 42 31 | 42 11 31"
            } else {
                line
            }
        });
        let new_input: String = Itertools::intersperse(new_lines, "\n").collect();

        self.solve_a(&new_input)
    }
}
