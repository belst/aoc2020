use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
pub fn input_gen(input: &str) -> Vec<usize> {
    input
        .lines()
        .filter_map(|x| x.parse().ok().filter(|&y| y < 2020))
        .collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[usize]) -> Option<usize> {
    input
        .iter()
        .flat_map(|n| {
            input
                .iter()
                .filter(move |&m| n + m == 2020)
                .map(move |&m| n * m)
        })
        .next()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[usize]) -> Option<usize> {
    input
        .iter()
        .flat_map(|n| {
            input
                .iter()
                .filter(move |&m| n + m < 2020)
                .flat_map(move |m| {
                    input
                        .iter()
                        .filter(move |&k| n + m + k == 2020)
                        .map(move |k| n * m * k)
                })
        })
        .next()
}
