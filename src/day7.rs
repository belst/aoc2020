use std::cell::RefCell;
use std::collections::{HashMap, HashSet};

type BagTree = HashMap<String, Vec<(usize, String)>>;

pub fn generatep2(input: &str) -> BagTree {
    // vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
    // bright white bags contain 1 shiny gold bag.
    let mut hm: BagTree = HashMap::new();
    for l in input.lines() {
        let mut childs = vec![];
        let color: String = l.split_whitespace().take(2).collect();
        let childrenstr = l.split(" contain ").nth(1).unwrap();
        for c in childrenstr.split(", ") {
            let nu = c.split_whitespace().next().unwrap();
            if nu == "no" {
                continue;
            }
            let count = nu.parse().unwrap();
            childs.push((count, c.split_whitespace().skip(1).take(2).collect()));
        }
        hm.entry(color).or_insert(vec![]).extend(childs);
    }

    hm
}

type InvertedBagTree = HashMap<String, HashSet<String>>;

pub fn generatep1(input: &str) -> InvertedBagTree {
    let mut hm: InvertedBagTree = HashMap::new();

    for l in input.lines() {
        let color: String = l.split_whitespace().take(2).collect();
        let childrenstr = l.split(" contain ").nth(1).unwrap();
        for c in childrenstr.split(", ") {
            let curr = c.split_whitespace().skip(1).take(2).collect();
            hm.entry(curr)
                .or_insert_with(HashSet::new)
                .insert(color.clone());
        }
    }
    hm
}

fn go(hm: &InvertedBagTree, current: &str, seen: &RefCell<HashSet<String>>) {
    seen.borrow_mut().insert(current.into());
    if let Some(p) = hm.get(current) {
        for c in p.iter().filter(|&c| !seen.borrow().contains(c)) {
            go(hm, c, seen);
        }
    }
}

pub fn part1(input: &str) -> usize {
    let input = generatep1(input);
    let seen = RefCell::new(HashSet::new());
    go(&input, "shinygold", &seen);
    seen.into_inner().len() - 1
}

fn go2(hm: &BagTree, current: &str) -> usize {
    1 + if let Some(cs) = hm.get(current) {
        cs.iter().map(|(c, next)| c * go2(hm, next)).sum()
    } else {
        0
    }
}

pub fn part2(input: &str) -> usize {
    let input = generatep2(input);
    go2(&input, "shinygold") - 1
}

#[test]
fn testp1() {
    const INPUT: &'static str = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

    assert_eq!(part1(&INPUT), 4);
}

#[test]
fn testp2small() {
    const INPUT: &'static str = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

    assert_eq!(part2(&INPUT), 32);
}

#[test]
fn testp2big() {
    const INPUT: &'static str = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";

    assert_eq!(part2(&INPUT), 126);
}
