use aoc_runner_derive::{aoc, aoc_generator};


#[aoc_generator(day5)]
pub fn generate(input: &str) -> Vec<usize> {
    let mut ret: Vec<usize> = input.lines().map(|s| {
        let (mut min, mut max) = (0, 127);
        for d in  s.chars().take(7) {
            match d {
                'F' => max = (min + max) / 2,
                'B' => min = (min + max + 1) / 2,
                _ => {},
            }
        }
        let r = min;
        min = 0;
        max = 7;
        for d in s.chars().skip(7) {
            match d {
                'L' => max = (min + max) / 2,
                'R' => min = (min + max + 1) / 2,
                _ => {},
            }
        }
        r * 8 + min
    }).collect();
    ret.sort_unstable();
    ret
}

#[aoc(day5, part1)]
pub fn part1(input: &[usize]) -> usize {
    *input.iter().last().unwrap()
}

#[aoc(day5, part2)]
pub fn part2(input: &[usize]) -> usize {
    let mut lastid = input[0] - 1;
    for &s in input {
        if lastid + 1 == s {
            lastid = s;
            continue;
        }
        return lastid + 1;
    }
    unreachable!()
}