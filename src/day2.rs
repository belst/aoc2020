use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
pub struct PasswordReq {
    val: char,
    amount: (usize, usize),
    pw: String,
}

impl PasswordReq {
    fn valid1(&self) -> bool {
        (self.amount.0..=self.amount.1)
            .contains(&self.pw.chars().filter(|&c| c == self.val).count())
    }

    fn valid2(&self) -> bool {
        let mut it = self.pw.chars();
        let chars = [
            it.nth(self.amount.0 - 1).unwrap(),
            it.nth(self.amount.1 - self.amount.0 - 1).unwrap(),
        ];
        chars.iter().any(|&c| c == self.val) && !chars.iter().all(|&c| c == self.val)
    }
}

#[aoc_generator(day2)]
pub fn input_gen(input: &str) -> Vec<PasswordReq> {
    input
        .lines()
        .map(|l| {
            let mut parts = l.split_whitespace();
            let p1 = parts.next().unwrap();
            let val = parts.next().unwrap().trim_end_matches(':');
            let pw = parts.next().unwrap();
            let mut range = p1.split('-').map(|n| n.parse().unwrap());

            PasswordReq {
                val: val.chars().next().unwrap(),
                amount: (range.next().unwrap(), range.next().unwrap()),
                pw: pw.into(),
            }
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &[PasswordReq]) -> usize {
    input.iter().filter(|p| p.valid1()).count()
}

#[aoc(day2, part2)]
pub fn part2(input: &[PasswordReq]) -> usize {
    input.iter().filter(|p| p.valid2()).count()
}
