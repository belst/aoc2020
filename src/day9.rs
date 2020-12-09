pub fn generate(input: &str) -> Vec<usize> {
    input.lines().map(str::parse).flat_map(Result::ok).collect()
}

fn impl_p1(input: &[usize], window_size: usize) -> usize {
    input
        .windows(window_size + 1)
        .find(|&w| {
            for i in 0..w.len() - 1 {
                for j in (i + 1)..w.len() - 1 {
                    if w[w.len() - 1] == w[i] + w[j] {
                        return false;
                    }
                }
            }
            true
        })
        .map(|w| w[w.len() - 1])
        .unwrap()
}

pub fn part1(input: &[usize]) -> usize {
    impl_p1(input, 25)
}

fn impl_p2(input: &[usize], number: usize) -> usize {
    for w in 2..input.len() {
        if let Some(res) = input
            .windows(w)
            .find(|&w| w.iter().sum::<usize>() == number)
        {
            let (min, max) = res.iter().fold((usize::MAX, usize::MIN), |(min, max), &c| {
                if c < min {
                    (c, max)
                } else if max < c {
                    (min, c)
                } else {
                    (min, max)
                }
            });
            return min + max;
        }
    }
    panic!("Invalid Input, Can't find solution")
}

pub fn part2(input: &[usize]) -> usize {
    impl_p2(input, 90433990)
}

#[test]
fn testp1() {
    const INPUT: &'static str = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";
    let list = generate(INPUT);
    assert_eq!(impl_p1(&list, 5), 127);
}

#[test]
fn testp2() {
    const INPUT: &'static str = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";
    let list = generate(INPUT);
    assert_eq!(impl_p2(&list, 127), 62);
}
