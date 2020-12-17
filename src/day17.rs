use std::collections::HashSet;
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

type State = HashSet<Vec3>;

type State4 = HashSet<Vec4>;

pub fn generate(input: &str) -> State {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.bytes()
                .enumerate()
                .filter(|(_, c)| *c == b'#')
                .map(move |(x, _)| Vec3(x as isize, y as isize, 1_isize))
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
        let cneighbors = neighborsp1(c).filter(|v| state.contains(&v)).count();
        let s = state.contains(c);
        match (s, cneighbors) {
            (true, 2 | 3) => ret.insert(*c),
            (false, 3) => ret.insert(*c),
            _ => ret.remove(c),
        };
    }

    ret
}

fn transformp2(state: &State) -> State4 {
    state.iter().map(|&v| Vec4(v.0, v.1, v.2, 1)).collect()
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

    // not using this since almost twice as slow as just being dumb
    // let vecs = state.iter().flat_map(|c| neighborsp2(c)).chain(state.iter().cloned());

    let vecs = (start.0..=end.0).flat_map(|x| {
        (start.1..=end.1).flat_map(move |y| {
            (start.2..=end.2).flat_map(move |z| (start.3..=end.3).map(move |w| Vec4(x, y, z, w)))
        })
    });

    for ref c in vecs {
        let cneighbors = neighborsp2(c).filter(|v| state.contains(&v)).count();
        let s = state.contains(c);
        match (s, cneighbors) {
            (true, 2 | 3) => ret.insert(*c),
            (false, 3) => ret.insert(*c),
            _ => ret.remove(c),
        };
    }

    ret
}

fn minmaxp1(state: &State) -> (Vec3, Vec3) {
    state
        .iter()
        .map(|&s| (s, s))
        .fold_first(|(min, max), (c, _)| {
            (
                Vec3(c.0.min(min.0), c.1.min(min.1), c.2.min(min.2)),
                Vec3(c.0.max(max.0), c.1.max(max.1), c.2.max(max.2)),
            )
        })
        .unwrap()
}

fn minmaxp2(state: &State4) -> (Vec4, Vec4) {
    state
        .iter()
        .map(|&s| (s, s))
        .fold_first(|(min, max), (c, _)| {
            (
                Vec4(
                    c.0.min(min.0),
                    c.1.min(min.1),
                    c.2.min(min.2),
                    c.3.min(min.3),
                ),
                Vec4(
                    c.0.max(max.0),
                    c.1.max(max.1),
                    c.2.max(max.2),
                    c.3.max(max.3),
                ),
            )
        })
        .unwrap()
}

pub fn part1(input: &State) -> usize {
    let (min, max) = minmaxp1(input);
    (0..6)
        .fold((min, max, input.clone()), |(min, max, s), _| {
            // just be stupid with minmax
            (min + (-1), max + 1, iteratep1(&s, (min, max)))
        })
        .2
        .len()
}

pub fn part2(input: &State) -> usize {
    let input = transformp2(input);

    let (min, max) = minmaxp2(&input);
    (0..6)
        .fold((min, max, input), |(min, max, s), _| {
            (min + (-1), max + 1, iteratep2(&s, (min, max)))
        })
        .2
        .len()
}
