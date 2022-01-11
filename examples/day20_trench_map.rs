use std::fs;

struct Alg {
    bits: [bool; 512],
}

impl Alg {
    fn parse(input: &str) -> Alg {
        let mut bits = [false; 512];
        assert_eq!(bits.len(), input.len());
        for (i, c) in input.chars().enumerate() {
            bits[i] = match c {
                '.' => false,
                '#' => true,
                _ => panic!("invalid input"),
            };
        }
        Alg { bits }
    }
}

struct Map {
    default: bool,
    rows: Vec<Vec<bool>>,
}

impl Map {
    fn rows(&self) -> usize {
        self.rows.len()
    }

    fn cols(&self) -> usize {
        self.rows[0].len()
    }

    fn print(&self) {
        for row in &self.rows {
            for c in row {
                print!("{}", if *c { '#' } else { '.' });
            }
            println!();
        }
    }

    fn at(&self, row: isize, col: isize) -> bool {
        if row < 0 || row >= self.rows() as isize {
            return self.default;
        }
        if col < 0 || col >= self.cols() as isize {
            return self.default;
        }
        self.rows[row as usize][col as usize]
    }

    fn nine_at(&self, r: isize, c: isize) -> u32 {
        let mut sum = 0;
        for ro in -1..=1 {
            for co in -1..=1 {
                // println!("{} {} -> {}", r + ro, c + co, self.at(r + ro, c + co));
                sum = (sum << 1) + (self.at(r + ro, c + co) as u32);
            }
        }
        sum
    }

    fn apply(&self, alg: &Alg) -> Map {
        let mut rows = vec![vec![false; self.cols() + 2]; self.rows() + 2];
        for r in 0..rows.len() {
            for c in 0..rows[r].len() {
                let or = r as isize - 1;
                let oc = c as isize - 1;
                let nine = self.nine_at(or, oc);
                rows[r][c] = alg.bits[nine as usize];
            }
        }
        let default = alg.bits[if self.default { 511 } else { 0 }];
        Map { rows, default }
    }

    fn lit_pixels(&self) -> usize {
        assert!(!self.default);
        self.rows
            .iter()
            .map(|row| row.iter().filter(|&&c| c).count())
            .sum()
    }
}

struct Input {
    alg: Alg,
    map: Map,
}

impl Input {
    fn parse(filename: &str) -> Input {
        let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
        let mut lines = contents.lines();
        let alg = Alg::parse(lines.next().unwrap());
        assert_eq!("", lines.next().unwrap());

        let mut rows: Vec<Vec<bool>> = Vec::new();
        while let Some(line) = lines.next() {
            if let Some(first_row) = rows.first() {
                assert_eq!(first_row.len(), line.len());
            }
            let row = line
                .chars()
                .map(|c| match c {
                    '.' => false,
                    '#' => true,
                    _ => panic!("invalid input"),
                })
                .collect();
            rows.push(row);
        }
        let default = false;
        let map = Map { rows, default };
        Input { alg, map }
    }
}

fn test_input(map: &Map, alg: &Alg) {
    assert_eq!(34, map.nine_at(2, 2));
    assert!(alg.bits[34]);
}

fn run(filename: &str, test: bool) {
    println!("{}", filename);
    let input = Input::parse(filename);

    if test {
        test_input(&input.map, &input.alg);
    }

    let mut map = input.map;
    println!("init map:");
    map.print();

    for i in 0..50 {
        map = map.apply(&input.alg);
        println!("after step {}:", i);
        map.print();

        if i + 1 == 2 || i + 1 == 50 {
            println!();
            println!("lit pixels: {}", map.lit_pixels());
            if i + 1 == 2 {
                assert!(map.lit_pixels() == 35 || map.lit_pixels() == 5379);
            } else if i + 1 == 50 {
                assert!(map.lit_pixels() == 3351 || map.lit_pixels() == 17917);
            } else {
                unreachable!();
            }
        }
    }
}

fn main() {
    run("day20-input-test.txt", true);
    println!();
    run("day20-input.txt", false);
}
