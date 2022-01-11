#![feature(map_first_last)]

use std::collections::BTreeMap;
use std::fs;

#[derive(Copy, Clone, Debug)]
struct Range {
    from: i64,
    to_incl: i64,
}

impl Range {
    fn parse(mut s: &str, axis: char) -> Range {
        assert!(s.starts_with(&axis.to_string()));
        s = &s[axis.len_utf8()..];
        assert!(s.starts_with("="));
        s = &s["=".len()..];
        let parts = s.split("..").collect::<Vec<_>>();
        assert_eq!(2, parts.len());
        let from = parts[0].parse::<i64>().unwrap();
        let to_incl = parts[1].parse::<i64>().unwrap();
        assert!(from <= to_incl);
        Range { from, to_incl }
    }
}

#[derive(Debug)]
struct Command {
    on: bool,
    coords: [Range; 3],
}

impl Command {
    fn parse(mut s: &str) -> Command {
        let on;
        if s.starts_with("on ") {
            on = true;
            s = &s["on ".len()..];
        } else if s.starts_with("off ") {
            on = false;
            s = &s["off ".len()..];
        } else {
            panic!("invalid command: {}", s);
        }
        let parts = s.split(",").collect::<Vec<_>>();
        assert_eq!(3, parts.len());
        let x = Range::parse(parts[0], 'x');
        let y = Range::parse(parts[1], 'y');
        let z = Range::parse(parts[2], 'z');
        let coords = [x, y, z];
        Command { on, coords }
    }
}

struct Input {
    commands: Vec<Command>,
}

impl Input {
    fn parse(filename: &str) -> Input {
        let content = fs::read_to_string(filename).expect("Something went wrong reading the file");
        let commands = content.lines().map(|line| Command::parse(line)).collect();
        Input { commands }
    }
}

#[derive(Default, Clone)]
struct CuboidsChild {
    eq: Cuboids,
    gt: Cuboids,
}

#[derive(Default, Clone)]
struct Cuboids {
    on: bool,
    children: BTreeMap<i64, CuboidsChild>,
}

impl Cuboids {
    fn split(&mut self, at: i64) {
        let range = self.children.range_mut(..=at);
        match range.last() {
            None => {
                self.children.insert(at, CuboidsChild::default());
            }
            Some(last) => {
                if *last.0 == at {
                    return;
                }
                let child = CuboidsChild {
                    eq: last.1.gt.clone(),
                    gt: last.1.gt.clone(),
                };
                self.children.insert(at, child);
            }
        }
    }

    fn update(&mut self, key: &[Range], on: bool) {
        match key.split_first() {
            None => self.on = on,
            Some((first, rem)) => {
                self.split(first.from);
                self.split(first.to_incl);

                assert!(self.children.contains_key(&first.from));
                assert!(self.children.contains_key(&first.to_incl));

                let range = self.children.range_mut(first.from..=first.to_incl);
                for (&k, child) in range {
                    child.eq.update(rem, on);
                    if k != first.to_incl {
                        child.gt.update(rem, on);
                    }
                }
            }
        }
    }

    fn count_on(&self, ranges: &[Range]) -> u64 {
        match ranges.split_first() {
            None => self.on as u64,
            Some((first, rem)) => {
                assert!(self.children.contains_key(&first.from));
                assert!(self.children.contains_key(&first.to_incl));
                let range = self.children.range(first.from..=first.to_incl);

                let mut sum = 0;

                let mut range_again = range.clone();
                range_again.next().unwrap();
                for (&k_s, child) in range {
                    let k_e = range_again.next();
                    sum += child.eq.count_on(rem);
                    match k_e {
                        None => {
                            assert!(k_s == first.to_incl);
                        }
                        Some(k_e) => {
                            let k_e = *k_e.0;
                            assert!(k_e > k_s);
                            assert!(k_s < first.to_incl);
                            sum += child.gt.count_on(rem) * (k_e - k_s - 1) as u64;
                        }
                    }
                }
                sum
            }
        }
    }
}

fn part1(filename: &str, test: u32) {
    println!("{}", filename);
    let input = Input::parse(filename);
    let mut cuboids = Cuboids::default();
    let fifty = [
        Range {
            from: -50,
            to_incl: 50,
        },
        Range {
            from: -50,
            to_incl: 50,
        },
        Range {
            from: -50,
            to_incl: 50,
        },
    ];
    cuboids.update(&fifty, false);
    for (i, command) in input.commands.iter().enumerate() {
        // println!("{:?}", command);
        cuboids.update(&command.coords, command.on);
        if test == 0 {
            if i == 0 {
                assert_eq!(27, cuboids.count_on(&fifty));
            } else if i == 1 {
                assert_eq!(27 + 19, cuboids.count_on(&fifty));
            }
        }
    }

    let count_50 = cuboids.count_on(&fifty);
    println!("count 50: {}", count_50);
    assert!(count_50 == 39 || count_50 == 590784 || count_50 == 503864);
}

fn part2(filename: &str) {
    println!("{}", filename);
    let input = Input::parse(filename);
    let mut cuboids = Cuboids::default();

    let millions = [
        Range {
            from: -1000000,
            to_incl: 1000000,
        },
        Range {
            from: -1000000,
            to_incl: 1000000,
        },
        Range {
            from: -1000000,
            to_incl: 1000000,
        },
    ];

    cuboids.update(&millions, false);

    for command in &input.commands {
        cuboids.update(&command.coords, command.on);
    }

    let count_1m = cuboids.count_on(&millions);
    println!("count 1m: {}", count_1m);
    assert!(count_1m == 2758514936282235 || count_1m == 1255547543528356);
}

fn main() {
    println!("Part 1");
    part1("day22-input-part1-test-1.txt", 0);
    part1("day22-input-part1-test-2.txt", 1);
    part1("day22-input.txt", 2);

    println!();
    println!("Part 2");
    part2("day22-input-part2-test.txt");
    part2("day22-input.txt");
}
