use crate::solver::Solver;
use std::collections::HashSet;
use std::io::{self, BufRead, BufReader};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Inst {
    NOP(i32),
    ACC(i32),
    JMP(i32),
}

impl Inst {
    fn from_string(inst: &str) -> Self {
        let parse_inst: Vec<_> = inst.split(' ').collect();
        let num: i32 = parse_inst[1].parse().unwrap();
        match parse_inst[0] {
            "nop" => Self::NOP(num),
            "acc" => Self::ACC(num),
            "jmp" => Self::JMP(num),
            _ => panic!("Invalid instruction!"),
        }
    }

    fn execute(&self, acc: i32, ind: i32) -> (i32, i32) {
        match self {
            Self::NOP(_) => (acc, ind + 1),
            Self::ACC(i) => (acc + i, ind + 1),
            Self::JMP(i) => (acc, ind + i),
        }
    }
}

struct ProgramState {
    success: bool,
    #[allow(dead_code)]
    ind: usize,
    acc: i32,
    indices: HashSet<usize>,
}

pub struct Problem;

impl Problem {
    fn run_program(&self, input: &[Inst]) -> ProgramState {
        let mut indices = HashSet::new();
        let (mut acc, mut ind) = (0, 0);

        loop {
            indices.insert(ind);
            let new_result = input[ind].execute(acc, ind as i32);
            acc = new_result.0;
            ind = new_result.1 as usize;

            if indices.contains(&ind) || ind >= input.len() {
                return ProgramState {
                    acc,
                    ind,
                    indices,
                    success: ind >= input.len(),
                };
            }
        }
    }
}

impl Solver for Problem {
    type Input = Vec<Inst>;
    type Output1 = i32;
    type Output2 = i32;

    fn get_day(&self) -> i32 {
        8
    }

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        let r = BufReader::new(r);
        r.lines().flatten().map(|s| Inst::from_string(&s)).collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        let ProgramState { acc, .. } = self.run_program(input);
        acc
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        let ProgramState { indices, .. } = self.run_program(input);

        for index in indices {
            let mut new_input = input.clone();
            let new_inst = match new_input[index] {
                Inst::NOP(i) => Inst::JMP(i),
                Inst::JMP(i) => Inst::NOP(i),
                _ => continue,
            };
            new_input[index] = new_inst;

            let ProgramState { success, acc, .. } = self.run_program(&new_input);
            if success {
                return acc;
            }
        }

        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() {
        let raw_input = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6
";
        let problem = Problem {};
        let input = problem.parse_input(raw_input.as_bytes());
        assert_eq!(problem.solve_first(&input), 5);
    }

    #[test]
    fn test_second() {
        let raw_input = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6
";
        let problem = Problem {};
        let input = problem.parse_input(raw_input.as_bytes());
        assert_eq!(problem.solve_second(&input), 8);
    }
}
