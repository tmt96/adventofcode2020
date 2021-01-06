use crate::solver::Solver;
use regex::Regex;
use std::{
    collections::btree_map::Range,
    io::{self, BufRead, BufReader},
};
use std::{collections::HashMap, ops::RangeInclusive};

#[derive(Debug, Clone)]
pub struct Field {
    name: String,
    ranges: Vec<RangeInclusive<i32>>,
}

impl Field {
    fn new(name: &str, lower1: i32, higher1: i32, lower2: i32, higher2: i32) -> Self {
        Self {
            name: name.to_string(),
            ranges: vec![
                RangeInclusive::new(lower1, higher1),
                RangeInclusive::new(lower2, higher2),
            ],
        }
    }

    fn is_value_valid(&self, num: i32) -> bool {
        self.ranges.iter().any(|range| range.contains(&num))
    }
}

#[derive(Debug, Clone)]
pub struct Ticket {
    values: Vec<i32>,
}

impl Ticket {
    fn invalid_values(&self, fields: &[Field]) -> Vec<i32> {
        self.values
            .iter()
            .filter(|&i| !fields.iter().any(|field| field.is_value_valid(*i)))
            .cloned()
            .collect()
    }

    fn is_valid(&self, fields: &[Field]) -> bool {
        self.values
            .iter()
            .all(|&i| fields.iter().any(|field| field.is_value_valid(i)))
    }
}

pub struct Note {
    fields: Vec<Field>,
    ticket: Ticket,
    neighbors: Vec<Ticket>,
}

impl Note {
    fn valid_tickets(&self) -> Vec<Ticket> {
        self.neighbors
            .iter()
            .filter(|ticket| ticket.is_valid(&self.fields))
            .cloned()
            .collect()
    }

    fn column_mapping(&self) -> HashMap<String, usize> {
        let mut result = HashMap::new();
        let mut fields = self.fields.clone();
        let valid_tickets = self.valid_tickets();

        for i in 0..self.fields.len() {
            let chosen_field = fields.remove(
                fields
                    .iter()
                    .position(|field| {
                        valid_tickets
                            .iter()
                            .map(|neighbor| neighbor.values[i])
                            .all(|val| field.is_value_valid(val))
                    })
                    .unwrap(),
            );

            result.insert(chosen_field.name, i);
        }
        result
    }

    fn scanning_error_rate(&self) -> i32 {
        self.neighbors
            .iter()
            .flat_map(|ticket| ticket.invalid_values(&self.fields))
            .sum()
    }
}

pub struct Problem;

impl Solver for Problem {
    type Input = Note;
    type Output1 = i32;
    type Output2 = i32;

    fn get_day(&self) -> i32 {
        16
    }

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        let r = BufReader::new(r);
        let field_regex = Regex::new(
            r"(?P<name>[a-z ]+): (?P<lower1>\d+)-(?P<higher1>\d+) or (?P<lower2>\d+)-(?P<higher2>\d+)",
        ).unwrap();
        let mut fields = Vec::new();
        let mut neighbors = Vec::new();
        let mut my_ticket = Ticket { values: vec![] };
        let mut is_my_ticket = false;
        for line in r.lines() {
            let line = line.unwrap();

            if let Some(captures) = field_regex.captures(&line) {
                fields.push(Field::new(
                    &captures["name"],
                    captures["lower1"].parse().unwrap(),
                    captures["higher1"].parse().unwrap(),
                    captures["lower2"].parse().unwrap(),
                    captures["higher2"].parse().unwrap(),
                ))
            } else if line.contains("your ticket") {
                is_my_ticket = true;
            } else if line.contains("nearby tickets") {
                is_my_ticket = false;
            } else if !line.is_empty() {
                let values = line.split(',').map(|s| s.parse().unwrap()).collect();
                let ticket = Ticket { values };
                if is_my_ticket {
                    my_ticket = ticket;
                } else {
                    neighbors.push(ticket);
                }
            }
        }

        Note {
            fields,
            neighbors,
            ticket: my_ticket,
        }
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        input.scanning_error_rate()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        input
            .column_mapping()
            .iter()
            .filter(|(key, _i)| key.starts_with("departure"))
            .map(|(_key, i)| input.ticket.values[*i])
            .product()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() {
        let raw_input = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12
";
        let problem = Problem {};
        let input = problem.parse_input(raw_input.as_bytes());
        assert_eq!(problem.solve_first(&input), 71);
    }

    #[test]
    fn test_second() {
        let raw_input = "class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9
";
        let problem = Problem {};
        let input = problem.parse_input(raw_input.as_bytes());
        assert_eq!(
            input.column_mapping(),
            vec![
                ("row".to_string(), 0),
                ("class".to_string(), 1),
                ("seat".to_string(), 2),
            ]
            .into_iter()
            .collect()
        );
    }
}
