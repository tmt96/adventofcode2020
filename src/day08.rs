use crate::solver::Solver;
use std::io::{self, BufRead, BufReader};

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<i32>;
    type Output1 = i32;
    type Output2 = i32;

    fn get_day(&self) -> i32 {
        0
    }

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        let r = BufReader::new(r);
        vec![0]
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        0
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() {}

    #[test]
    fn test_second() {}
}
