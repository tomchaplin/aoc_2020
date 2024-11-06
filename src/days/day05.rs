use crate::ProblemSolution;
#[allow(unused_imports)]
use aoc_parse::{parser, prelude::*};
pub struct Solution {}

#[derive(Debug)]
struct Seat {
    row: usize,
    column: usize,
}

impl Seat {
    fn id(&self) -> usize {
        self.row * 8 + self.column
    }
}

fn binary_to_usize(binary: Vec<usize>) -> usize {
    let mut count = 0;
    let mut power = 0;
    let base: usize = 2;
    for next_digit in binary.into_iter().rev() {
        count += next_digit * base.pow(power);
        power += 1;
    }
    count
}

fn parse_input(input: &str) -> Vec<Seat> {
    // Give 0 to F, 1 to B
    let fb_parser = parser!(char_of("FB"));
    // Give 0 to L, 1 to R
    let lr_parser = parser!(char_of("LR"));

    let row_parser = parser!(dgs:fb_parser+ => binary_to_usize(dgs));
    let column_parser = parser!(dgs:lr_parser+ => binary_to_usize(dgs));
    let seat_parser = parser!(row:row_parser column:column_parser => Seat{row, column});

    parser!(lines(seat_parser)).parse(input).unwrap()
}

impl ProblemSolution for Solution {
    fn solve_a(&self, input: &str) -> Option<String> {
        let seats = parse_input(input);
        let max_id = seats.into_iter().map(|s| s.id()).max().unwrap();
        Some(max_id.to_string())
    }

    fn solve_b(&self, input: &str) -> Option<String> {
        let seats = parse_input(input);
        let mut all_ids: Vec<_> = seats.into_iter().map(|s| s.id()).collect();
        all_ids.sort_unstable();
        let mut previous_id = all_ids[0] - 1;
        for next_id in all_ids {
            if next_id != previous_id + 1 {
                let my_id = previous_id + 1;
                return Some(my_id.to_string());
            }
            previous_id = next_id;
        }
        unreachable!()
    }
}
