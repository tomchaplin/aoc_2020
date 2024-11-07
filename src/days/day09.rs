use rayon::prelude::*;

use crate::ProblemSolution;
#[allow(unused_imports)]
use aoc_parse::{parser, prelude::*};
use rayon::iter::IntoParallelIterator;
pub struct Solution {}

fn parse_input(input: &str) -> Vec<usize> {
    parser!(lines(usize)).parse(input).unwrap()
}

fn populate_row(sum_grid: &mut Vec<Vec<usize>>, row: usize, numbers: &Vec<usize>) {
    let new_row = (0..row).map(|col| numbers[row] + numbers[col]).collect();
    sum_grid.push(new_row);
}

fn find_failure(numbers: &Vec<usize>) -> usize {
    let mut working_idx = 0;
    let lookback = 25;
    let mut sum_grid = vec![];
    'outer: loop {
        working_idx += 1;
        populate_row(&mut sum_grid, working_idx - 1, &numbers);
        // Premable
        if working_idx < lookback {
            continue 'outer;
        }
        // Main loop
        for i in (working_idx - lookback)..working_idx {
            for j in (working_idx - lookback)..i {
                if sum_grid[i][j] == numbers[working_idx] {
                    continue 'outer;
                }
            }
        }
        break 'outer numbers[working_idx];
    }
}

fn test_for_range_starting(
    numbers: &Vec<usize>,
    start_idx: usize,
    target: usize,
) -> Option<(usize, usize)> {
    let mut sum = numbers[start_idx];
    for end_idx in (start_idx + 1)..(numbers.len()) {
        sum += numbers[end_idx];
        if sum == target {
            return Some((start_idx, end_idx));
        }
        if sum > target {
            return None;
        }
    }
    return None;
}

impl ProblemSolution for Solution {
    fn solve_a(&self, input: &str) -> Option<String> {
        let numbers = parse_input(input);
        let failure_number = find_failure(&numbers);
        Some(failure_number.to_string())
    }

    fn solve_b(&self, input: &str) -> Option<String> {
        let numbers = parse_input(input);
        let target = find_failure(&numbers);
        let (start_idx, end_idx) = (0..numbers.len())
            .into_par_iter()
            .find_map_any(|start_idx| test_for_range_starting(&numbers, start_idx, target))
            .unwrap();

        let min_in_range = numbers[start_idx..=end_idx].iter().min().unwrap();
        let max_in_range = numbers[start_idx..=end_idx].iter().max().unwrap();

        let answer = min_in_range + max_in_range;
        Some(answer.to_string())
    }
}
