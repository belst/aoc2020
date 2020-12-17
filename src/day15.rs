use std::collections::HashMap;

pub fn generate(input: &str) -> Vec<usize> {
    input.split(",").filter_map(|n| n.parse().ok()).collect()
}

fn solve(input: &[usize], limit: usize) -> usize {
    let mut last_seen: HashMap<_, _> = input[..input.len() - 1]
        .iter()
        .enumerate()
        .map(|(l, &r)| (r, l))
        .collect();

    let last = input[input.len() - 1];

    (input.len() - 1..limit - 1).fold(last, |last, curr| {
        let new = if let Some(&idx) = last_seen.get(&last) {
            curr - idx
        } else {
            0
        };
        last_seen.insert(last, curr);
        new
    })
}

pub fn part1(input: &[usize]) -> usize {
    solve(input, 2020)
}

pub fn part2(input: &[usize]) -> usize {
    solve(input, 30000000)
}

#[cfg(test)]
mod test {
    #[test]
    fn part1() {
        assert_eq!(super::part1(&[1, 3, 2]), 1);
        assert_eq!(super::part1(&[2, 1, 3]), 10);
        assert_eq!(super::part1(&[1, 2, 3]), 27);
        assert_eq!(super::part1(&[2, 3, 1]), 78);
        assert_eq!(super::part1(&[3, 2, 1]), 438);
        assert_eq!(super::part1(&[3, 1, 2]), 1836);
    }

    // #[test]
    // fn part2() {
    //     assert_eq!(super::part2(&[0, 3, 6]), 175594);
    //     assert_eq!(super::part2(&[1, 3, 2]), 2578);
    //     assert_eq!(super::part2(&[2, 1, 3]), 3544142);
    //     assert_eq!(super::part2(&[1, 2, 3]), 261214);
    //     assert_eq!(super::part2(&[2, 3, 1]), 6895259);
    //     assert_eq!(super::part2(&[3, 2, 1]), 18);
    //     assert_eq!(super::part2(&[3, 1, 2]), 362);
    // }
}
