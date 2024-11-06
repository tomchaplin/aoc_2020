use std::collections::HashSet;

use crate::ProblemSolution;
#[allow(unused_imports)]
use aoc_parse::{parser, prelude::*};
pub struct Solution {}

#[derive(Clone, Copy)]
enum Instruction {
    Acc(isize),
    Jmp(isize),
    Nop(isize),
}
use Instruction::*;

impl Instruction {
    fn swap(self) -> Self {
        match self {
            Acc(val) => Acc(val),
            Jmp(val) => Nop(val),
            Nop(val) => Jmp(val),
        }
    }
}

#[derive(Clone, Copy)]
struct MachineState {
    position: usize,
    accumulator: isize,
}

impl MachineState {
    fn init() -> Self {
        MachineState {
            position: 0,
            accumulator: 0,
        }
    }

    fn update(&self, instruction: Instruction) -> Self {
        use Instruction::*;
        match instruction {
            Acc(arg) => MachineState {
                position: self.position + 1,
                accumulator: self.accumulator + arg,
            },
            Jmp(arg) => MachineState {
                position: (self.position as isize + arg) as usize,
                accumulator: self.accumulator,
            },
            Nop(_) => MachineState {
                position: self.position + 1,
                accumulator: self.accumulator,
            },
        }
    }
}

enum SimulationOutcome {
    InfiniteLoop(isize),
    Terminates(isize),
}
use SimulationOutcome::*;

fn simulate(instructions: &Vec<Instruction>) -> SimulationOutcome {
    let mut state = MachineState::init();
    let mut prior_positions: HashSet<usize> = HashSet::default();
    loop {
        let next_instruction = instructions[state.position];
        let next_state = state.update(next_instruction);
        if prior_positions.contains(&next_state.position) {
            // Return accumulator before loop begins
            return InfiniteLoop(state.accumulator);
        } else if next_state.position == instructions.len() {
            // Return accumulator after final instruction runs
            return Terminates(next_state.accumulator);
        } else {
            state = next_state;
            prior_positions.insert(state.position);
        }
    }
}

fn parse_input(input: &str) -> Vec<Instruction> {
    let p = parser!(lines({
        "acc " arg:isize => Acc(arg),
        "jmp " arg:isize => Jmp(arg),
        "nop " arg:isize => Nop(arg)
    }));
    p.parse(input).unwrap()
}

impl ProblemSolution for Solution {
    fn solve_a(&self, input: &str) -> Option<String> {
        let instructions = parse_input(input);
        match simulate(&instructions) {
            InfiniteLoop(acc) => Some(acc.to_string()),
            Terminates(_) => unreachable!(),
        }
    }

    fn solve_b(&self, input: &str) -> Option<String> {
        let mut instructions = parse_input(input);
        for idx in 0..instructions.len() {
            instructions[idx] = instructions[idx].swap();
            match simulate(&instructions) {
                InfiniteLoop(_) => {
                    // Undo instruction swap before continuing
                    instructions[idx] = instructions[idx].swap();
                    continue;
                }
                Terminates(acc) => return Some(acc.to_string()),
            }
        }
        unreachable!()
    }
}
