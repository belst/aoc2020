use aoc_runner_derive::{aoc, aoc_generator};
use derive_builder::Builder;

#[derive(Debug, Builder)]
pub struct Passport {
    byr: String,
    iyr: String,
    eyr: String,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    cid: Option<String>,
}

impl Passport {
    fn valid(&self) -> bool {
        self.byr
            .parse()
            .ok()
            .filter(|v| (1920..=2002).contains(v))
            .and(self.iyr.parse().ok().filter(|v| (2010..=2020).contains(v)))
            .and(self.eyr.parse().ok().filter(|v| (2020..=2030).contains(v)))
            .and(
                match &self.hgt.trim_end_matches("cm").trim_end_matches("in") {
                    cm if self.hgt.ends_with("cm") => {
                        cm.parse().ok().filter(|v| (150..=193).contains(v))
                    }
                    inch if self.hgt.ends_with("in") => {
                        inch.parse().ok().filter(|v| (59..=76).contains(v))
                    }
                    _ => None,
                },
            )
            .and(if self.hcl.starts_with('#') && self.hcl[1..].len() == 6 {
                usize::from_str_radix(&self.hcl[1..], 16).ok()
            } else {
                None
            })
            .and(
                if ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&self.ecl.as_ref()) {
                    Some(1)
                } else {
                    None
                },
            )
            .and(
                self.pid
                    .parse()
                    .ok()
                    .filter(|_: &usize| self.pid.len() == 9),
            )
            .is_some()
    }
}

fn parse(input: &str) -> Option<Passport> {
    let mut ppb = PassportBuilder::default();
    for mut c in input.split_whitespace().map(|c| c.split(':')) {
        ppb.cid(None);
        match (c.next().unwrap(), c.next().unwrap()) {
            ("byr", v) => ppb.byr(v.into()),
            ("iyr", v) => ppb.iyr(v.into()),
            ("eyr", v) => ppb.eyr(v.into()),
            ("hgt", v) => ppb.hgt(v.into()),
            ("hcl", v) => ppb.hcl(v.into()),
            ("ecl", v) => ppb.ecl(v.into()),
            ("pid", v) => ppb.pid(v.into()),
            ("cid", v) => ppb.cid(v.parse().ok()),
            _ => unreachable!(),
        };
    }

    ppb.build().ok()
}

#[aoc_generator(day4)]
pub fn generate(input: &str) -> Vec<Passport> {
    input.split("\n\n").filter_map(parse).collect()
}

#[aoc(day4, part1)]
pub fn part1(input: &[Passport]) -> usize {
    input.len()
}

#[aoc(day4, part2)]
pub fn part2(input: &[Passport]) -> usize {
    input.iter().filter(|pp| pp.valid()).count()
}

#[test]
fn testp1() {
    const INPUT: &'static str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

    let pps = dbg!(generate(INPUT));
    assert_eq!(part1(&pps), 2);
}

#[test]
fn testp2valid() {
    const INPUT: &'static str = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
    ";

    let pps = dbg!(generate(INPUT));

    assert_eq!(part2(&pps), 4);
}

#[test]
fn testp2invalid() {
    const INPUT: &'static str = "eyr:1972 cid:100
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

    let pps = dbg!(generate(INPUT));

    assert_eq!(part2(&pps), 0);
}
