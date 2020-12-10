use crate::solver::Solver;
use std::io::{self, BufRead, BufReader};

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<i32>;
    type Output1 = usize;
    type Output2 = i64;

    fn get_day(&self) -> i32 {
        10
    }

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        let r = BufReader::new(r);
        let mut input: Vec<i32> = r.lines().flatten().flat_map(|s| s.parse()).collect();
        input.push(0);
        input.sort();
        input
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        let one_jolt_diffs = input
            .windows(2)
            .filter(|window| window[1] - window[0] == 1)
            .count();

        let three_jolt_diffs = input
            .windows(2)
            .filter(|window| window[1] - window[0] == 3)
            .count();

        one_jolt_diffs * (three_jolt_diffs + 1)
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        let n = input.len();
        let mut result: Vec<i64> = vec![1; n];

        for i in 1..n {
            if input[i] - input[i - 1] < 4 {
                result[i] = result[i - 1];
            }
            if i > 1 && input[i] - input[i - 2] < 4 {
                result[i] += result[i - 2];
            }
            if i > 2 && input[i] - input[i - 3] < 4 {
                result[i] += result[i - 3];
            }
        }

        result[n - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() {
        let raw_input = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";
        let problem = Problem {};
        let input = problem.parse_input(raw_input.as_bytes());
        assert_eq!(problem.solve_first(&input), 220);
    }

    #[test]
    fn test_second() {
        let raw_input = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";
        let problem = Problem {};
        let input = problem.parse_input(raw_input.as_bytes());
        assert_eq!(problem.solve_second(&input), 19208);
    }
}
