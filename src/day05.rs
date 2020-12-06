use crate::solver::Solver;
use std::io::{self, BufRead, BufReader};

pub struct Seat {
    row: i32,
    col: i32,
}

impl Seat {
    fn from_string(s: &str) -> Self {
        Self {
            row: Self::binary_seek(&s[..7], 'F', 128),
            col: Self::binary_seek(&s[7..], 'L', 8),
        }
    }

    fn binary_seek(s: &str, lower_ch: char, upper_lim: i32) -> i32 {
        let (low, _) = s.chars().fold((0, upper_lim), |(l, h), ch| {
            if ch == lower_ch {
                (l, (l + h) / 2)
            } else {
                ((l + h) / 2, h)
            }
        });
        low
    }

    fn seat_id(&self) -> i32 {
        self.row * 8 + self.col
    }
}

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<Seat>;
    type Output1 = i32;
    type Output2 = i32;

    fn get_day(&self) -> i32 {
        5
    }

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        let r = BufReader::new(r);
        r.lines().flatten().map(|s| Seat::from_string(&s)).collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        input.iter().map(|seat| seat.seat_id()).max().unwrap()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        const SEATS: usize = 128 * 8;
        let mut seat_exists = [false; SEATS];
        for seat_id in input.iter().map(|seat| seat.seat_id()) {
            seat_exists[seat_id as usize] = true;
        }

        (0..SEATS)
            .find(|ind| {
                *ind >= 8
                    && *ind <= 127 * 8
                    && !seat_exists[*ind]
                    && seat_exists[ind - 1]
                    && seat_exists[ind + 1]
            })
            .unwrap() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seat_id() {
        assert_eq!(Seat::from_string("FBFBBFFRLR").seat_id(), 357);
        assert_eq!(Seat::from_string("BFFFBBFRRR").seat_id(), 567);
        assert_eq!(Seat::from_string("FFFBBBFRRR").seat_id(), 119);
        assert_eq!(Seat::from_string("BBFFBBFRLL").seat_id(), 820);
    }
}
