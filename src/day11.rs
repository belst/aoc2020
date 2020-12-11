use std::collections::HashMap;
type Coord = (isize, isize);

pub fn generate(input: &str) -> HashMap<Coord, bool> {
    let mut hm = HashMap::new();
    for (y, l) in input.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            if c == 'L' {
                hm.insert((x as isize, y as isize), false);
            }
        }
    }
    hm
}

fn surrounding1((x, y): Coord, (xmax, ymax): Coord, _state: &HashMap<Coord, bool>) -> Vec<Coord> {
    [
        (x - 1, y - 1),
        (x, y - 1),
        (x + 1, y - 1),
        (x - 1, y),
        (x + 1, y),
        (x - 1, y + 1),
        (x, y + 1),
        (x + 1, y + 1),
    ]
    .iter()
    .cloned()
    .filter(|(x, y)| x >= &0 && y >= &0 && x <= &xmax && y <= &ymax)
    .collect()
}

fn iterate(state: &HashMap<Coord, bool>, max: Coord, threshold: usize) -> HashMap<Coord, bool> {
    let surrounding = if threshold == 4 {
        // 4 is part1, 5 is part2
        surrounding1
    } else {
        surrounding2
    };
    let mut ret = state.clone();
    for (&k, _) in state {
        match surrounding(k, max, state)
            .iter()
            .filter_map(|c| state.get(c))
            .filter(|&&s| s)
            .count()
        {
            0 => ret.insert(k, true),
            n if n >= threshold => ret.insert(k, false),
            _ => None,
        };
    }
    ret
}

pub fn part1(input: &HashMap<Coord, bool>) -> usize {
    let mut oldstate = input.clone();
    let max = input.keys().max().unwrap().clone();

    loop {
        let newstate = iterate(&oldstate, max, 4);
        if newstate == oldstate {
            return newstate.values().filter(|&&v| v).count();
        }
        oldstate = newstate;
    }
}

fn surrounding2((x, y): Coord, (xmax, ymax): Coord, state: &HashMap<Coord, bool>) -> Vec<Coord> {
    (0..x)
        .rev()
        .find(|&xs| state.contains_key(&(xs, y)))
        .map(|x| (x, y))
        .into_iter()
        .chain(
            (0..y)
                .rev()
                .find(|&ys| state.contains_key(&(x, ys)))
                .map(|y| (x, y)),
        )
        .chain(
            (x + 1..=xmax)
                .find(|&xs| state.contains_key(&(xs, y)))
                .map(|x| (x, y)),
        )
        .chain(
            (y + 1..=ymax)
                .find(|&ys| state.contains_key(&(x, ys)))
                .map(|y| (x, y)),
        )
        .chain(
            (0..x)
                .rev()
                .zip((0..y).rev())
                .find(|&(xs, ys)| state.contains_key(&(xs, ys))),
        )
        .chain(
            (x + 1..=xmax)
                .zip((0..y).rev())
                .find(|&(xs, ys)| state.contains_key(&(xs, ys))),
        )
        .chain(
            (0..x)
                .rev()
                .zip(y + 1..=ymax)
                .find(|&(xs, ys)| state.contains_key(&(xs, ys))),
        )
        .chain(
            (x + 1..=xmax)
                .zip(y + 1..=ymax)
                .find(|&(xs, ys)| state.contains_key(&(xs, ys))),
        )
        .collect()
}

pub fn part2(input: &HashMap<Coord, bool>) -> usize {
    let mut oldstate = input.clone();
    let max = input.keys().max().unwrap().clone();

    loop {
        let newstate = iterate(&oldstate, max, 5);
        if newstate == oldstate {
            return newstate.values().filter(|&&v| v).count();
        }
        oldstate = newstate;
    }
}
