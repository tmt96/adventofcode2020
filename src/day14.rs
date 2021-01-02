use crate::solver::Solver;
use regex::Regex;
use std::collections::HashMap;
use std::io::{self, BufRead, BufReader};
use std::str;

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

    fn apply_mask_pt2(s: &[u8], indices: &[usize]) -> Vec<i64> {
        if indices.is_empty() {
            let s = s.to_owned();
            vec![i64::from_str_radix(str::from_utf8(&s).unwrap(), 2).unwrap()]
        } else {
            let first_index = indices[0];
            [b'1', b'0']
                .iter()
                .flat_map(|ch| {
                    let mut m = s.to_owned();
                    m[first_index] = *ch;
                    Self::apply_mask_pt2(&m, &indices[1..])
                })
                .collect()
        }
    }

    fn mask_pt2(num: i64, mask: &str) -> Vec<i64> {
        let num = format!("{:036b}", num);
        let num = num.as_bytes();
        let mask = mask.as_bytes();

        let masked_num: Vec<_> = num
            .iter()
            .zip(mask.iter())
            .map(|(&n, &m)| match m {
                b'0' => n,
                m => m,
            })
            .collect();
        let indices: Vec<_> = (0..36).filter(|&i| masked_num[i] == b'X').collect();
        Self::apply_mask_pt2(&masked_num, &indices)
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
        let mut address_space = HashMap::new();
        for command in input.iter() {
            match command {
                Command::Mask(s) => mask = s,
                Command::Write { addr, val } => {
                    let addr_list = Self::mask_pt2(*addr, mask);
                    for new_addr in addr_list {
                        address_space.insert(new_addr, *val);
                    }
                }
            }
        }

        address_space.values().sum()
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
        let raw_input = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1
";
        let problem = Problem {};
        let input = problem.parse_input(raw_input.as_bytes());
        assert_eq!(problem.solve_second(&input), 208);
    }
}
