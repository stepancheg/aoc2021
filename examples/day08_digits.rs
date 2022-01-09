use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Copy, Clone, Hash)]
struct UnknownWire {
    value: u8,
}

impl UnknownWire {
    fn parse(c: char) -> UnknownWire {
        assert!(c >= 'a');
        assert!(c <= 'g');
        UnknownWire {
            value: c as u8 - 'a' as u8,
        }
    }
}

#[derive(PartialEq, Eq, Hash)]
struct Pattern {
    wires: Vec<UnknownWire>,
}

impl Pattern {
    fn parse(s: &str) -> Pattern {
        let mut wires: Vec<UnknownWire> = s.chars().map(|c| UnknownWire::parse(c)).collect();
        wires.sort();
        Pattern { wires }
    }

    fn to_set(&self) -> HashSet<UnknownWire> {
        self.wires.iter().copied().collect()
    }

    fn is_superset(&self, other: &Pattern) -> bool {
        self.to_set().is_superset(&other.to_set())
    }
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
        let left: Vec<_> = parts[0].split(" ").map(|s| Pattern::parse(s)).collect();
        let right: Vec<_> = parts[1].split(" ").map(|s| Pattern::parse(s)).collect();
        assert_eq!(10, left.len());
        assert_eq!(4, right.len());
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
    assert!(count == 26 || count == 514);
    println!("{}", count);
}

struct Patterns<'a> {
    patterns: &'a [Pattern],
}

impl<'a> Patterns<'a> {
    fn find_1(&self) -> &'a Pattern {
        let p1: Vec<_> = self.patterns.iter().filter(|p| p.is_1()).collect();
        assert_eq!(1, p1.len());
        p1[0]
    }

    fn find_7(&self) -> &'a Pattern {
        let p7: Vec<_> = self.patterns.iter().filter(|p| p.is_7()).collect();
        assert_eq!(1, p7.len());
        p7[0]
    }

    fn find_4(&self) -> &'a Pattern {
        let p4: Vec<_> = self.patterns.iter().filter(|p| p.is_4()).collect();
        assert_eq!(1, p4.len());
        p4[0]
    }

    fn find_8(&self) -> &'a Pattern {
        let p8: Vec<_> = self.patterns.iter().filter(|p| p.is_8()).collect();
        assert_eq!(1, p8.len());
        p8[0]
    }

    fn find_of_len(&self, len: usize) -> Vec<&'a Pattern> {
        self.patterns
            .iter()
            .filter(|p| p.wires.len() == len)
            .collect()
    }
}

fn decode(patterns: &[Pattern]) -> Vec<&Pattern> {
    assert_eq!(10, patterns.len());
    let patterns = Patterns { patterns };
    let p1 = patterns.find_1();
    let p7 = patterns.find_7();
    let p4 = patterns.find_4();
    let p8 = patterns.find_8();

    let patterns_len_6 = patterns.find_of_len(6);
    assert_eq!(3, patterns_len_6.len());

    let patterns_len_5 = patterns.find_of_len(5);
    assert_eq!(3, patterns_len_5.len());

    let p9: Vec<_> = patterns_len_6
        .iter()
        .copied()
        .filter(|p| p.is_superset(p4))
        .collect();
    assert_eq!(1, p9.len());
    let p9 = p9[0];

    let p0: Vec<_> = patterns_len_6
        .iter()
        .copied()
        .filter(|&p| p.is_superset(p1) && p != p9)
        .collect();
    assert_eq!(1, p0.len());
    let p0 = p0[0];

    let p6: Vec<_> = patterns_len_6
        .iter()
        .copied()
        .filter(|&p| p != p0 && p != p9)
        .collect();
    assert_eq!(1, p6.len());
    let p6 = p6[0];

    let p5: Vec<_> = patterns_len_5
        .iter()
        .copied()
        .filter(|&p| p6.is_superset(p))
        .collect();
    assert_eq!(1, p5.len());
    let p5 = p5[0];

    let p3: Vec<_> = patterns_len_5
        .iter()
        .copied()
        .filter(|&p| p9.is_superset(p) && p != p5)
        .collect();
    assert_eq!(1, p3.len());
    let p3 = p3[0];

    let p2: Vec<_> = patterns_len_5
        .iter()
        .copied()
        .filter(|&p| p != p5 && p != p3)
        .collect();
    assert_eq!(1, p2.len());
    let p2 = p2[0];

    vec![p0, p1, p2, p3, p4, p5, p6, p7, p8, p9]
}

fn part2(filename: &str) {
    println!("{}", filename);
    let content = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let lines: Vec<_> = content.lines().map(|s| Line::parse(s)).collect();
    let mut sum = 0;
    for line in &lines {
        let decoded = decode(&line.left);
        let decoded_index: HashMap<&Pattern, u32> = decoded
            .iter()
            .copied()
            .enumerate()
            .map(|(i, p)| (p, i as u32))
            .collect();
        let digits = line
            .right
            .iter()
            .map(|d| decoded_index[&d])
            .collect::<Vec<_>>();
        let dec_value = digits.iter().fold(0, |acc, &d| acc * 10 + d);
        // println!("{}", dec_value);
        sum += dec_value;
    }
    println!("{}", sum);
}

fn main() {
    println!("Part 1");
    part1("day08-input-test.txt");
    part1("day08-input.txt");

    println!();
    println!("Part 2");
    part2("day08-input-test.txt");
    part2("day08-input.txt");
}
