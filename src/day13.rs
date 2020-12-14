pub fn generate(input: &str) -> (usize, Vec<Option<usize>>) {
    let timestamp = input.lines().next().unwrap().parse().unwrap();

    let lines = input
        .lines()
        .nth(1)
        .unwrap()
        .split(',')
        .map(|n| n.parse().ok())
        .collect();

    (timestamp, lines)
}

pub fn part1((time, lines): &(usize, Vec<Option<usize>>)) -> usize {
    let (m, l) = lines
        .iter()
        .flatten()
        .map(|l| time % l)
        .zip(lines.iter().flatten())
        .min_by_key(|(m, &l)| l - m)
        .unwrap();

    (l - m) * l
}

/// Computes greatest common denominator
fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

/// Computes x ^ y in mod m
fn pow(x: usize, y: usize, m: usize) -> usize {
    if y == 0 {
        return 1;
    }
    let p = pow(x, y / 2, m) % m;
    let p = (p * p) % m;
    if y % 2 == 0 {
        p
    } else {
        (x * p) % m
    }
}

/// calculates x in a * x = 1 (mod m) using Fermats's little Theorem
/// Only works if m is prime
fn mod_inverse(a: usize, m: usize) -> usize {
    assert_eq!(gcd(a, m), 1, "GCD of {} and {} has to be 1 (coprime)", a, m);

    pow(a, m - 2, m)
}

/// This uses Chinese Remainder Theorem, this works since all Bus IDs are prime
/// Actually this is not stated anywhere but all examples and input are
/// For an input `67, 7, 59, 61` we basically calculate
/// ```hs
/// xs = [
///    (0, 67),
///    (1, 7),
///    (2, 59),
///    (3, 61),
/// ]
/// ```
/// The first column is t + x (its the x)
/// t is then:
/// ```hs
/// -- weird `mod` m in the substraction to not underflow usize
/// system = [t === (m - (b `mod` m)) `mod` m] | (b, m) <- xs]
/// solve using Chinese Remainder Theorem
fn impl_p2(input: &[Option<usize>]) -> usize {
    let input: Vec<_> = input
        .iter()
        .enumerate()
        .filter_map(|(i, &n)| n.map(|n| (i, n)))
        .collect();
    // (x = b (mod n))
    let bs = input.iter().map(|&(n, m)| m - (n % m));
    let n: usize = input.iter().map(|(_, n)| n).product();
    let mods = input.iter().map(|&(_, n)| n);
    let ns = input.iter().map(|(_, ni)| n / ni);
    let xs = ns.clone().zip(mods).map(|(n, m)| mod_inverse(n, m));
    bs.zip(ns.zip(xs))
        .map(|(b, (n, x))| b * n * x)
        .sum::<usize>()
        % n
}

pub fn part2((_, lines): &(usize, Vec<Option<usize>>)) -> usize {
    impl_p2(lines)
}

#[cfg(test)]
mod test {
    const INPUT: &'static str = "939
7,13,x,x,59,x,31,19";

    #[test]
    fn part1() {
        let parsed = super::generate(INPUT);
        assert_eq!(super::part1(&parsed), 295);
    }

    #[test]
    fn part2() {
        let parsedp1 = super::generate(INPUT);
        assert_eq!(super::part2(&parsedp1), 1068781);
        // more
        assert_eq!(super::impl_p2(&[Some(17), None, Some(13), Some(19)]), 3417);
        assert_eq!(
            super::impl_p2(&[Some(67), Some(7), Some(59), Some(61)]),
            754018
        );
        assert_eq!(
            super::impl_p2(&[Some(67), None, Some(7), Some(59), Some(61)]),
            779210
        );
        assert_eq!(
            super::impl_p2(&[Some(67), Some(7), None, Some(59), Some(61)]),
            1261476
        );
        assert_eq!(
            super::impl_p2(&[Some(1789), Some(37), Some(47), Some(1889)]),
            1202161486
        );
    }
}
