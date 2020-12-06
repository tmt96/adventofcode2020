use crate::solver::Solver;
use regex::Regex;
use std::io::{self, BufRead, BufReader};

pub struct PasswordEntry {
    letter: char,
    low_pos: usize,
    high_pos: usize,
    password: String,
}

impl PasswordEntry {
    fn from_string(passwd: &str) -> Self {
        let re = Regex::new(r"(?P<low>\d+)-(?P<high>\d+) (?P<letter>[a-z]): (?P<password>[a-z]+)")
            .unwrap();
        let caps = re.captures(passwd).unwrap();
        Self {
            letter: caps["letter"].chars().next().unwrap(),
            low_pos: caps["low"].parse().unwrap(),
            high_pos: caps["high"].parse().unwrap(),
            password: caps["password"].to_string(),
        }
    }

    fn is_valid_pt1(&self) -> bool {
        let letter_count = self
            .password
            .chars()
            .filter(|letter| letter == &self.letter)
            .count();
        letter_count >= self.low_pos && letter_count <= self.high_pos
    }

    fn is_valid_pt2(&self) -> bool {
        let letters: Vec<_> = self.password.chars().collect();
        let low_char = letters[self.low_pos - 1];
        let high_char = letters[self.high_pos - 1];
        (self.letter == low_char) ^ (self.letter == high_char)
    }
}

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<PasswordEntry>;
    type Output1 = usize;
    type Output2 = usize;

    fn get_day(&self) -> i32 {
        2
    }

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        let r = BufReader::new(r);
        r.lines()
            .flatten()
            .map(|s| PasswordEntry::from_string(&s))
            .collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        input.iter().filter(|entry| entry.is_valid_pt1()).count()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        input.iter().filter(|entry| entry.is_valid_pt2()).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() {
        let input = r"1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc
";
        let entries = input.lines().map(PasswordEntry::from_string).collect();
        assert_eq!(Problem {}.solve_first(&entries), 2);
    }

    #[test]
    fn test_second() {
        let input = r"1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc
";
        let entries = input.lines().map(PasswordEntry::from_string).collect();
        assert_eq!(Problem {}.solve_second(&entries), 1);
    }
}
