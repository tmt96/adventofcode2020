mod day01;
mod solver;

use crate::solver::Solver;

fn solve_day(day: i32) {
    match day {
        // TODO: Write macro to automate solve by day
        1 => day01::Problem {}.solve(),
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
