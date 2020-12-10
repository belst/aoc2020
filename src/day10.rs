pub fn generate(input: &str) -> Vec<usize> {
    let mut ret: Vec<usize> = input
        .lines()
        .map(str::parse)
        .filter_map(Result::ok)
        .collect();
    ret.insert(0, 0);
    ret.sort_unstable();
    ret.push(ret.last().unwrap() + 3);
    ret
}

pub fn part1(input: &[usize]) -> usize {
    let (acc1, acc3) = input
        .windows(2)
        .fold((0, 0), |(acc1, acc3), w| match w[1] - w[0] {
            1 => (acc1 + 1, acc3),
            3 => (acc1, acc3 + 1),
            _ => (acc1, acc3),
        });
    acc1 * acc3
}

fn tribonacci(n: usize, seq: &mut Vec<usize>) -> usize {
    for _ in seq.len()..=n {
        let last = seq.len() - 1;
        seq.push(seq[last] + seq[last - 1] + seq[last - 2]);
    }
    seq[n]
}

pub fn part2(input: &[usize]) -> usize {
    let mut acc = 1;
    let mut trib = vec![1, 1, 2];

    let mut len = 0;
    for i in 1..input.len() {
        if input[i] - input[i - 1] == 1 {
            len += 1;
            continue;
        }
        acc *= tribonacci(len, &mut trib);
        len = 0;
    }
    acc
}

#[cfg(test)]
mod test {
    const INPUTSMOL: &'static str = "16
10
15
5
1
11
7
19
6
12
4";

    const INPUTBIG: &'static str = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";

    #[test]
    fn testp1small() {
        let input = super::generate(INPUTSMOL);
        assert_eq!(super::part1(&input), 7 * 5);
    }

    #[test]
    fn testp1big() {
        let input = super::generate(INPUTBIG);
        assert_eq!(super::part1(&input), 22 * 10);
    }

    #[test]
    fn testp2small() {
        let input = super::generate(INPUTSMOL);
        assert_eq!(super::part2(&input), 8);
    }

    #[test]
    fn testp2big() {
        let input = super::generate(INPUTBIG);
        assert_eq!(super::part2(&input), 19208);
    }
}
