use crate::ProblemSolution;
#[allow(unused_imports)]
use aoc_parse::{parser, prelude::*};
pub struct Solution {}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Left(usize),
    Right(usize),
    Forward(usize),
    North(usize),
    East(usize),
    South(usize),
    West(usize),
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn rotate_left(self, count: usize) -> Direction {
        if count == 0 {
            return self;
        }
        let new_direction = match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        };
        new_direction.rotate_left(count - 1)
    }

    fn rotate_right(self, count: usize) -> Direction {
        self.rotate_left(4 - count)
    }
}

#[derive(Debug)]
struct ShipState {
    position: (isize, isize),
    direction: Direction,
}

impl ShipState {
    fn init() -> ShipState {
        ShipState {
            position: (0, 0),
            direction: Direction::East,
        }
    }

    fn update(&mut self, instruction: Instruction) {
        use Instruction::*;
        match instruction {
            Left(count) => self.direction = self.direction.rotate_left(count),
            Right(count) => self.direction = self.direction.rotate_right(count),
            North(count) => self.position.1 += count as isize,
            East(count) => self.position.0 += count as isize,
            South(count) => self.position.1 -= count as isize,
            West(count) => self.position.0 -= count as isize,
            Forward(count) => {
                let replacement = match self.direction {
                    Direction::North => North(count),
                    Direction::East => East(count),
                    Direction::South => South(count),
                    Direction::West => West(count),
                };
                self.update(replacement)
            }
        }
    }

    fn manhattan_norm(&self) -> usize {
        (self.position.0.abs() + self.position.1.abs()) as usize
    }
}

#[derive(Debug)]
struct ShipStateWithWaypoint {
    position: (isize, isize),
    waypoint: (isize, isize),
}

impl ShipStateWithWaypoint {
    fn init() -> Self {
        Self {
            position: (0, 0),
            waypoint: (10, 1),
        }
    }

    fn update(&mut self, instruction: Instruction) {
        use Instruction::*;
        match instruction {
            Left(count) => {
                if count == 0 {
                    return ();
                }
                self.waypoint = (-self.waypoint.1, self.waypoint.0);
                self.update(Left(count - 1))
            }
            Right(count) => self.update(Left(4 - count)),
            North(count) => self.waypoint.1 += count as isize,
            East(count) => self.waypoint.0 += count as isize,
            South(count) => self.waypoint.1 -= count as isize,
            West(count) => self.waypoint.0 -= count as isize,
            Forward(count) => {
                if count == 0 {
                    return ();
                }
                self.position.0 += self.waypoint.0;
                self.position.1 += self.waypoint.1;
                self.update(Forward(count - 1))
            }
        }
    }

    fn manhattan_norm(&self) -> usize {
        (self.position.0.abs() + self.position.1.abs()) as usize
    }
}

fn parse_input(input: &str) -> Vec<Instruction> {
    use Instruction::*;
    let p = parser!(lines({
        "L" num:usize => Left(num/90),
        "R" num:usize => Right(num/90),
        "F" num:usize => Forward(num),
        "N" num:usize => North(num),
        "E" num:usize => East(num),
        "S" num:usize => South(num),
        "W" num:usize => West(num),
    }));

    p.parse(input).unwrap()
}

impl ProblemSolution for Solution {
    fn solve_a(&self, input: &str) -> Option<String> {
        let instructions = parse_input(input);
        let mut ship = ShipState::init();
        for instruction in instructions {
            ship.update(instruction);
        }
        Some(ship.manhattan_norm().to_string())
    }

    fn solve_b(&self, input: &str) -> Option<String> {
        let instructions = parse_input(input);
        let mut ship = ShipStateWithWaypoint::init();
        for instruction in instructions {
            ship.update(instruction);
        }
        Some(ship.manhattan_norm().to_string())
    }
}
