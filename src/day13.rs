use crate::solver::Solver;
use regex::Regex;
use std::io::{self, BufRead, BufReader};

pub struct Input {
    timestamp: i64,
    routes: Vec<i64>,
}

pub struct Problem;

impl Solver for Problem {
    type Input = Input;
    type Output1 = i64;
    type Output2 = i64;

    fn get_day(&self) -> i32 {
        13
    }

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        let r = BufReader::new(r);
        let mut lines = r.lines().flatten();
        let timestamp: i64 = lines.next().unwrap().trim().parse().unwrap();

        let regex = Regex::new(r"(\d+)").unwrap();
        let routes: Vec<_> = regex
            .captures_iter(&lines.next().unwrap())
            .flat_map(|capture| capture[1].parse::<i64>())
            .collect();
        dbg!(&routes);
        Input { routes, timestamp }
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        let Input { timestamp, routes } = input;

        let (route, wait_time) = routes
            .iter()
            .map(|&route| (route, (-timestamp).rem_euclid(route)))
            .min_by_key(|&(_, wait_time)| wait_time)
            .unwrap();

        route * wait_time
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        // TODO: Use Chinese Remainer Theorem: https://rosettacode.org/wiki/Chinese_remainder_theorem#Rust
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() {
        let raw_input = "939
7,13,x,x,59,x,31,19
";
        let problem = Problem {};
        let input = problem.parse_input(raw_input.as_bytes());
        assert_eq!(problem.solve_first(&input), 295);
    }

    #[test]
    fn test_second() {
        let raw_input = "939
7,13,x,x,59,x,31,19
";
        let problem = Problem {};
        let input = problem.parse_input(raw_input.as_bytes());
        assert_eq!(problem.solve_second(&input), 0);
    }
}
