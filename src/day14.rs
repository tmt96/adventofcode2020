use crate::solver::Solver;
use regex::Regex;
use std::collections::HashMap;
use std::io::{self, BufRead, BufReader};
use std::iter::once_with;

pub enum Command {
    Mask(String),
    Write { addr: i64, val: i64 },
}

impl Command {
    fn from_string(s: &str) -> Self {
        let mask_regex = Regex::new(r"mask = ([X10]+)").unwrap();
        let mem_regex = Regex::new(r"mem\[(?P<addr>\d+)\] = (?P<val>\d+)").unwrap();
        if let Some(capture) = mask_regex.captures(s) {
            Self::Mask(capture[1].to_string())
        } else if let Some(capture) = mem_regex.captures(s) {
            Self::Write {
                addr: capture["addr"].parse().unwrap(),
                val: capture["val"].parse().unwrap(),
            }
        } else {
            panic!("Unexpected command!");
        }
    }
}

fn change_nth_char(s: &str, n: usize, ch: u8) -> String {
    let mut bytes = s.as_bytes().to_owned();
    bytes[n] = ch;
    String::from_utf8(bytes).unwrap()
}

pub struct Problem;

impl Problem {
    fn get_masks_pt1(mask: &str) -> (i64, i64) {
        let and_mask = i64::from_str_radix(&mask.replace('X', "1"), 2).unwrap();
        let or_mask = i64::from_str_radix(&mask.replace('X', "0"), 2).unwrap();
        (and_mask, or_mask)
    }

    fn mask_pt1(num: i64, mask: &str) -> i64 {
        let (and_mask, or_mask) = Self::get_masks_pt1(mask);
        (num & and_mask) | or_mask
    }

    fn get_masks_pt2(mask: &str) -> Box<dyn Iterator<Item = i64>> {
        match mask.find('X') {
            Some(i) => {
                let mask_1 = change_nth_char(mask, i, b'1');
                let mask_2 = change_nth_char(mask, i, b'0');
                let iter = Self::get_masks_pt2(&mask_1);
                Box::new(iter.chain(Self::get_masks_pt2(&mask_2)))
            }
            None => Box::new(once_with(move || i64::from_str_radix(mask, 2).unwrap())),
        }
    }

    fn mask_pt2(num: i64, mask: &str) -> Box<dyn Iterator<Item = i64>> {
        let masks = Self::get_masks_pt2(mask.to_string()).map(move |m| num & m);
        Box::new(masks)
    }
}

impl Solver for Problem {
    type Input = Vec<Command>;
    type Output1 = i64;
    type Output2 = i64;

    fn get_day(&self) -> i32 {
        14
    }

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        let r = BufReader::new(r);
        r.lines()
            .flatten()
            .map(|s| Command::from_string(&s))
            .collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        let mut mask = "";
        let mut address_space = HashMap::new();
        for command in input.iter() {
            match command {
                Command::Mask(s) => mask = s,
                Command::Write { addr, val } => {
                    address_space.insert(addr, Self::mask_pt1(*val, mask));
                }
            }
        }

        address_space.values().sum()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        let mut mask = "";
        let mut address_space = [0; 1 << 36];
        for command in input.iter() {
            match command {
                Command::Mask(s) => mask = s,
                Command::Write { addr, val } => {
                    let addr_list = Self::mask_pt2(*addr, mask);
                    for new_addr in addr_list {
                        address_space[new_addr as usize] = *val;
                    }
                }
            }
        }

        address_space.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() {
        let raw_input = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0
";
        let problem = Problem {};
        let input = problem.parse_input(raw_input.as_bytes());
        assert_eq!(problem.solve_first(&input), 165);
    }

    #[test]
    fn test_second() {
        let raw_input = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0
";
        let problem = Problem {};
        let input = problem.parse_input(raw_input.as_bytes());
        assert_eq!(problem.solve_second(&input), 208);
    }
}
