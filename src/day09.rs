use crate::solver::Solver;
use std::io::{self, BufRead, BufReader};

pub struct Problem;

impl Problem {
    fn first_invalid(&self, input: &[i64], n: usize) -> i64 {
        let (_, num) = input
            .iter()
            .enumerate()
            .find(|&(ind, num)| {
                let slice = &input[ind - n..ind];
                ind >= n && !slice.iter().any(|i| slice.contains(&(num - i)))
            })
            .unwrap();
        *num
    }

    fn encryption_weakness(&self, input: &[i64], n: usize) -> i64 {
        let target_sum = self.first_invalid(input, n);
        let (mut low_ind, mut high_ind, mut sum) = (0, 0, 0);

        loop {
            #[allow(clippy::clippy::comparison_chain)]
            if sum < target_sum {
                high_ind += 1;
                sum += input[high_ind];
            } else if sum > target_sum {
                low_ind += 1;
                sum -= input[low_ind];
            } else {
                let slice = &input[low_ind..=high_ind];
                return slice.iter().max().unwrap() + slice.iter().min().unwrap();
            }
        }
    }
}

impl Solver for Problem {
    type Input = Vec<i64>;
    type Output1 = i64;
    type Output2 = i64;

    fn get_day(&self) -> i32 {
        9
    }

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        let r = BufReader::new(r);
        r.lines().flatten().flat_map(|s| s.parse()).collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        self.first_invalid(input, 25)
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        self.encryption_weakness(input, 25)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() {
        let raw_input = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";
        let problem = Problem {};
        let input = problem.parse_input(raw_input.as_bytes());
        assert_eq!(problem.first_invalid(&input, 5), 127);
    }

    #[test]
    fn test_second() {
        let raw_input = "5
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";
        let problem = Problem {};
        let input = problem.parse_input(raw_input.as_bytes());
        assert_eq!(problem.encryption_weakness(&input, 5), 62);
    }
}
