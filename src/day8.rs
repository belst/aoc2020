use std::collections::HashSet;

#[derive(Clone, Copy, Debug)]
pub enum Instr {
    Nop(isize),
    Acc(isize),
    Jmp(isize),
}

impl Instr {
    fn swap(self) -> Self {
        use Instr::*;
        match self {
            Nop(n) => Jmp(n),
            Jmp(n) => Nop(n),
            op => op,
        }
    }
}

type Programm = Vec<Instr>;

pub fn generate(input: &str) -> Programm {
    let mut ret = vec![];
    for l in input.lines() {
        let mut instr = l.split_whitespace();
        ret.push(match instr.next() {
            Some("nop") => Instr::Nop(instr.next().unwrap().parse().unwrap()),
            Some("acc") => Instr::Acc(instr.next().unwrap().parse().unwrap()),
            Some("jmp") => Instr::Jmp(instr.next().unwrap().parse().unwrap()),
            _ => unreachable!(),
        });
    }
    ret
}

pub fn part1(input: &[Instr]) -> isize {
    let mut acc = 0;
    let mut idx: isize = 0;
    let mut seen = HashSet::new();

    while !seen.contains(&idx) {
        seen.insert(idx);
        use Instr::*;
        match input[idx as usize] {
            Nop(_) => idx += 1,
            Acc(n) => {
                acc += n;
                idx += 1
            }
            Jmp(n) => idx += n,
        }
    }

    acc
}

fn tryp2(input: &[Instr]) -> Option<isize> {
    let mut acc = 0;
    let mut idx: isize = 0;
    let mut seen = HashSet::new();

    while !seen.contains(&idx) {
        seen.insert(idx);
        use Instr::*;
        match input[idx as usize] {
            Nop(_) => idx += 1,
            Acc(n) => {
                acc += n;
                idx += 1
            }
            Jmp(n) => idx += n,
        }
        if idx >= input.len() as isize {
            return Some(acc);
        }
    }

    None
}
// Brute force it is
pub fn part2(input: &[Instr]) -> isize {
    let mut input = input.to_vec();
    for i in 0..input.len() {
        if let Instr::Acc(_) = input[i] {
            continue;
        }

        input[i] = input[i].swap();
        if let Some(n) = tryp2(&input) {
            return n;
        }
        input[i] = input[i].swap();
    }
    panic!("Impossible");
}

#[test]
fn testp1() {
    const INPUT: &'static str = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
    let program = generate(INPUT);
    assert_eq!(part1(&program), 5);
}

#[test]
fn testp2() {
    const INPUT: &'static str = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
    let program = generate(INPUT);
    assert_eq!(part2(&program), 8);
}
