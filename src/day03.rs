use crate::solver::Solver;
use std::collections::HashSet;
use std::io::{self, BufRead, BufReader};

type Position = (usize, usize);

pub struct Grid {
    height: usize,
    width: usize,
    tree_list: HashSet<Position>,
}

impl Grid {
    fn step(&self, cur_pos: Position, step: Position) -> Position {
        ((cur_pos.0 + step.0) % self.width, cur_pos.1 + step.1)
    }

    fn is_tree(&self, pos: Position) -> bool {
        self.tree_list.contains(&pos)
    }
}

pub struct Problem;

impl Problem {
    fn traverse_slope(&self, input: &Grid, step: Position) -> i64 {
        let mut pos = (0, 0);
        let mut result = 0;
        while pos.1 < input.height {
            pos = input.step(pos, step);
            if input.is_tree(pos) {
                result += 1;
            }
        }

        result
    }
}

impl Solver for Problem {
    type Input = Grid;
    type Output1 = i64;
    type Output2 = i64;

    fn get_day(&self) -> i32 {
        3
    }

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        let mut tree_list = HashSet::new();
        let (mut height, mut width) = (0, 0);
        let r = BufReader::new(r);

        for (row, line) in r.lines().flatten().enumerate() {
            if width == 0 {
                width = line.len();
            }
            height += 1;
            for (col, ch) in line.chars().enumerate() {
                if ch == '#' {
                    tree_list.insert((col, row));
                }
            }
        }

        Grid {
            width,
            height,
            tree_list,
        }
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        let step = (3, 1);
        self.traverse_slope(input, step)
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        let steps = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
        steps
            .iter()
            .map(|step| self.traverse_slope(input, *step))
            .product()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() {
        let input = r"..##.........##.........##.........##.........##.........##.......
#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..
.#....#..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.
..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#
.#...##..#..#...##..#..#...##..#..#...##..#..#...##..#..#...##..#.
..#.##.......#.##.......#.##.......#.##.......#.##.......#.##.....
.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#
.#........#.#........#.#........#.#........#.#........#.#........#
#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...
#...##....##...##....##...##....##...##....##...##....##...##....#
.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.# ";
        let problem = Problem {};
        let grid = problem.parse_input(input.as_bytes());
        assert_eq!(problem.solve_first(&grid), 7);
    }

    #[test]
    fn test_second() {
        let input = r"..##.........##.........##.........##.........##.........##.......
#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..
.#....#..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.
..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#
.#...##..#..#...##..#..#...##..#..#...##..#..#...##..#..#...##..#.
..#.##.......#.##.......#.##.......#.##.......#.##.......#.##.....
.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#
.#........#.#........#.#........#.#........#.#........#.#........#
#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...
#...##....##...##....##...##....##...##....##...##....##...##....#
.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.# ";
        let problem = Problem {};
        let grid = problem.parse_input(input.as_bytes());
        assert_eq!(problem.solve_second(&grid), 336);
    }
}
