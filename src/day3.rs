use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;
use std::ops::Add;

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd)]
struct Coord(usize, usize);
#[derive(Debug)]
pub struct Forest {
    trees: HashSet<Coord>,
    size: Coord,
}

impl Forest {
    fn is_tree(&self, coord: &Coord) -> bool {
        assert!(coord.1 < self.size.1, "Index out of Bounds");
        let coord = Coord(coord.0 % self.size.0, coord.1);
        self.trees.contains(&coord)
    }

    fn count(&self, step: &Coord) -> usize {
        let mut pos = Coord(0, 0);
        let mut c = 0;
        while pos.1 < self.size.1 {
            if self.is_tree(&pos) {
                c += 1;
            }
            pos = &pos + &step;
        }
        c
    }
}

impl Add for &Coord {
    type Output = Coord;

    fn add(self, other: Self) -> Self::Output {
        Coord(self.0 + other.0, self.1 + other.1)
    }
}

#[aoc_generator(day3)]
pub fn generate(input: &str) -> Forest {
    let mut hs = HashSet::new();

    let x = input.chars().position(|c| c == '\n').unwrap();
    let y = input.lines().count();

    for (i, l) in input.lines().enumerate() {
        for (j, c) in l.chars().enumerate() {
            if c == '#' {
                hs.insert(Coord(j, i));
            }
        }
    }

    Forest {
        trees: hs,
        size: Coord(x, y),
    }
}

#[aoc(day3, part1)]
pub fn part1(input: &Forest) -> usize {
    (0..input.size.1)
        .filter(|n| input.is_tree(&Coord(n * 3, n * 1)))
        .count()
}

#[aoc(day3, part2)]
pub fn part2(input: &Forest) -> usize {
    let steps = [
        Coord(1, 1),
        Coord(3, 1),
        Coord(5, 1),
        Coord(7, 1),
        Coord(1, 2),
    ];
    steps.iter().map(|c| input.count(c)).product()
}

#[test]
fn testp1() {
    const INPUT: &'static str = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#
";
    let forest = generate(INPUT);

    assert_eq!(forest.size, Coord(11, 11), "Size");

    let trees = part1(&forest);
    assert_eq!(trees, 7, "Number of Trees Part1");
}

#[test]
fn testp2() {
    const INPUT: &'static str = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#
";
    let forest = generate(INPUT);
    let trees = part2(&forest);
    assert_eq!(trees, 336, "Product of Number of Trees Part2")
}
