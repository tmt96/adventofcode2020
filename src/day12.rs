use crate::solver::Solver;
use std::convert::{From, Into};
use std::io::{self, BufRead, BufReader};
use std::ops::{AddAssign, SubAssign};

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Direction {
    E = 0,
    S,
    W,
    N,
}

impl From<i64> for Direction {
    fn from(i: i64) -> Self {
        match i {
            0 => Self::E,
            1 => Self::S,
            2 => Self::W,
            3 => Self::N,
            _ => panic!("unexpected direction"),
        }
    }
}

impl AddAssign<i64> for Direction {
    fn add_assign(&mut self, i: i64) {
        *self = ((*self as i64 + i / 90).rem_euclid(4)).into()
    }
}

impl SubAssign<i64> for Direction {
    fn sub_assign(&mut self, i: i64) {
        *self = ((*self as i64 - i / 90).rem_euclid(4)).into()
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Command {
    command: char,
    num: i64,
}

impl From<&str> for Command {
    fn from(s: &str) -> Self {
        let num: i64 = s.get(1..).unwrap().parse().unwrap();
        let command = s.chars().next().unwrap();
        Self { command, num }
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
struct Ship {
    x: i64,
    y: i64,
    dir: Direction,
    waypoint_x: i64,
    waypoint_y: i64,
}

impl Ship {
    fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            dir: Direction::E,
            waypoint_x: 10,
            waypoint_y: 1,
        }
    }

    fn perform_command_pt1(&mut self, command: Command) {
        let Command { command, num } = command;
        match command {
            'N' => self.y += num,
            'E' => self.x += num,
            'S' => self.y -= num,
            'W' => self.x -= num,
            'L' => self.dir -= num,
            'R' => self.dir += num,
            'F' => match self.dir {
                Direction::N => self.y += num,
                Direction::E => self.x += num,
                Direction::S => self.y -= num,
                Direction::W => self.x -= num,
            },
            _ => panic!("Unexpected command"),
        }
    }

    fn perform_command_pt2(&mut self, command: Command) {
        let Command { command, num } = command;
        match command {
            'N' => self.waypoint_y += num,
            'E' => self.waypoint_x += num,
            'S' => self.waypoint_y -= num,
            'W' => self.waypoint_x -= num,
            'L' => {
                let (temp_x, temp_y) =
                    (0..num / 90).fold((self.waypoint_x, self.waypoint_y), |(x, y), _| (-y, x));
                self.waypoint_x = temp_x;
                self.waypoint_y = temp_y;
            }
            'R' => {
                let (temp_x, temp_y) =
                    (0..num / 90).fold((self.waypoint_x, self.waypoint_y), |(x, y), _| (y, -x));
                self.waypoint_x = temp_x;
                self.waypoint_y = temp_y;
            }
            'F' => {
                self.x += self.waypoint_x * num;
                self.y += self.waypoint_y * num;
            }
            _ => panic!("Unexpected command"),
        }
    }
}

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<Command>;
    type Output1 = i64;
    type Output2 = i64;

    fn get_day(&self) -> i32 {
        12
    }

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        let r = BufReader::new(r);
        r.lines()
            .flatten()
            .map(|l| Command::from(l.as_ref()))
            .collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        let mut ship = Ship::new();
        for command in input.iter() {
            ship.perform_command_pt1(*command);
        }

        ship.x.abs() + ship.y.abs()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        let mut ship = Ship::new();
        for command in input.iter() {
            ship.perform_command_pt2(*command);
        }

        ship.x.abs() + ship.y.abs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() {
        let raw_input = "F10
N3
F7
R90
F11
";
        let problem = Problem {};
        let input = problem.parse_input(raw_input.as_bytes());
        assert_eq!(problem.solve_first(&input), 25);
    }

    #[test]
    fn test_second() {
        let raw_input = "F10
N3
F7
R90
F11
";
        let problem = Problem {};
        let input = problem.parse_input(raw_input.as_bytes());
        assert_eq!(problem.solve_second(&input), 286);
    }
}
