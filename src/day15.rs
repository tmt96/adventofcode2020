use crate::solver::Solver;
use std::collections::HashMap;
use std::io;
use std::iter::{successors, Iterator};

pub struct Problem;

impl Problem {
    fn solve_problem(&self, input: &[i64], target: usize) -> i64 {
        let mut map = HashMap::new();
        for (ind, &num) in input[..input.len() - 1].iter().enumerate() {
            map.insert(num, ind);
        }
        let mut ind = input.len() - 1;
        successors(input.last().cloned(), |num| {
            let next_num = map.get(num).map_or(0, |v| ind - v);
            map.insert(*num, ind);
            ind += 1;
            Some(next_num as i64)
        })
        .nth(target - input.len())
        .unwrap()
    }
}

impl Solver for Problem {
    type Input = Vec<i64>;
    type Output1 = i64;
    type Output2 = i64;

    fn get_day(&self) -> i32 {
        15
    }

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        let mut buf = "".to_string();
        let mut r = r;
        r.read_to_string(&mut buf).unwrap();
        buf.split(',').map(|s| s.parse().unwrap()).collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        self.solve_problem(input, 2020)
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        self.solve_problem(input, 30000000)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() {
        let raw_input = "0,3,6";
        let problem = Problem {};
        let input = problem.parse_input(raw_input.as_bytes());
        assert_eq!(problem.solve_first(&input), 436);
    }

    #[test]
    fn test_second() {
        let raw_input = "0,3,6";
        let problem = Problem {};
        let input = problem.parse_input(raw_input.as_bytes());
        assert_eq!(problem.solve_second(&input), 175594);
    }
}
