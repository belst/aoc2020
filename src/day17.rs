use std::collections::HashMap;
use std::ops::Add;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Vec3(isize, isize, isize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Vec4(isize, isize, isize, isize);

impl Add<isize> for Vec3 {
    type Output = Self;
    fn add(self, other: isize) -> Self::Output {
        Vec3(self.0 + other, self.1 + other, self.2 + other)
    }
}

impl Add<isize> for Vec4 {
    type Output = Self;
    fn add(self, other: isize) -> Self::Output {
        Vec4(
            self.0 + other,
            self.1 + other,
            self.2 + other,
            self.3 + other,
        )
    }
}

type State = HashMap<Vec3, bool>;

type State4 = HashMap<Vec4, bool>;

pub fn generate(input: &str) -> State {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.bytes().enumerate().map(move |(x, c)| {
                if c == b'#' {
                    (Vec3(x as isize, y as isize, 1_isize), true)
                } else {
                    (Vec3(x as isize, y as isize, 1_isize), false)
                }
            })
        })
        .collect()
}

fn neighborsp1(vo @ Vec3(xo, yo, zo): &Vec3) -> impl Iterator<Item = Vec3> + '_ {
    (-1..=1)
        .flat_map(move |x| {
            (-1..=1).flat_map(move |y| (-1..=1).map(move |z| Vec3(xo + x, yo + y, zo + z)))
        })
        .filter(move |v| v != vo)
}

fn iteratep1(state: &State, (min, max): (Vec3, Vec3)) -> State {
    let mut ret = state.clone();
    let (start, end) = (min + (-1), max + 1);

    let vecs = (start.0..=end.0).flat_map(|x| {
        (start.1..=end.1).flat_map(move |y| (start.2..=end.2).map(move |z| Vec3(x, y, z)))
    });

    for ref c in vecs {
        let cneighbors = neighborsp1(c)
            .filter_map(|v| state.get(&v))
            .filter(|&&cube| cube)
            .count();
        let s = *state.get(c).unwrap_or(&false);
        match (s, cneighbors) {
            (true, 2 | 3) => ret.insert(*c, true),
            (false, 3) => ret.insert(*c, true),
            _ => ret.insert(*c, false),
        };
    }

    ret
}

fn transformp2(state: &State) -> State4 {
    state
        .iter()
        .map(|(&k, &v)| (Vec4(k.0, k.1, k.2, 1), v))
        .collect()
}

fn neighborsp2(vo @ Vec4(xo, yo, zo, wo): &Vec4) -> impl Iterator<Item = Vec4> + '_ {
    (-1..=1)
        .flat_map(move |x| {
            (-1..=1).flat_map(move |y| {
                (-1..=1)
                    .flat_map(move |z| (-1..=1).map(move |w| Vec4(xo + x, yo + y, zo + z, wo + w)))
            })
        })
        .filter(move |v| v != vo)
}

fn iteratep2(state: &State4, (min, max): (Vec4, Vec4)) -> State4 {
    let mut ret = state.clone();
    let (start, end) = (min + (-1), max + 1);

    let vecs = (start.0..=end.0).flat_map(|x| {
        (start.1..=end.1).flat_map(move |y| {
            (start.2..=end.2).flat_map(move |z| (start.3..=end.3).map(move |w| Vec4(x, y, z, w)))
        })
    });

    for ref c in vecs {
        let cneighbors = neighborsp2(c)
            .filter_map(|v| state.get(&v))
            .filter(|&&cube| cube)
            .count();
        let s = *state.get(c).unwrap_or(&false);
        match (s, cneighbors) {
            (true, 2 | 3) => ret.insert(*c, true),
            (false, 3) => ret.insert(*c, true),
            _ => ret.insert(*c, false),
        };
    }

    ret
}

pub fn part1(input: &State) -> usize {
    let min = *input.keys().min().unwrap();
    let max = *input.keys().max().unwrap();
    (0..6)
        .fold((min, max, input.clone()), |(min, max, s), _| {
            (min + (-1), max + 1, iteratep1(&s, (min, max)))
        })
        .2
        .values()
        .filter(|&&v| v)
        .count()
}

pub fn part2(input: &State) -> usize {
    let input = transformp2(input);

    let min = *input.keys().min().unwrap();
    let max = *input.keys().max().unwrap();
    (0..6)
        .fold((min, max, input), |(min, max, s), _| {
            (min + (-1), max + 1, iteratep2(&s, (min, max)))
        })
        .2
        .values()
        .filter(|&&v| v)
        .count()
}
