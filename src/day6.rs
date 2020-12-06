use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

#[aoc_generator(day6)]
pub fn generate(input: &str) -> Vec<(usize, HashMap<char, usize>)> {
    let mut ret = vec![];

    for group in input.split("\n\n") {
        let mut hm = HashMap::new();

        let members = group.lines().count();
        group
            .lines()
            .for_each(|l| l.chars().for_each(|c| *hm.entry(c).or_insert(0) += 1));

        ret.push((members, hm));
    }

    ret
}

#[aoc(day6, part1)]
pub fn part1(input: &[(usize, HashMap<char, usize>)]) -> usize {
    input.iter().map(|(_, hm)| hm.len()).sum()
}

#[aoc(day6, part2)]
pub fn part2(input: &[(usize, HashMap<char, usize>)]) -> usize {
    input
        .iter()
        .map(|(mc, hm)| hm.values().filter(|&v| v == mc).count())
        .sum()
}

#[test]
fn testp1() {
    const INPUT: &'static str = "abc

a
b
c

ab
ac

a
a
a
a

b";
    let questions = generate(INPUT);
    assert_eq!(part1(&questions), 11);
}

#[test]
fn testp2() {
    const INPUT: &'static str = "abc

a
b
c

ab
ac

a
a
a
a

b";
    let questions = generate(INPUT);
    assert_eq!(part2(&questions), 6);
}
