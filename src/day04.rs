use crate::solver::Solver;
use regex::Regex;
use std::io;

#[derive(Default)]
pub struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    #[allow(dead_code)]
    cid: Option<String>,
}

impl Passport {
    fn from_string(s: &str) -> Self {
        let re = Regex::new(r"(?P<key>byr|iyr|eyr|hgt|hcl|ecl|pid|cid):(?P<value>\S+)").unwrap();
        let (mut byr, mut iyr, mut eyr, mut hgt, mut hcl, mut ecl, mut pid, mut cid) = (
            Option::None,
            Option::None,
            Option::None,
            Option::None,
            Option::None,
            Option::None,
            Option::None,
            Option::None,
        );

        for cap in re.captures_iter(s) {
            match &cap["key"] {
                "byr" => byr = Some(cap["value"].to_string()),
                "iyr" => iyr = Some(cap["value"].to_string()),
                "eyr" => eyr = Some(cap["value"].to_string()),
                "hgt" => hgt = Some(cap["value"].to_string()),
                "hcl" => hcl = Some(cap["value"].to_string()),
                "ecl" => ecl = Some(cap["value"].to_string()),
                "pid" => pid = Some(cap["value"].to_string()),
                "cid" => cid = Some(cap["value"].to_string()),
                _ => {}
            }
        }

        Self {
            byr,
            iyr,
            eyr,
            hgt,
            hcl,
            ecl,
            pid,
            cid,
        }
    }

    fn is_valid_pt1(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }

    fn is_valid_pt2(&self) -> bool {
        self.is_byr_valid()
            && self.is_iyr_valid()
            && self.is_eyr_valid()
            && self.is_hgt_valid()
            && self.is_hcl_valid()
            && self.is_ecl_valid()
            && self.is_pid_valid()
    }

    fn is_byr_valid(&self) -> bool {
        match &self.byr {
            Some(s) => s
                .parse::<i32>()
                .map_or(false, |num| num >= 1920 && num <= 2002),
            None => false,
        }
    }

    fn is_iyr_valid(&self) -> bool {
        match &self.iyr {
            Some(s) => s
                .parse::<i32>()
                .map_or(false, |num| num >= 2010 && num <= 2020),
            None => false,
        }
    }

    fn is_eyr_valid(&self) -> bool {
        match &self.eyr {
            Some(s) => s
                .parse::<i32>()
                .map_or(false, |num| num >= 2020 && num <= 2030),
            None => false,
        }
    }

    fn is_hgt_valid(&self) -> bool {
        let re = Regex::new(r"^(?P<num>\d+)(?P<unit>cm|in)$").unwrap();
        self.hgt.as_deref().map_or(false, |s| {
            re.captures(s).map_or(false, |caps| {
                let num: i32 = caps["num"].parse().unwrap();
                match &caps["unit"] {
                    "cm" => num >= 150 && num <= 193,
                    "in" => num >= 59 && num <= 76,
                    _ => false,
                }
            })
        })
    }

    fn is_hcl_valid(&self) -> bool {
        let re = Regex::new(r"^#[a-f0-9]{6}$").unwrap();
        self.hcl.as_deref().map_or(false, |s| re.is_match(s))
    }

    fn is_ecl_valid(&self) -> bool {
        match &self.ecl {
            Some(s) => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&s.as_ref()),
            None => false,
        }
    }

    fn is_pid_valid(&self) -> bool {
        let re = Regex::new(r"^\d{9}$").unwrap();
        self.pid.as_deref().map_or(false, |s| re.is_match(s))
    }
}

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<Passport>;
    type Output1 = usize;
    type Output2 = usize;

    fn get_day(&self) -> i32 {
        4
    }

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        let mut buf = String::new();
        let mut r = r;
        let _ = r.read_to_string(&mut buf);
        let lines = buf.split("\n\n");
        lines.map(Passport::from_string).collect()
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
        let input = r".ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in
 ";
        let problem = Problem {};
        let grid = problem.parse_input(input.as_bytes());
        assert_eq!(problem.solve_first(&grid), 2);
    }

    #[test]
    fn test_second_invalid() {
        let input = r"eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007
";
        let problem = Problem {};
        let grid = problem.parse_input(input.as_bytes());
        assert_eq!(problem.solve_second(&grid), 0);
    }

    #[test]
    fn test_second_valid() {
        let input = r"pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
";
        let problem = Problem {};
        let grid = problem.parse_input(input.as_bytes());
        assert_eq!(problem.solve_second(&grid), 4);
    }
}
