use crate::solver::Solver;
use std::collections::HashSet;
use std::io::{self, BufRead, BufReader};

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<i32>;
    type Output1 = i32;
    type Output2 = i32;

    fn get_day(&self) -> i32 {
        1
    }

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        let r = BufReader::new(r);
        r.lines().flatten().flat_map(|s| s.parse::<i32>()).collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        let mut all_numbers = HashSet::<i32>::new();
        for i in input {
            if all_numbers.contains(&(2020 - i)) {
                return i * (2020 - i);
            }
            all_numbers.insert(*i);
        }
        0
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        for (fst_index, i) in input.iter().enumerate() {
            let mut all_numbers = HashSet::<i32>::new();
            for (snd_index, j) in input.iter().enumerate() {
                if snd_index == fst_index {
                    continue;
                }
                if all_numbers.contains(&(2020 - i - j)) {
                    return i * j * (2020 - i - j);
                }
                all_numbers.insert(*j);
            }
        }
        0
    }
}
