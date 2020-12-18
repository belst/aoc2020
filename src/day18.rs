/// lots of copy pasting, could probably just create like a Map<op, precedence> and only use genstack2
/// implements [Shunting Yard Algorithm](https://en.wikipedia.org/wiki/Shunting-yard_algorithm)
fn genstack1(input: &[u8]) -> Vec<u8> {
    let mut operatorstack: Vec<u8> = vec![];
    let mut outqueue = vec![];

    for &c in input {
        match c {
            b' ' => {}
            b'0'..=b'9' => outqueue.push(c),
            op @ (b'+' | b'*') => {
                while let Some(n) = operatorstack.pop() {
                    if n == b'(' {
                        operatorstack.push(n);
                        break;
                    }
                    outqueue.push(n);
                }
                operatorstack.push(op);
            }
            b'(' => operatorstack.push(c),
            b')' => {
                while let Some(n) = operatorstack.pop() {
                    if n == b'(' {
                        break;
                    }
                    outqueue.push(n);
                }
            }
            _ => panic!("Invalid Character"),
        }
    }

    outqueue.extend(operatorstack.iter().rev());
    outqueue
}

fn genstack2(input: &[u8]) -> Vec<u8> {
    let mut operatorstack: Vec<u8> = vec![];
    let mut outqueue = vec![];

    for &c in input {
        match c {
            b' ' => {}
            b'0'..=b'9' => outqueue.push(c),
            op @ (b'+' | b'*') => {
                while let Some(n) = operatorstack.pop() {
                    if n == b'(' || n == b'*' {
                        operatorstack.push(n);
                        break;
                    }
                    outqueue.push(n);
                }
                operatorstack.push(op);
            }
            b'(' => operatorstack.push(c),
            b')' => {
                while let Some(n) = operatorstack.pop() {
                    if n == b'(' {
                        break;
                    }
                    outqueue.push(n);
                }
            }
            _ => panic!("Invalid Character"),
        }
    }

    outqueue.extend(operatorstack.iter().rev());
    outqueue
}

/// Evaluates a stack in Reverse Polish Notation
fn evaluate(stack: &[u8]) -> u64 {
    let mut ret = vec![];
    for op in stack {
        match op {
            b'0'..=b'9' => ret.push((op - b'0') as u64),
            b'+' => {
                let r = ret.pop().unwrap();
                let l = ret.pop().unwrap();
                ret.push(r + l);
            }
            b'*' => {
                let r = ret.pop().unwrap();
                let l = ret.pop().unwrap();
                ret.push(r * l);
            }
            _ => panic!("Invalid OP"),
        }
    }
    ret[0]
}

pub fn part1(input: &str) -> u64 {
    input
        .lines()
        .map(|l| {
            let stack = genstack1(&l.as_bytes());
            evaluate(&stack)
        })
        .sum()
}

pub fn part2(input: &str) -> u64 {
    input
        .lines()
        .map(|l| {
            let stack = genstack2(&l.as_bytes());
            evaluate(&stack)
        })
        .sum()
}

#[test]
fn testgenstackp1() {
    let input1 = b"2*3+(4*5)";
    let expected1 = "23*45*+";
    let input2 = b"((2+4*9)*(6+9*8+6)+6)+2+4*2";
    let expected2 = "24+9*69+8*6+*6+2+4+2*";
    assert_eq!(String::from_utf8_lossy(&genstack1(input1)), expected1);
    assert_eq!(String::from_utf8_lossy(&genstack1(input2)), expected2);
}
#[test]
fn testgenstackp2() {
    let input1 = b"1+(2*3)+(4*(5+6))";
    let expected1 = "123*+456+*+";
    assert_eq!(String::from_utf8_lossy(&genstack1(input1)), expected1);
}
