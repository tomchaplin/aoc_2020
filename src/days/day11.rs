use std::{env, fmt::Display, hash::Hash, thread::sleep, time::Duration};

use crate::ProblemSolution;
#[allow(unused_imports)]
use aoc_parse::{parser, prelude::*};
use colored::Colorize;
pub struct Solution {}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
enum SeatState {
    Floor,
    Empty,
    Occupied,
}

impl SeatState {
    fn is_occupied(&self) -> bool {
        matches!(self, SeatState::Occupied)
    }
    fn not_occupied(&self) -> bool {
        !self.is_occupied()
    }
}

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
struct GridState {
    grid: Vec<Vec<SeatState>>,
    bounds: (usize, usize),
}

impl Display for GridState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.grid.iter() {
            for entry in row.iter() {
                write!(
                    f,
                    "{}",
                    match entry {
                        SeatState::Floor => ".".dimmed(),
                        SeatState::Empty => "L".dimmed(),
                        SeatState::Occupied => "#".red(),
                    }
                )?
            }
            writeln!(f, "")?
        }
        Ok(())
    }
}

impl GridState {
    fn at_position_unchecked(&self, position: (usize, usize)) -> GridPosition<'_> {
        GridPosition {
            position,
            grid: self,
        }
    }

    fn at_position(&self, position: (isize, isize)) -> Option<GridPosition<'_>> {
        if self.in_bounds(&(position.0, position.1)) {
            Some(self.at_position_unchecked((position.0 as usize, position.1 as usize)))
        } else {
            None
        }
    }

    fn in_bounds(&self, position: &(isize, isize)) -> bool {
        position.0 >= 0
            && (position.0 as usize) < self.bounds.0
            && position.1 >= 0
            && (position.1 as usize) < self.bounds.1
    }

    fn iter_pos<'a>(&'a self) -> impl Iterator<Item = (usize, usize)> + 'a {
        (0..self.bounds.0).flat_map(|i| (0..self.bounds.1).map(move |j| (i, j)))
    }

    // TODO: In theory, we ought to be able to parallelise this

    fn update_into_grid(&self, other_grid: &mut GridState) {
        for (i, j) in self.iter_pos() {
            other_grid.grid[i][j] = self.at_position_unchecked((i, j)).next_state();
        }
    }

    fn update_into_grid_b(&self, other_grid: &mut GridState) {
        for (i, j) in self.iter_pos() {
            other_grid.grid[i][j] = self.at_position_unchecked((i, j)).next_state_b();
        }
    }

    fn n_occupied(&self) -> usize {
        self.iter_pos()
            .filter(|pos| self.at_position_unchecked(*pos).state().is_occupied())
            .count()
    }
}

struct GridPosition<'a> {
    position: (usize, usize),
    grid: &'a GridState,
}

impl<'a> GridPosition<'a> {
    fn state(&self) -> SeatState {
        self.grid.grid[self.position.0][self.position.1]
    }

    fn neighbours<'b>(&'b self) -> impl Iterator<Item = GridPosition<'b>> {
        vec![-1, 0, 1]
            .into_iter()
            .flat_map(|dx| vec![-1, 0, 1].clone().into_iter().map(move |dy| (dx, dy)))
            .filter(|(dx, dy)| (*dx, *dy) != (0, 0))
            .map(|(dx, dy)| (self.position.0 as isize + dx, self.position.1 as isize + dy))
            .filter(|new_pos| self.grid.in_bounds(new_pos))
            .map(|new_pos| {
                self.grid
                    .at_position_unchecked((new_pos.0 as usize, new_pos.1 as usize))
            })
    }

    fn next_state(&self) -> SeatState {
        match self.state() {
            SeatState::Floor => SeatState::Floor,
            SeatState::Empty => {
                if self.neighbours().all(|n| n.state().not_occupied()) {
                    SeatState::Occupied
                } else {
                    SeatState::Empty
                }
            }
            SeatState::Occupied => {
                let n_occupied_neighbours = self
                    .neighbours()
                    .filter(|n| n.state().is_occupied())
                    .count();
                if n_occupied_neighbours >= 4 {
                    SeatState::Empty
                } else {
                    SeatState::Occupied
                }
            }
        }
    }

    fn can_see_occupied(&self, direction: (isize, isize)) -> bool {
        let mut checking = (self.position.0 as isize, self.position.1 as isize);
        loop {
            checking.0 += direction.0;
            checking.1 += direction.1;
            if let Some(other_pos) = self.grid.at_position(checking) {
                match other_pos.state() {
                    SeatState::Floor => continue,
                    SeatState::Empty => return false,
                    SeatState::Occupied => return true,
                }
                // Otherwise we need to carry on checking this line of sight
            } else {
                // Reached bounds without finding anything
                return false;
            }
        }
    }

    fn occupied_directions_in_view(&self) -> usize {
        vec![-1, 0, 1]
            .into_iter()
            .flat_map(|dx| vec![-1, 0, 1].clone().into_iter().map(move |dy| (dx, dy)))
            .filter(|(dx, dy)| (*dx, *dy) != (0, 0))
            .filter(|dir| self.can_see_occupied(*dir))
            .count()
    }

    fn next_state_b(&self) -> SeatState {
        match self.state() {
            SeatState::Floor => SeatState::Floor,
            SeatState::Empty => {
                let dirs_occupied = self.occupied_directions_in_view();
                if dirs_occupied == 0 {
                    SeatState::Occupied
                } else {
                    SeatState::Empty
                }
            }
            SeatState::Occupied => {
                let dirs_occupied = self.occupied_directions_in_view();
                if dirs_occupied >= 5 {
                    SeatState::Empty
                } else {
                    SeatState::Occupied
                }
            }
        }
    }
}

fn parse_input(input: &str) -> GridState {
    let pos_parser = parser!({
        "." => SeatState::Floor,
        "L" => SeatState::Empty
    });
    let grid_parser = parser!(lines(pos_parser+));
    let grid = grid_parser.parse(input).unwrap();
    let bounds = (grid.len(), grid[0].len());
    GridState { grid, bounds }
}

fn check_should_print() -> bool {
    env::var("PRINT_GRID").is_ok_and(|val| val == "true")
}

impl ProblemSolution for Solution {
    fn solve_a(&self, input: &str) -> Option<String> {
        let should_print = check_should_print();
        let mut previous_grid = parse_input(input);
        let mut next_grid = previous_grid.clone();

        loop {
            if should_print {
                println!("{}[2J{previous_grid}", 27 as char);
            }
            previous_grid.update_into_grid(&mut next_grid);
            if should_print {
                sleep(Duration::from_millis(100));
            }
            if next_grid == previous_grid {
                break;
            }
            // Vec already has indirection so should be cheap
            std::mem::swap(&mut previous_grid, &mut next_grid);
        }
        if should_print {
            sleep(Duration::from_secs(5));
        }
        Some(next_grid.n_occupied().to_string())
    }

    fn solve_b(&self, input: &str) -> Option<String> {
        let should_print = check_should_print();
        let mut previous_grid = parse_input(input);
        let mut next_grid = previous_grid.clone();
        loop {
            if should_print {
                println!("{}[2J{previous_grid}", 27 as char);
            }
            previous_grid.update_into_grid_b(&mut next_grid);
            if should_print {
                sleep(Duration::from_millis(100));
            }
            if next_grid == previous_grid {
                break;
            }
            // Vec already has indirection so should be cheap
            std::mem::swap(&mut previous_grid, &mut next_grid);
        }
        if should_print {
            sleep(Duration::from_secs(5));
        }
        Some(next_grid.n_occupied().to_string())
    }
}
