use std::fs;

struct Pattern {
    wires: String,
}

impl Pattern {
    fn is_1(&self) -> bool {
        self.wires.len() == 2
    }

    fn is_4(&self) -> bool {
        self.wires.len() == 4
    }

    fn is_7(&self) -> bool {
        self.wires.len() == 3
    }

    fn is_8(&self) -> bool {
        self.wires.len() == 7
    }
}

struct Line {
    left: Vec<Pattern>,
    right: Vec<Pattern>,
}

impl Line {
    fn parse(s: &str) -> Line {
        let parts: Vec<_> = s.split(" | ").collect();
        assert_eq!(2, parts.len());
        let left = parts[0]
            .split(" ")
            .map(|s| Pattern {
                wires: s.to_owned(),
            })
            .collect();
        let right = parts[1]
            .split(" ")
            .map(|s| Pattern {
                wires: s.to_owned(),
            })
            .collect();
        Line { left, right }
    }
}

fn part1(filename: &str) {
    println!("{}", filename);
    let content = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let lines: Vec<_> = content.lines().map(|s| Line::parse(s)).collect();
    let mut count = 0;
    for line in &lines {
        count += line
            .right
            .iter()
            .filter(|p| p.is_1() || p.is_4() || p.is_7() || p.is_8())
            .count();
    }
    println!("{}", count);
}

fn main() {
    part1("day08-input-test.txt");
    part1("day08-input.txt");
}
