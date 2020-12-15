use std::collections::HashMap;
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Instr {
    SetMask(usize, usize),
    SetMem(usize, usize),
}

impl Instr {
    fn from_mask_str(input: &str) -> Self {
        assert!(input.starts_with("mask = "));
        let mask = input.strip_prefix("mask = ").unwrap();
        let ones = usize::from_str_radix(&mask.replace("X", "0"), 2).unwrap();
        let zeroes = usize::from_str_radix(&mask.replace("X", "1"), 2).unwrap();

        Instr::SetMask(zeroes, ones)
    }

    fn from_mem_str(input: &str) -> Self {
        assert!(input.starts_with("mem["));

        let (addr, value) = input.strip_prefix("mem[").unwrap().split_once("] = ").unwrap();

        Instr::SetMem(addr.parse().unwrap(), value.parse().unwrap())
    }
}



// mask = 100X000X100X00XX1010X0001X11XX100110
// mem[33470] = 43619
// mem[17642] = 12960
// mem[54949] = 1594
// mem[25705] = 17992
// mem[28651] = 47662
// mask = 1000X01110X1101X111X010XX110000X0010
pub fn generate(input: &str) -> Vec<Instr> {
    let mut ret = vec![];
    for l in input.lines() {
        if l.starts_with("mask") {
            ret.push(Instr::from_mask_str(l));
        } else {
            ret.push(Instr::from_mem_str(l));
        }
    }

    ret
}

pub fn part1(input: &[Instr]) -> usize {
    let mut mask = (0, 0);
    if let Instr::SetMask(z, o) = input[0] {
        mask = (z, o);
    }
    let mut state: HashMap<usize, usize> = HashMap::new();
    for instr in &input[1..] {
        match *instr {
            Instr::SetMask(z, o) => mask = (z, o),
            Instr::SetMem(addr, val) => {
                state.insert(addr, (val & mask.0) | mask.1);
            }
        }
    }

    state.values().sum()
}

pub fn part2(input: &[Instr]) -> usize {
    let mut mask = (0, 0, 0);
    if let Instr::SetMask(z, o) = input[0] {
        mask = (z, o, z ^ o);
    }
    let mut state: HashMap<usize, usize> = HashMap::new();

    for instr in &input[1..] {
        match *instr {
            Instr::SetMask(z, o) => mask = (z, o, z ^ o),
            Instr::SetMem(addr, val) => {
                let addr = addr | mask.1;
                let mut floats = mask.2 + 1;
                for _ in 0..2u16.pow(mask.2.count_ones()) {
                    floats = (floats - 1) & mask.2;
                    state.insert(addr ^ floats, val);
                }
            }
        }
    }

    state.values().sum()
}


#[cfg(test)]
mod test {
    const INPUT1: &'static str = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";
    const INPUT2: &'static str = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";

    #[test]
    fn part1() {
        let instrs = super::generate(INPUT1);
        assert_eq!(super::part1(&instrs), 165);
    }

    #[test]
    fn part2() {
        let instrs = super::generate(INPUT2);
        assert_eq!(super::part2(&instrs), 208);
    }

}