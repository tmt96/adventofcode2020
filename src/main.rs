mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod solver;

use crate::solver::Solver;

fn solve_day(day: i32) {
    match day {
        // TODO: Write macro to automate solve by day
        1 => day01::Problem {}.solve(),
        2 => day02::Problem {}.solve(),
        3 => day03::Problem {}.solve(),
        4 => day04::Problem {}.solve(),
        5 => day05::Problem {}.solve(),
        6 => day06::Problem {}.solve(),
        7 => day07::Problem {}.solve(),
        8 => day08::Problem {}.solve(),
        9 => day09::Problem {}.solve(),
        10 => day10::Problem {}.solve(),
        11 => day11::Problem {}.solve(),
        12 => day12::Problem {}.solve(),
        13 => day13::Problem {}.solve(),
        14 => day14::Problem {}.solve(),
        15 => day15::Problem {}.solve(),
        16 => day16::Problem {}.solve(),
        d => println!("Day {} hasn't been solved yet :(", d),
    }
}

fn main() {
    let day = std::env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("1"))
        .parse()
        .unwrap_or(1);
    solve_day(day);
}
