use std::fmt::Display;

use crate::ProblemSolution;
#[allow(unused_imports)]
use aoc_parse::{parser, prelude::*};
pub struct Solution {}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum State {
    Inactive,
    Active,
}
use State::*;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct PocketDimension {
    states: Vec<Vec<Vec<State>>>,
    bounds: (usize, usize, usize),
}

impl Display for PocketDimension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, slice) in self.states.iter().enumerate() {
            writeln!(f, "z={i}")?;
            for row in slice {
                for state in row {
                    write!(
                        f,
                        "{}",
                        match state {
                            Inactive => ".",
                            Active => "#",
                        }
                    )?
                }
                writeln!(f, "")?;
            }
        }
        Ok(())
    }
}

fn pad_slice(slice: &mut Vec<Vec<State>>) {
    let width = slice.len();
    let top_and_bottom: Vec<_> = (0..(width + 2)).map(|_| Inactive).collect();
    for line in slice.iter_mut() {
        line.insert(0, Inactive);
        line.push(Inactive);
    }
    slice.insert(0, top_and_bottom.clone());
    slice.push(top_and_bottom);
}

impl PocketDimension {
    fn is_valid_pos(&self, pos: (isize, isize, isize)) -> bool {
        pos.0 >= 0
            && pos.1 >= 0
            && pos.2 >= 0
            && (pos.0 as usize) < self.bounds.0
            && (pos.1 as usize) < self.bounds.1
            && (pos.2 as usize) < self.bounds.2
    }

    // Wrap pocket dimension in a shell of Inactive
    fn pad(&mut self) {
        let new_bounds = (self.bounds.0 + 2, self.bounds.1 + 2, self.bounds.2 + 2);
        let empty_slice: Vec<Vec<State>> = (0..new_bounds.1)
            .map(|_| (0..new_bounds.2).map(|_| Inactive).collect())
            .collect();
        for slice in self.states.iter_mut() {
            pad_slice(slice);
        }
        self.states.push(empty_slice.clone());
        self.states.insert(0, empty_slice);
        self.bounds = new_bounds;
    }

    fn set_pos(&mut self, position: (usize, usize, usize), state: State) {
        self.states[position.0][position.1][position.2] = state;
    }

    fn at_pos(&self, position: (usize, usize, usize)) -> PocketDimensionPosition<'_> {
        PocketDimensionPosition {
            dimension: self,
            position,
        }
    }

    fn update_into(&self, other: &mut PocketDimension) {
        for i in 0..self.bounds.0 {
            for j in 0..self.bounds.1 {
                for k in 0..self.bounds.2 {
                    let pos_ijk = self.at_pos((i, j, k));
                    let current_state = pos_ijk.state();
                    let active_nbrs = pos_ijk
                        .neighbours()
                        .filter(|nbr| matches!(nbr.state(), Active))
                        .count();
                    let next_state = match (current_state, active_nbrs) {
                        (Active, 2 | 3) => Active,
                        (Inactive, 3) => Active,
                        (_, _) => Inactive,
                    };
                    other.set_pos((i, j, k), next_state);
                }
            }
        }
    }

    fn n_active(&self) -> usize {
        self.states
            .iter()
            .flat_map(|slice| slice.iter())
            .flat_map(|row| row.iter())
            .filter(|state| matches!(state, Active))
            .count()
    }
}

struct PocketDimensionPosition<'a> {
    dimension: &'a PocketDimension,
    position: (usize, usize, usize),
}

impl<'a> PocketDimensionPosition<'a> {
    fn shift_pos(&self, delta: (isize, isize, isize)) -> Option<PocketDimensionPosition<'a>> {
        let new_x = self.position.0 as isize + delta.0;
        let new_y = self.position.1 as isize + delta.1;
        let new_z = self.position.2 as isize + delta.2;
        if self.dimension.is_valid_pos((new_x, new_y, new_z)) {
            Some(PocketDimensionPosition {
                dimension: self.dimension,
                position: (new_x as usize, new_y as usize, new_z as usize),
            })
        } else {
            None
        }
    }

    fn state(&self) -> State {
        self.dimension.states[self.position.0][self.position.1][self.position.2]
    }

    fn neighbours<'b>(&'b self) -> impl Iterator<Item = PocketDimensionPosition<'b>> {
        let deltas: [isize; 3] = [-1, 0, 1];
        deltas
            .into_iter()
            .flat_map(move |dx| deltas.into_iter().map(move |dy| (dx, dy)))
            .flat_map(move |(dx, dy)| deltas.into_iter().map(move |dz| (dx, dy, dz)))
            .filter(|&delta| delta != (0, 0, 0))
            .filter_map(|delta| self.shift_pos(delta))
    }
}

fn parse_input(input: &str) -> PocketDimension {
    let p = parser!(lines({
        "#" => Active,
        "." => Inactive
    }+));
    let slice = p.parse(input).unwrap();
    let bounds = (1, slice.len(), slice[0].len());
    PocketDimension {
        states: vec![slice],
        bounds,
    }
}

impl ProblemSolution for Solution {
    fn solve_a(&self, input: &str) -> Option<String> {
        let mut dim_1 = parse_input(input);
        let mut dim_2 = dim_1.clone();

        for _ in 0..6 {
            dim_1.pad();
            dim_2.pad();
            dim_1.update_into(&mut dim_2);
            std::mem::swap(&mut dim_1, &mut dim_2);
        }

        let answer = dim_1.n_active();
        Some(answer.to_string())
    }

    fn solve_b(&self, input: &str) -> Option<String> {
        None
    }
}
