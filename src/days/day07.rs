use std::collections::HashMap;

use crate::ProblemSolution;
#[allow(unused_imports)]
use aoc_parse::{parser, prelude::*};
pub struct Solution {}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct BagType(String, String);

struct BagTree(HashMap<BagType, Vec<(usize, BagType)>>);

fn parse_input(input: &str) -> BagTree {
    let bag_type = parser!(
        word1:string(lower+) " " word2:string(lower+) => BagType(word1, word2)
    );

    let bag_or_bags = parser!({"bag", "bags"});

    let single_content = parser!(
        count:usize " " t:bag_type " " bag_or_bags => (count, t)
    );
    let all_content = parser!({
        repeat_sep(single_content,", "),
        "no other bags" => vec![]
    } "." );

    let line_parser = parser!(
        main:bag_type " bags contain " contents:all_content => (main, contents)
    );

    let p = parser!(hash_map(lines(line_parser)));
    BagTree(p.parse(input).unwrap())
}

fn contains_shiny_gold(
    query_key: &BagType,
    bag_tree: &BagTree,
    cache: &mut HashMap<BagType, bool>,
) -> bool {
    if let Some(contains) = cache.get(&query_key) {
        // Already computed
        return *contains;
    }

    let shiny_gold = BagType("shiny".to_string(), "gold".to_string());

    for (_count, bag_type) in bag_tree.0.get(&query_key).unwrap() {
        // If bag type is what we want then insert true
        if *bag_type == shiny_gold {
            cache.insert(query_key.clone(), true);
            return true;
        }
        // If any contents contains then we do
        if contains_shiny_gold(bag_type, bag_tree, cache) {
            cache.insert(query_key.clone(), true);
            return true;
        }
    }

    // No contents contain
    cache.insert(query_key.clone(), false);
    return false;
}

fn bags_inside(
    query_key: &BagType,
    bag_tree: &BagTree,
    cache: &mut HashMap<BagType, usize>,
) -> usize {
    if let Some(count) = cache.get(query_key) {
        return *count;
    }

    let contents = bag_tree
        .0
        .get(query_key)
        .unwrap()
        .iter()
        .map(|(count, sub_type)| bags_inside(sub_type, bag_tree, cache) * count + count)
        .sum();

    cache.insert(query_key.clone(), contents);

    contents
}

impl ProblemSolution for Solution {
    fn solve_a(&self, input: &str) -> Option<String> {
        let bag_tree: BagTree = parse_input(input).into();
        let all_types: Vec<_> = bag_tree.0.keys().cloned().collect();
        let mut cache: HashMap<BagType, bool> = HashMap::default();

        let n_types_contain = all_types
            .iter()
            .filter(|bag_type| contains_shiny_gold(bag_type, &bag_tree, &mut cache))
            .count();

        Some(n_types_contain.to_string())
    }

    fn solve_b(&self, input: &str) -> Option<String> {
        let bag_tree: BagTree = parse_input(input).into();
        let shiny_gold = BagType("shiny".to_string(), "gold".to_string());
        let mut cache = HashMap::default();
        let contents = bags_inside(&shiny_gold, &bag_tree, &mut cache);
        Some(contents.to_string())
    }
}
