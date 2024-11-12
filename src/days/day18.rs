use crate::ProblemSolution;
#[allow(unused_imports)]
use aoc_parse::{parser, prelude::*};
pub struct Solution {}

#[derive(Debug)]
enum Expression {
    Sum(Box<Expression>, Box<Expression>),
    Product(Box<Expression>, Box<Expression>),
    Integer(usize),
}

#[derive(Debug)]
enum RawSymbol {
    Multiply,
    Add,
    Integer(usize),
    Bracketed(Vec<RawSymbol>),
}

// Look for all contiguous sums, and turn them into bracketed expressions
fn bracket_additions(symbols: &mut Vec<RawSymbol>) {
    // Find the position of the first add
    let first_add = symbols.iter().position(|sym| matches!(sym, RawSymbol::Add));
    // Nothing to do
    if first_add.is_none() {
        return;
    }
    let first_add = first_add.unwrap();

    // First add starts a streak of adds, when does that streak end?
    let mut first_add_streak_end = first_add;
    while first_add_streak_end + 2 < symbols.len()
        && matches!(symbols[first_add_streak_end + 2], RawSymbol::Add)
    {
        first_add_streak_end += 2;
    }

    // Pull out those terms and bracket them
    let to_bracket = symbols.drain((first_add - 1)..=(first_add_streak_end + 1));
    let bracketed = RawSymbol::Bracketed(to_bracket.collect());

    // Insert back in the correct position (the position one before the first add)
    symbols.insert(first_add - 1, bracketed);

    // Recurse
    bracket_additions(symbols);
}

fn all_operations_are_additions(symbols: &Vec<RawSymbol>) -> bool {
    symbols[1..]
        .iter()
        .step_by(2)
        .all(|sym| matches!(sym, RawSymbol::Add))
}

fn build_expression<const BRACKET_ADDITIONS: bool>(mut symbols: Vec<RawSymbol>) -> Expression {
    // Check for all operations being additions otherwise we would infinitely bracket!
    if BRACKET_ADDITIONS && !all_operations_are_additions(&symbols) {
        bracket_additions(&mut symbols);
    }

    if symbols.len() <= 1 {
        return match symbols.pop().expect("Cannot build an empty expression") {
            RawSymbol::Integer(n) => Expression::Integer(n),
            RawSymbol::Bracketed(bracketed_symbols) => {
                build_expression::<BRACKET_ADDITIONS>(bracketed_symbols)
            }
            _ => panic!("An operation without operands is not an expression"),
        };
    }

    // If we reach here then symbols was length at least 2

    let right_operand = symbols.pop().unwrap();
    let right_operand = match right_operand {
        RawSymbol::Integer(n) => Expression::Integer(n),
        RawSymbol::Bracketed(bracketed_symbols) => {
            build_expression::<BRACKET_ADDITIONS>(bracketed_symbols)
        }
        _ => panic!("Symbols ended with an operation that has no right operand"),
    };
    let right_operand = Box::new(right_operand);

    let operation = symbols.pop().unwrap();

    // LHS is fully evaluated first
    let left_operand = build_expression::<BRACKET_ADDITIONS>(symbols);
    let left_operand = Box::new(left_operand);

    match operation {
        RawSymbol::Multiply => Expression::Product(left_operand, right_operand),
        RawSymbol::Add => Expression::Sum(left_operand, right_operand),
        _ => panic!("Expressions must be separated by operations"),
    }
}

impl Expression {
    fn evaluate(&self) -> usize {
        match self {
            Expression::Sum(e_left, e_right) => e_left.evaluate() + e_right.evaluate(),
            Expression::Product(e_left, e_right) => e_left.evaluate() * e_right.evaluate(),
            Expression::Integer(num) => *num,
        }
    }
}

fn parse_input(input: &str) -> Vec<Vec<RawSymbol>> {
    let raw_expression_parser = parser!(
        rule raw_expression: RawSymbol = {
            "*" => RawSymbol::Multiply,
            "+" => RawSymbol::Add,
            n:usize => RawSymbol::Integer(n),
            br:bracketed => RawSymbol::Bracketed(br)
        };

        rule bracketed: Vec<RawSymbol>
            = '(' br:repeat_sep(raw_expression, " ") ')' => br;

        exps:repeat_sep(raw_expression, " ") => exps
    );

    let p = parser!(lines(raw_expression_parser));
    p.parse(input).unwrap()
}

impl ProblemSolution for Solution {
    fn solve_a(&self, input: &str) -> Option<String> {
        let raw_symbols = parse_input(input);
        let answer: usize = raw_symbols
            .into_iter()
            .map(|symbols| build_expression::<false>(symbols).evaluate())
            .sum();
        Some(answer.to_string())
    }

    fn solve_b(&self, input: &str) -> Option<String> {
        let raw_symbols = parse_input(input);
        let answer: usize = raw_symbols
            .into_iter()
            .map(|symbols| build_expression::<true>(symbols).evaluate())
            .sum();
        Some(answer.to_string())
    }
}
