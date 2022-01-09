use std::fmt;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
struct Bits {
    bits: Vec<bool>,
}

impl fmt::Display for Bits {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for bit in self.bits.iter() {
            write!(f, "{}", if *bit { '1' } else { '0' })?;
        }
        Ok(())
    }
}

impl Bits {
    fn len(&self) -> usize {
        self.bits.len()
    }

    fn parse(s: &str) -> Bits {
        let mut bits = Vec::new();
        for c in s.chars() {
            match c {
                '0' => bits.push(false),
                '1' => bits.push(true),
                _ => panic!("invalid bit"),
            }
        }
        Bits { bits }
    }

    fn to_int(&self) -> u32 {
        let mut result = 0;
        for (i, bit) in self.bits.iter().enumerate() {
            if *bit {
                result += 1 << (self.len() - i - 1);
            }
        }
        result
    }
}

#[derive(Debug, Clone)]
struct BitsList {
    bits_list: Vec<Bits>,
}

impl fmt::Display for BitsList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, bits) in self.bits_list.iter().enumerate() {
            if i != 0 {
                write!(f, " ")?;
            }
            write!(f, "{}", bits)?;
        }
        Ok(())
    }
}

impl BitsList {
    fn parse(s: &str) -> BitsList {
        let bits_list: Vec<Bits> = s.lines().map(|s| Bits::parse(s)).collect();
        BitsList { bits_list }
    }

    fn parse_file(path: &Path) -> BitsList {
        let s = fs::read_to_string(path).expect("failed to read file");
        BitsList::parse(&s)
    }

    fn bit_count(&self) -> usize {
        self.bits_list[0].len()
    }

    fn len(&self) -> usize {
        self.bits_list.len()
    }

    fn one_count_at(&self, i: usize) -> usize {
        self.bits_list
            .iter()
            .map(|b| b.bits[i])
            .filter(|b| *b)
            .count()
    }

    fn one_counts(&self) -> Vec<usize> {
        (0..self.bit_count())
            .map(|i| self.one_count_at(i))
            .collect()
    }

    fn most_common_bit_at(&self, i: usize) -> Option<bool> {
        if self.one_count_at(i) * 2 == self.len() {
            None
        } else {
            Some(self.one_count_at(i) > self.len() / 2)
        }
    }

    fn filter(&self, cond: impl Fn(&Bits) -> bool) -> BitsList {
        BitsList {
            bits_list: self.bits_list.iter().filter(|b| cond(b)).cloned().collect(),
        }
    }
}

fn run_part1(filename: &str) {
    let lines = BitsList::parse_file(Path::new(filename));

    let mut gamma = 0;
    let mut epsilon = 0;

    for i in 0..lines.bit_count() {
        let the_most_common_is_one = lines.most_common_bit_at(i).unwrap_or(true);
        let the_most_common = if the_most_common_is_one { 1 } else { 0 };
        let the_least_common = if the_most_common_is_one { 0 } else { 1 };
        let power = 1 << (lines.bit_count() - i - 1);
        gamma += the_most_common * power;
        epsilon += the_least_common * power;
    }

    println!(
        "gamma={} epsilon={} power={}",
        gamma,
        epsilon,
        gamma * epsilon
    );
    assert!(gamma * epsilon == 2003336 || gamma * epsilon == 198);
}

#[derive(Copy, Clone, Debug)]
enum Part2Which {
    MostCommonOxygen,
    LeastCommonCo2,
}

fn run_part2_impl(orig_lines: &BitsList, which: Part2Which) -> u32 {
    println!("{:?}", which);
    let mut lines = orig_lines.clone();
    let mut i = 0;
    while lines.len() > 1 {
        // println!("i={} rem={}", i, lines);
        let the_most_common = lines.most_common_bit_at(i);
        let select_bit = match (the_most_common, which) {
            (Some(the_most_common), Part2Which::MostCommonOxygen) => the_most_common,
            (Some(the_most_common), Part2Which::LeastCommonCo2) => !the_most_common,
            (None, Part2Which::MostCommonOxygen) => true,
            (None, Part2Which::LeastCommonCo2) => false,
        };
        // println!("the_most_common={:?} select_bit={}", the_most_common, select_bit);
        lines = lines.filter(|bits| bits.bits[i] == select_bit);
        i += 1;
    }
    assert_eq!(1, lines.len());
    println!("{} {}", lines.bits_list[0], lines.bits_list[0].to_int());
    lines.bits_list[0].to_int()
}

fn run_part2(filename: &str) {
    println!("{}", filename);
    let lines = BitsList::parse_file(Path::new(filename));
    let ox = run_part2_impl(&lines, Part2Which::MostCommonOxygen);
    let co2 = run_part2_impl(&lines, Part2Which::LeastCommonCo2);
    println!("life_support={}", ox * co2);
}

fn main() {
    println!("part 1");
    run_part1("day03-input-test.txt");
    run_part1("day03-input.txt");

    println!("part 2");
    assert_eq!(23, Bits::parse("10111").to_int());
    run_part2("day03-input-test.txt");
    run_part2("day03-input.txt");
}
