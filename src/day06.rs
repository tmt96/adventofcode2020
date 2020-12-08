use crate::solver::Solver;
use std::collections::HashSet;
use std::io;

type Person = String;
type Group = Vec<Person>;

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<Group>;
    type Output1 = usize;
    type Output2 = usize;

    fn get_day(&self) -> i32 {
        6
    }

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        let mut buf = String::new();
        let mut r = r;
        let _ = r.read_to_string(&mut buf);
        buf.split("\n\n")
            .map(|group| group.lines().map(|s| s.to_string()).collect())
            .collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        let question_per_group = |group: &Group| {
            group
                .iter()
                .fold(HashSet::new(), |set, s| {
                    let string_chars: HashSet<_> = s.chars().collect();
                    set.union(&string_chars).cloned().collect()
                })
                .len()
        };

        input.iter().map(question_per_group).sum()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        let question_per_group = |group: &Group| {
            group
                .iter()
                .fold(group[0].chars().collect::<HashSet<_>>(), |set, s| {
                    let string_chars: HashSet<_> = s.chars().collect();
                    set.intersection(&string_chars).cloned().collect()
                })
                .len()
        };

        input.iter().map(question_per_group).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() {
        let raw_input = r"abc

a
b
c

ab
ac

a
a
a
a

b
";
        let prob = Problem {};
        let input = prob.parse_input(raw_input.as_bytes());
        assert_eq!(prob.solve_first(&input), 11);
    }

    #[test]
    fn test_second() {
        let raw_input = r"abc

a
b
c

ab
ac

a
a
a
a

b
";
        let prob = Problem {};
        let input = prob.parse_input(raw_input.as_bytes());
        assert_eq!(prob.solve_second(&input), 6);
    }
}
