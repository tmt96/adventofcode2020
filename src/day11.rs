use crate::solver::Solver;
use std::io::{self, BufRead, BufReader};

pub struct Problem;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum State {
    Floor,
    Empty,
    Occupied,
}

impl State {
    fn from_char(ch: char) -> Self {
        match ch {
            '.' => Self::Floor,
            '#' => Self::Empty,
            'L' => Self::Occupied,
            _ => panic!("Unrecognizable char"),
        }
    }
}

type Pos = (usize, usize);

#[derive(Clone, PartialEq, Eq)]
pub struct Model {
    board: Vec<Vec<State>>,
}

impl Model {
    fn new(board: Vec<Vec<State>>) -> Self {
        Self { board }
    }

    fn width(&self) -> usize {
        self.board[0].len()
    }

    fn height(&self) -> usize {
        self.board.len()
    }

    fn get_signed(&self, pos: (isize, isize)) -> Option<State> {
        let (x, y) = pos;
        if x < 0 || y < 0 || x >= self.height() as isize || y >= self.width() as isize {
            None
        } else {
            self.board
                .get(x as usize)
                .and_then(|row| row.get(y as usize))
                .copied()
        }
    }

    fn get(&self, pos: Pos) -> Option<State> {
        self.get_signed((pos.0 as isize, pos.1 as isize))
    }

    fn get_adjacent(&self, pos: Pos) -> impl Iterator<Item = State> + '_ {
        let (x, y) = (pos.0 as isize, pos.1 as isize);

        (-1..=1).flat_map(move |i| {
            (-1..=1).filter_map(move |j| {
                if i == 0 && j == 0 {
                    None
                } else {
                    self.get_signed((x + i, y + j))
                }
            })
        })
    }

    fn occupied_adjacent_count(&self, pos: Pos) -> usize {
        self.get_adjacent(pos)
            .filter(|&state| state == State::Occupied)
            .count()
    }

    fn new_state(&self, pos: Pos) -> State {
        let state = self.get(pos).unwrap();
        if state == State::Empty && self.occupied_adjacent_count(pos) == 0 {
            State::Occupied
        } else if state == State::Occupied && self.occupied_adjacent_count(pos) >= 4 {
            State::Empty
        } else {
            state
        }
    }

    fn perform_one_round(&self) -> Self {
        let board = (0..self.height())
            .map(|x| (0..self.width()).map(|y| self.new_state((x, y))).collect())
            .collect();
        Self::new(board)
    }

    fn occupied_seats(&self) -> usize {
        self.board
            .iter()
            .map(|row| {
                row.iter()
                    .filter(|&state| state == &State::Occupied)
                    .count()
            })
            .sum()
    }
}

impl Problem {}

impl Solver for Problem {
    type Input = Model;
    type Output1 = usize;
    type Output2 = i32;

    fn get_day(&self) -> i32 {
        11
    }

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        let r = BufReader::new(r);
        let board = r
            .lines()
            .flatten()
            .map(|l| l.chars().map(State::from_char).collect())
            .collect();
        Model::new(board)
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        let mut board = input.clone();
        let mut new_board = board.perform_one_round();
        while new_board != board {
            board = new_board.clone();
            new_board = board.perform_one_round();
        }
        new_board.occupied_seats()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() {
        let raw_input = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL
";
        let problem = Problem {};
        let input = problem.parse_input(raw_input.as_bytes());
        assert_eq!(problem.solve_first(&input), 37);
    }

    #[test]
    fn test_second() {
        let raw_input = "";
        let problem = Problem {};
        let input = problem.parse_input(raw_input.as_bytes());
        assert_eq!(problem.solve_second(&input), 0);
    }
}
