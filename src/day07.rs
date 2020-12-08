use crate::solver::Solver;
use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};
use std::io::{self, BufRead, BufReader};

type Bag = String;
type BagContent = Vec<(String, i32)>;
#[derive(Debug, Default)]
pub struct BagEntry {
    content: BagContent,
    parents: Vec<Bag>,
}
type BagRule = HashMap<Bag, BagEntry>;

pub struct Problem;

impl Solver for Problem {
    type Input = BagRule;
    type Output1 = usize;
    type Output2 = i32;

    fn get_day(&self) -> i32 {
        7
    }

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        let r = BufReader::new(r);
        let mut result = HashMap::new();

        let bag_name_regex = Regex::new(r"^(?P<bag_name>\w+ \w+) bag").unwrap();
        let content_list_regex = Regex::new(r"(?P<count>\d+) (?P<child_bag>\w+ \w+) bag").unwrap();
        for rule in r.lines().flatten().filter(|l| !l.is_empty()) {
            let bag_name = bag_name_regex.captures(&rule).unwrap()["bag_name"].to_string();
            if rule.contains("no other") {
                result.entry(bag_name).or_insert(BagEntry::default());
            } else {
                let mut content: BagContent = Vec::new();
                for captures in content_list_regex.captures_iter(&rule) {
                    let count: i32 = captures["count"].parse().unwrap();
                    let child_bag = captures["child_bag"].to_string();
                    result
                        .entry(child_bag.clone())
                        .or_insert(BagEntry::default())
                        .parents
                        .push(bag_name.clone());
                    content.push((child_bag, count));
                }

                result
                    .entry(bag_name)
                    .or_insert(BagEntry::default())
                    .content = content;
            }
        }

        result
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        let orig_bag = "shiny gold";
        let mut processing_bags: VecDeque<_> = input[orig_bag].parents.iter().collect();
        let mut parent_bags = HashSet::new();

        while let Some(next_bag) = processing_bags.pop_front() {
            parent_bags.insert(next_bag.clone());
            processing_bags.extend(input[next_bag].parents.iter());
        }

        parent_bags.len()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        let mut bag_count: HashMap<String, i32> = HashMap::new();

        fn bag_count_fn(input: &BagRule, bag: &str, bag_count: &mut HashMap<String, i32>) -> i32 {
            if bag_count.contains_key(bag) {
                bag_count[bag]
            } else if input[bag].content.is_empty() {
                bag_count.insert(bag.to_string(), 0);
                0
            } else {
                let result: i32 = input[bag]
                    .content
                    .iter()
                    .fold(0, |acc, (child_bag, count)| {
                        acc + count + count * bag_count_fn(input, child_bag, bag_count)
                    });
                bag_count.insert(bag.to_string(), result);
                result
            }
        }

        bag_count_fn(input, "shiny gold", &mut bag_count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() {
        let raw_input = r"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

        let problem = Problem {};
        let input = problem.parse_input(raw_input.as_bytes());
        assert_eq!(problem.solve_first(&input), 4);
    }

    #[test]
    fn test_second() {
        let raw_input = r"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

        let problem = Problem {};
        let input = problem.parse_input(raw_input.as_bytes());
        assert_eq!(problem.solve_second(&input), 32);
    }

    #[test]
    fn test_second_2() {
        let raw_input = r"shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.
";

        let problem = Problem {};
        let input = problem.parse_input(raw_input.as_bytes());
        assert_eq!(problem.solve_second(&input), 126);
    }
}
