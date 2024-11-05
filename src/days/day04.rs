use std::collections::HashMap;

use crate::ProblemSolution;
#[allow(unused_imports)]
use aoc_parse::{parser, prelude::*};
pub struct Solution {}

struct Passport(HashMap<String, String>);

enum Units {
    Cm,
    In,
}

impl Passport {
    fn all_present(&self) -> bool {
        ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
            .iter()
            .all(|key| self.0.contains_key(*key))
    }

    fn all_valid(&self) -> bool {
        let byr = self.0.get("byr").unwrap();
        if byr.len() != 4 {
            return false;
        }
        let byr: usize = byr.parse().unwrap();
        if byr < 1920 || byr > 2002 {
            return false;
        }

        let iyr = self.0.get("iyr").unwrap();
        if iyr.len() != 4 {
            return false;
        }
        let iyr: usize = iyr.parse().unwrap();
        if iyr < 2010 || iyr > 2020 {
            return false;
        }

        let eyr = self.0.get("eyr").unwrap();
        if eyr.len() != 4 {
            return false;
        }
        let eyr: usize = eyr.parse().unwrap();
        if eyr < 2020 || eyr > 2030 {
            return false;
        }

        let hgt_parser = parser!(usize {"cm" => Units::Cm, "in" => Units::In});
        let hgt = self.0.get("hgt").unwrap();
        match hgt_parser.parse(&hgt) {
            Err(_) => return false,
            Ok((height, Units::Cm)) => {
                if height < 150 || height > 193 {
                    return false;
                }
            }
            Ok((height, Units::In)) => {
                if height < 59 || height > 76 {
                    return false;
                }
            }
        }

        let hcl_char_parser = parser!(char_of("0123456789abcdef"));
        let hcl_parser = parser!("#"
           hcl_char_parser
           hcl_char_parser
           hcl_char_parser
           hcl_char_parser
           hcl_char_parser
           hcl_char_parser
        );
        let hcl = self.0.get("hcl").unwrap();
        if hcl_parser.parse(&hcl).is_err() {
            return false;
        }

        let ecl_parser = parser!({"amb", "blu", "brn", "gry", "grn", "hzl", "oth"});
        let ecl = self.0.get("ecl").unwrap();
        if ecl_parser.parse(&ecl).is_err() {
            return false;
        }

        let pid = self.0.get("pid").unwrap();
        let pid_parser = parser!(digit+);
        if pid_parser.parse(&pid).is_err() {
            return false;
        }
        if pid.len() != 9 {
            return false;
        }

        true
    }
}

fn parse_passports(input: &str) -> impl Iterator<Item = Passport> {
    let key_or_value_parser = parser!(string({alnum, "#" => '#'}+));
    let entry_parser = parser!(key_or_value_parser ":" key_or_value_parser);
    let passport_parser = parser!(
        passport_lines:lines(repeat_sep(entry_parser, " "))
        =>
        Passport(passport_lines.into_iter().flatten().collect())
    );
    let p = parser!(sections(passport_parser));
    p.parse(input).unwrap().into_iter()
}

impl ProblemSolution for Solution {
    fn solve_a(&self, input: &str) -> Option<String> {
        let passports = parse_passports(input);
        let n_valid = passports.filter(|p| p.all_present()).count();
        Some(n_valid.to_string())
    }

    fn solve_b(&self, input: &str) -> Option<String> {
        let passports = parse_passports(input);
        let n_valid = passports
            .filter(|p| p.all_present())
            .filter(|p| p.all_valid())
            .count();
        Some(n_valid.to_string())
    }
}
