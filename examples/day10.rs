use std::fs;

enum Validate {
    Invalid(u64),
    Complete(u64),
}

fn validate(s: &str) -> Validate {
    let mut stack = Vec::new();
    for b in s.bytes() {
        match b {
            b'{' | b'[' | b'(' | b'<' => stack.push(b),
            b => {
                let pop = stack.pop().unwrap();
                match (b, pop) {
                    (b'}', b'{') | (b']', b'[') | (b')', b'(') | (b'>', b'<') => (),
                    (b')', _) => return Validate::Invalid(3),
                    (b']', _) => return Validate::Invalid(57),
                    (b'}', _) => return Validate::Invalid(1197),
                    (b'>', _) => return Validate::Invalid(25137),
                    _ => panic!(),
                }
            }
        }
    }

    let mut score = 0;

    while let Some(c) = stack.pop() {
        match c {
            b'(' => score = score * 5 + 1,
            b'[' => score = score * 5 + 2,
            b'{' => score = score * 5 + 3,
            b'<' => score = score * 5 + 4,
            _ => unreachable!(),
        }
    }

    Validate::Complete(score)
}

fn part1(filename: &str) {
    println!("{}", filename);
    let content = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut sum = 0;
    for line in content.lines() {
        if let Validate::Invalid(n) = validate(line) {
            sum += n;
        }
    }
    println!("{}", sum);
    assert!(sum == 323691 || sum == 26397)
}

fn part2(filename: &str) {
    println!("{}", filename);
    let content = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut scores = Vec::new();
    for line in content.lines() {
        if let Validate::Complete(score) = validate(line) {
            scores.push(score);
        }
    }
    scores.sort();
    assert_eq!(1, scores.len() % 2);
    println!("middle score: {}", scores[scores.len() / 2]);
}

fn main() {
    println!("Part 1");
    part1("day10-input-test.txt");
    part1("day10-input.txt");

    println!();
    println!("Part 2");
    part2("day10-input-test.txt");
    part2("day10-input.txt");
}
