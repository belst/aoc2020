use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
pub fn input_gen(input: &str) -> Vec<usize> {
    input.lines()
        .filter_map(|x| x.parse().ok().filter(|&y| y < 2020))
        .collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[usize]) -> usize {
    for &n in input {
        for &m in input {
            if n + m == 2020 {
                return n * m;
            }
        }
    }
    return 0;
}
    


#[aoc(day1, part2)]
pub fn solve_part2(input: &[usize]) -> usize {
    for &n in input {
        for &m in input {
            if n + m >= 2020 {
                continue;
            }
            else {
                for &k in input {
                    if n + m + k == 2020 {
                        return n * m * k;
                    }        
                }
            }
        }
    }
    return 0;
}
