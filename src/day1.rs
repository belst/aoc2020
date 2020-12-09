pub fn input_gen(input: &str) -> Vec<usize> {
    input
        .lines()
        .filter_map(|x| x.parse().ok().filter(|&y| y < 2020))
        .collect()
}

pub fn solve_part1(input: &[usize]) -> usize {
    input
        .iter()
        .flat_map(|n| {
            input
                .iter()
                .filter(move |&m| n + m == 2020)
                .map(move |&m| n * m)
        })
        .next()
        .unwrap()
}

pub fn solve_part2(input: &[usize]) -> usize {
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
        .unwrap()
}
