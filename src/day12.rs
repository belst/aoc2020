use std::ops::{Add, AddAssign, Mul};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instr {
    North(isize),
    South(isize),
    East(isize),
    West(isize),
    Rotate(isize),
    Forward(isize),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Vec2(isize, isize);

impl Add for Vec2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
        self.1 += other.1;
    }
}

impl Mul<isize> for Vec2 {
    type Output = Self;
    fn mul(self, other: isize) -> Self {
        Self(self.0 * other, self.1 * other)
    }
}

#[derive(Debug, Clone)]
struct Ship {
    pos: Vec2,
    dir: Vec2,
}

impl Ship {
    fn new() -> Self {
        Self {
            pos: Vec2(0, 0),
            dir: Vec2(1, 0),
        }
    }

    fn with_waypoint(wp: Vec2) -> Self {
        Self {
            pos: Vec2(0, 0),
            dir: wp,
        }
    }

    fn move_ship(&mut self, instr: &Instr) {
        use Instr::*;

        match *instr {
            North(n) => self.pos += Vec2(0, n),
            South(n) => self.pos += Vec2(0, -n),
            East(n) => self.pos += Vec2(n, 0),
            West(n) => self.pos += Vec2(-n, 0),
            Rotate(n) => (0..n).step_by(90).for_each(|_| self.rotate90()),
            Forward(n) => self.pos += self.dir * n,
        }
    }

    fn move_waypoint(&mut self, instr: &Instr) {
        use Instr::*;
        match *instr {
            North(n) => self.dir += Vec2(0, n),
            South(n) => self.dir += Vec2(0, -n),
            East(n) => self.dir += Vec2(n, 0),
            West(n) => self.dir += Vec2(-n, 0),
            Rotate(n) => (0..n).step_by(90).for_each(|_| self.rotate90()),
            Forward(n) => self.pos += self.dir * n,
        }
    }

    fn rotate90(&mut self) {
        self.dir = Vec2(self.dir.1, -self.dir.0)
    }
}

pub fn generate(input: &str) -> Vec<Instr> {
    input
        .lines()
        .map(|l| l.as_bytes())
        .map(|l: &[u8]| match l[0] {
            b'N' => Instr::North(std::str::from_utf8(&l[1..]).unwrap().parse().unwrap()),
            b'S' => Instr::South(std::str::from_utf8(&l[1..]).unwrap().parse().unwrap()),
            b'E' => Instr::East(std::str::from_utf8(&l[1..]).unwrap().parse().unwrap()),
            b'W' => Instr::West(std::str::from_utf8(&l[1..]).unwrap().parse().unwrap()),
            b'F' => Instr::Forward(std::str::from_utf8(&l[1..]).unwrap().parse().unwrap()),
            b'R' => Instr::Rotate(std::str::from_utf8(&l[1..]).unwrap().parse().unwrap()),
            b'L' => Instr::Rotate(
                360 - std::str::from_utf8(&l[1..])
                    .unwrap()
                    .parse::<isize>()
                    .unwrap(),
            ),
            _ => unreachable!(),
        })
        .collect()
}

pub fn part1(input: &[Instr]) -> isize {
    let mut ship = Ship::new();

    for i in input {
        ship.move_ship(i);
    }

    ship.pos.0.abs() + ship.pos.1.abs()
}

pub fn part2(input: &[Instr]) -> isize {
    let mut ship = Ship::with_waypoint(Vec2(10, 1));

    for i in input {
        ship.move_waypoint(i);
    }

    ship.pos.0.abs() + ship.pos.1.abs()
}

#[cfg(test)]
mod test {
    const INPUT: &'static str = "F10
N3
F7
R90
F11";

    #[test]
    fn testp1() {
        let instr = super::generate(INPUT);
        assert_eq!(super::part1(&instr), 25);
    }

    #[test]
    fn testp2() {
        let instr = super::generate(INPUT);
        assert_eq!(super::part2(&instr), 286);
    }
}
