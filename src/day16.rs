use std::collections::HashMap;
use std::ops::RangeInclusive;

type Rules = HashMap<String, Vec<RangeInclusive<usize>>>;
#[derive(Debug)]
pub struct Tickets {
    rules: Rules,
    my_ticket: Vec<usize>,
    other_tickets: Vec<Vec<usize>>,
}

impl Tickets {
    fn valid(&self, n: usize) -> bool {
        self.rules.values().flatten().any(|r| r.contains(&n))
    }
}

fn parse_rule(input: &str) -> (String, Vec<RangeInclusive<usize>>) {
    let (name, rules) = input.split_once(": ").unwrap();
    let rules: Vec<_> = rules
        .split(" or ")
        .map(|r| {
            let (low, high) = r.split_once("-").unwrap();
            low.parse().unwrap()..=high.parse().unwrap()
        })
        .collect();

    (name.into(), rules)
}

fn parse_rules(input: &str) -> Rules {
    input.lines().map(parse_rule).collect()
}

fn parse_ticket(input: &str) -> Vec<usize> {
    input.split(',').flat_map(|n| n.parse().ok()).collect()
}

fn rule_valid(rules: &[RangeInclusive<usize>], n: usize) -> bool {
    rules.iter().any(|r| r.contains(&n))
}

pub fn generate(input: &str) -> Tickets {
    let ins: Vec<&str> = input.split("\n\n").collect();
    let rules = parse_rules(ins[0]);
    let my_ticket = parse_ticket(ins[1].lines().nth(1).unwrap());
    let other_tickets = ins[2].lines().skip(1).map(parse_ticket).collect();

    Tickets {
        rules,
        my_ticket,
        other_tickets,
    }
}

pub fn part1(input: &Tickets) -> usize {
    input
        .other_tickets
        .iter()
        .flatten()
        .filter(|&&n| !input.valid(n))
        .sum()
}

pub fn part2(input: &Tickets) -> usize {
    let mut valid_rules = vec![input.rules.clone(); input.my_ticket.len()];
    let valids = input
        .other_tickets
        .iter()
        .filter(|&v| v.iter().all(|&n| input.valid(n)));

    // Initialize with own ticket
    for i in 0..valid_rules.len() {
        valid_rules[i].retain(|_, r| rule_valid(&r, input.my_ticket[i]));
    }

    // reduce with valid neighbor tickets
    for v in valids {
        for i in 0..valid_rules.len() {
            valid_rules[i].retain(|_, r| rule_valid(&r, v[i]));
        }
    }

    // remove single rules from all other indizes
    while valid_rules.iter().any(|r| r.len() > 1) {
        for i in 0..valid_rules.len() {
            if valid_rules[i].len() > 1 {
                continue;
            }
            let key = valid_rules[i].keys().next().unwrap().clone();
            for j in 0..valid_rules.len() {
                if i == j {
                    continue;
                }
                valid_rules[j].remove(&key);
            }
        }
    }

    valid_rules
        .iter()
        .enumerate()
        .filter(|(_, r)| r.keys().next().unwrap().starts_with("departure"))
        .map(|(i, _)| input.my_ticket[i])
        .product()
}
