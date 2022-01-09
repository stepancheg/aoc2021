use std::fs;

fn validate(s: &str) -> Option<u64> {
    let mut stack = Vec::new();
    for b in s.bytes() {
        match b {
            b'{' | b'[' | b'(' | b'<' => stack.push(b),
            b => {
                let pop = stack.pop().unwrap();
                match (b, pop) {
                    (b'}', b'{') | (b']', b'[') | (b')', b'(') | (b'>', b'<') => (),
                    (b')', _) => return Some(3),
                    (b']', _) => return Some(57),
                    (b'}', _) => return Some(1197),
                    (b'>', _) => return Some(25137),
                    _ => panic!(),
                }
            }
        }
    }
    None
}

fn part1(filename: &str) {
    println!("{}", filename);
    let content = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut sum = 0;
    for line in content.lines() {
        if let Some(n) = validate(line) {
            sum += n;
        }
    }
    println!("{}", sum);
}

fn main() {
    part1("day10-input-test.txt");
    part1("day10-input.txt");
}
