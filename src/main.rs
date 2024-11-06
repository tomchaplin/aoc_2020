mod days;
mod io;
mod problem;

use colored::Colorize;
use io::{AocRunError, RunCode};
use problem::ProblemSolution;
use std::{env, str::FromStr};

fn print_solution(solution: &Result<RunCode<true>, AocRunError>) {
    match solution.as_ref() {
        Ok(sol) => print!("{}", sol),
        Err(e) => println!("{}", e),
    };
}

pub fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() > 2 {
        println!(
            "{}",
            "Please provide at most 1 argument".to_string().bold().red()
        );
        return;
    }

    if args.len() == 1 {
        for i in 1..=25 {
            println!("Day {}", i);
            let run_code = RunCode::init_run_all(i);
            let solution = run_code.run();
            print_solution(&solution);
            if solution.is_err() {
                break;
            }
        }
    } else {
        let run_code = RunCode::from_str(&args[1]);
        let solution = run_code.and_then(|rc| rc.run());
        print_solution(&solution);
    }
}
