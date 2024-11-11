use crate::ProblemSolution;
#[allow(unused_imports)]
use aoc_parse::{parser, prelude::*};
pub struct Solution {}

enum Service {
    OutOfService,
    Frequency(usize),
}

fn crt_iso_base_case(x1: ModuloNumber, x2: ModuloNumber) -> ModuloNumber {
    let mut x = x1.number;
    loop {
        if x.rem_euclid(x2.modulus) == x2.number {
            return ModuloNumber::init(x, x1.modulus * x2.modulus);
        } else {
            x += x1.modulus;
        }
    }
}

// Implements the isomorphism from the Chinese Remainder Theorem
fn crt_iso(mut inputs: Vec<ModuloNumber>) -> ModuloNumber {
    let x1 = inputs.pop().unwrap();
    let x2 = inputs.pop().unwrap();
    let x12 = crt_iso_base_case(x1, x2);
    if inputs.is_empty() {
        x12
    } else {
        inputs.push(x12);
        crt_iso(inputs)
    }
}

// Should ensure 0 <= number < modulo
// Represents a number in Z/nZ where n==modulo
struct ModuloNumber {
    number: usize,
    modulus: usize,
}

impl ModuloNumber {
    fn init(number: usize, modulus: usize) -> Self {
        Self { number, modulus }
    }
}

fn parse_input(input: &str) -> (usize, Vec<Service>) {
    let p = parser!(
        line(usize)
        line(repeat_sep({
            "x" => Service::OutOfService,
            freq:usize => Service::Frequency(freq)
        }, ","))
    );
    p.parse(input).unwrap()
}

impl ProblemSolution for Solution {
    fn solve_a(&self, input: &str) -> Option<String> {
        let (arrival, services) = parse_input(input);
        let frequencies = services.iter().filter_map(|s| match s {
            Service::OutOfService => None,
            Service::Frequency(freq) => Some(freq),
        });
        let answer = frequencies
            .map(|freq| (freq, (*freq - arrival.rem_euclid(*freq))))
            .min_by_key(|(_freq, wait)| *wait)
            .unwrap();

        let answer = answer.0 * answer.1;
        Some(answer.to_string())
    }

    fn solve_b(&self, input: &str) -> Option<String> {
        let (_arrival, services) = parse_input(input);
        let inputs: Vec<_> = services
            .iter()
            .enumerate()
            .filter_map(|(idx, s)| match s {
                Service::OutOfService => None,
                Service::Frequency(freq) => {
                    let offset = -(idx as isize);
                    let offset = offset.rem_euclid(*freq as isize) as usize;
                    let number = ModuloNumber::init(offset, *freq);
                    Some(number)
                }
            })
            .collect();

        let answer = crt_iso(inputs).number;
        Some(answer.to_string())
    }
}
