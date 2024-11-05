use crate::ProblemSolution;
#[allow(unused_imports)]
use aoc_parse::{parser, prelude::*};
pub struct Solution {}

enum Space {
    Empty,
    Tree,
}

impl From<char> for Space {
    fn from(value: char) -> Self {
        match value {
            '.' => Space::Empty,
            '#' => Space::Tree,
            _ => panic!("Unrecognised"),
        }
    }
}

fn run_slope_experiment(lines: &Vec<Vec<Space>>, slope: (usize, usize)) -> usize {
    let height = lines.len();
    let width = lines[0].len();
    let mut x: usize = 0;
    let mut y: usize = 0;
    let mut trees = 0;
    while y < height {
        if matches!(lines[y][x], Space::Tree) {
            trees += 1
        }
        x += slope.0;
        y += slope.1;
        x = x.rem_euclid(width);
    }
    trees
}

impl ProblemSolution for Solution {
    fn solve_a(&self, input: &str) -> Option<String> {
        let p = parser!(lines({"." => Space::Empty, "#" => Space::Tree}+));
        let lines = p.parse(input).unwrap();
        let trees = run_slope_experiment(&lines, (3, 1));
        Some(trees.to_string())
    }

    fn solve_b(&self, input: &str) -> Option<String> {
        let p = parser!(lines({"." => Space::Empty, "#" => Space::Tree}+));
        let lines = p.parse(input).unwrap();

        let mut answer = 1;
        answer *= run_slope_experiment(&lines, (1, 1));
        answer *= run_slope_experiment(&lines, (3, 1));
        answer *= run_slope_experiment(&lines, (5, 1));
        answer *= run_slope_experiment(&lines, (7, 1));
        answer *= run_slope_experiment(&lines, (1, 2));

        Some(answer.to_string())
    }
}
