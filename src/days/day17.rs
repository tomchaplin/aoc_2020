use std::{fmt::Display, iter};

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
struct PocketDimension<const N: usize> {
    states: Vec<State>,
    bounds: [usize; N],
}

fn is_valid_pos<const N: usize>(pos: &[isize; N], bounds: &[usize; N]) -> bool {
    for i in 0..N {
        if pos[i] < 0 {
            return false;
        }
        if (pos[i] as usize) >= bounds[i] {
            return false;
        }
    }
    true
}

fn position_to_idx<const N: usize>(position: &[usize; N], bounds: &[usize; N]) -> usize {
    let mut total_idx = 0;
    for i in 0..N {
        let mut slice_size: usize = bounds[(i + 1)..N].iter().product();
        if slice_size == 0 {
            slice_size = 1;
        }
        total_idx += position[i] * slice_size;
    }
    total_idx
}

fn idx_to_position<const N: usize>(mut idx: usize, bounds: &[usize; N]) -> [usize; N] {
    let mut position = vec![];
    //let starting_idx = idx;
    for i in 0..N {
        let slice_size: usize = bounds[(i + 1)..N].iter().product();
        let slice_idx = idx / slice_size;
        position.push(slice_idx);
        idx -= slice_idx * slice_size;
    }
    position.try_into().unwrap()
}

impl<const N: usize> PocketDimension<N> {
    fn is_valid_pos(&self, pos: &[isize; N]) -> bool {
        is_valid_pos(pos, &self.bounds)
    }

    // Wrap pocket dimension in a shell of Inactive
    fn pad(&mut self) {
        let new_bounds = self.bounds.clone().map(|b| b + 2);
        let mut new_states: Vec<State> = vec![];
        let max_new_idx = new_bounds.iter().product();
        for new_idx in 0..max_new_idx {
            let new_pos = idx_to_position(new_idx, &new_bounds);
            let new_pos_in_old_coords = new_pos.map(|e| e as isize - 1);
            if self.is_valid_pos(&new_pos_in_old_coords) {
                new_states.push(
                    self.at_pos(new_pos_in_old_coords.map(|e| e as usize))
                        .state(),
                )
            } else {
                new_states.push(Inactive)
            }
        }
        self.states = new_states;
        self.bounds = new_bounds;
    }

    fn position_to_idx(&self, position: &[usize; N]) -> usize {
        position_to_idx(position, &self.bounds)
    }

    fn idx_to_position(&self, idx: usize) -> [usize; N] {
        idx_to_position(idx, &self.bounds)
    }

    fn set_pos(&mut self, position: [usize; N], state: State) {
        let idx = self.position_to_idx(&position);
        self.states[idx] = state;
    }

    fn at_pos(&self, position: [usize; N]) -> PocketDimensionPosition<'_, N> {
        PocketDimensionPosition {
            dimension: self,
            position,
        }
    }

    fn update_into(&self, other: &mut Self) {
        for idx in 0..(self.states.len()) {
            let pos = self.idx_to_position(idx);
            let pos_ref = self.at_pos(pos.clone());
            let current_state = pos_ref.state();
            let active_nbrs = pos_ref
                .neighbours()
                .filter(|nbr| matches!(nbr.state(), Active))
                .count();
            let next_state = match (current_state, active_nbrs) {
                (Active, 2 | 3) => Active,
                (Inactive, 3) => Active,
                (_, _) => Inactive,
            };
            other.set_pos(pos, next_state);
        }
    }

    fn n_active(&self) -> usize {
        self.states
            .iter()
            .filter(|state| matches!(state, Active))
            .count()
    }
}

struct PocketDimensionPosition<'a, const N: usize> {
    dimension: &'a PocketDimension<N>,
    position: [usize; N],
}

impl<'a, const N: usize> PocketDimensionPosition<'a, N> {
    fn shift_pos(&self, delta: [isize; N]) -> Option<Self> {
        let new_pos: [isize; N] = self
            .position
            .iter()
            .zip(delta.iter())
            .map(|(x, y)| (*x as isize + y))
            .collect::<Vec<isize>>()
            .try_into()
            .unwrap();
        if self.dimension.is_valid_pos(&new_pos) {
            let new_pos_as_usize: [usize; N] = new_pos.map(|e| e as usize);
            Some(PocketDimensionPosition {
                dimension: self.dimension,
                position: new_pos_as_usize,
            })
        } else {
            None
        }
    }

    fn state(&self) -> State {
        let idx = self.dimension.position_to_idx(&self.position);
        self.dimension.states[idx]
    }

    fn neighbours<'b>(&'b self) -> impl Iterator<Item = PocketDimensionPosition<'b, N>> {
        let delta_bounds = [3; N];
        let max_delta_idx = delta_bounds.iter().product();
        (0..max_delta_idx)
            .map(move |delta_idx| idx_to_position(delta_idx, &delta_bounds).map(|e| e as isize - 1))
            .filter(|delta| !delta.iter().all(|&e| e == 0))
            .filter_map(|delta| self.shift_pos(delta))
    }
}

fn parse_input<const N: usize>(input: &str) -> PocketDimension<N> {
    let p = parser!(lines({
        "#" => Active,
        "." => Inactive
    }+));
    let slice = p.parse(input).unwrap();
    let bounds: [usize; N] = (0..(N - 2))
        .map(|_| 1)
        .chain(iter::once(slice[0].len()))
        .chain(iter::once(slice.len()))
        .collect::<Vec<usize>>()
        .try_into()
        .unwrap();
    PocketDimension {
        states: slice.into_iter().flat_map(|row| row.into_iter()).collect(),
        bounds,
    }
}

impl ProblemSolution for Solution {
    fn solve_a(&self, input: &str) -> Option<String> {
        let mut dim_1 = parse_input::<3>(input);
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
        let mut dim_1 = parse_input::<4>(input);
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
}
