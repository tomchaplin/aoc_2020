use std::collections::{HashMap, HashSet};

use crate::ProblemSolution;
#[allow(unused_imports)]
use aoc_parse::{parser, prelude::*};
pub struct Solution {}

type MaskValue = Option<bool>;
type Mask = Vec<MaskValue>;
type BinaryNumber = Vec<bool>;

enum Instruction {
    SetMask(Mask),
    SetMemory(u64, u64),
}

fn apply_mask(number: BinaryNumber, mask: &Mask) -> BinaryNumber {
    let digits = number.into_iter();
    let mask_values = mask.iter();
    digits
        .zip(mask_values)
        .map(|(dig, &msk)| match msk {
            Some(val) => val,
            None => dig,
        })
        .collect()
}

fn apply_mask_to_address(address: BinaryNumber, mask: &Mask) -> Mask {
    let digits = address.into_iter();
    let mask_values = mask.iter();
    digits
        .zip(mask_values)
        .map(|(dig, &msk)| match msk {
            Some(false) => Some(dig),
            Some(true) => Some(true),
            None => None,
        })
        .collect()
}

fn mask_to_all_addresses(mask: &mut Mask) -> HashSet<u64> {
    let first_unknown_idx = mask.iter().position(|itm| itm.is_none());
    if let Some(idx) = first_unknown_idx {
        mask[idx] = Some(true);
        let with_true_adds = mask_to_all_addresses(mask);
        mask[idx] = Some(false);
        let with_false_adds = mask_to_all_addresses(mask);
        // Restore mask before propogating back up
        mask[idx] = None;
        with_true_adds.union(&with_false_adds).cloned().collect()
    } else {
        let mut answer = HashSet::new();
        answer.insert(mask_to_integer(&mask));
        answer
    }
}

fn mask_to_integer(number: &Mask) -> u64 {
    let base: usize = 2;
    number
        .iter()
        .enumerate()
        .map(|(idx, val)| {
            let val = val.unwrap();
            if val {
                base.pow((35 - idx) as u32) as u64
            } else {
                0
            }
        })
        .sum()
}

fn binary_to_integer(number: &BinaryNumber) -> u64 {
    let base: usize = 2;
    number
        .iter()
        .enumerate()
        .map(|(idx, val)| {
            if *val {
                base.pow((35 - idx) as u32) as u64
            } else {
                0
            }
        })
        .sum()
}

fn integer_to_binary(mut input: u64) -> BinaryNumber {
    let mut output = [false; 36];
    let base: u64 = 2;
    for i in 0..36 {
        let modulus = base.pow(35 - i);
        let quo = input / modulus;
        output[i as usize] = quo == 1;
        input -= quo * modulus;
    }
    output.to_vec()
}

fn parse_input(input: &str) -> Vec<Instruction> {
    let mask_parser = parser!({
        "X" => None,
        "0" => Some(false),
        "1" => Some(true)
    }+);

    use Instruction::*;
    let line_parser = parser!({
        "mask = " mask:mask_parser => SetMask(mask),
        "mem[" address:u64 "] = " val:u64 => SetMemory(address, val)
    });

    let p = parser!(lines(line_parser));
    p.parse(input).unwrap()
}

impl ProblemSolution for Solution {
    fn solve_a(&self, input: &str) -> Option<String> {
        let instructions = parse_input(input);
        let mut memory = HashMap::new();
        let mut mask = vec![None; 36];
        for instruction in instructions {
            match instruction {
                Instruction::SetMask(mask_vec) => mask = mask_vec,
                Instruction::SetMemory(address, val) => {
                    let val_as_binary = integer_to_binary(val);
                    let masked = apply_mask(val_as_binary, &mask);
                    let masked_int = binary_to_integer(&masked);
                    memory.insert(address, masked_int);
                }
            }
        }

        let answer: u64 = memory.values().into_iter().sum();
        Some(answer.to_string())
    }

    fn solve_b(&self, input: &str) -> Option<String> {
        let instructions = parse_input(input);
        let mut memory = HashMap::new();
        let mut mask = vec![None; 36];
        for instruction in instructions {
            match instruction {
                Instruction::SetMask(mask_vec) => mask = mask_vec,
                Instruction::SetMemory(address, val) => {
                    let address_as_binary = integer_to_binary(address);
                    let mut address_set = apply_mask_to_address(address_as_binary, &mask);
                    for address in mask_to_all_addresses(&mut address_set) {
                        memory.insert(address, val);
                    }
                }
            }
        }

        let answer: u64 = memory.values().into_iter().sum();
        Some(answer.to_string())
    }
}
