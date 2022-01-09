use std::fs;

struct Map {
    rows: Vec<Vec<u8>>,
}

impl Map {
    fn cols(&self) -> usize {
        self.rows[0].len()
    }

    fn parse(filename: &str) -> Map {
        let content = fs::read_to_string(filename).expect("Something went wrong reading the file");
        let rows: Vec<Vec<u8>> = content
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_string().parse::<u8>().unwrap())
                    .collect()
            })
            .collect();
        Map { rows }
    }

    fn is_low_point(&self, r: usize, c: usize) -> bool {
        if r > 0 && self.rows[r - 1][c] <= self.rows[r][c] {
            return false;
        }
        if r < self.rows.len() - 1 && self.rows[r + 1][c] <= self.rows[r][c] {
            return false;
        }
        if c > 0 && self.rows[r][c - 1] <= self.rows[r][c] {
            return false;
        }
        if c < self.rows[r].len() - 1 && self.rows[r][c + 1] <= self.rows[r][c] {
            return false;
        }
        true
    }
}

fn part1(filename: &str) {
    println!("{}", filename);
    let map = Map::parse(filename);
    let mut res = 0;
    for r in 0..map.rows.len() {
        for c in 0..map.cols() {
            if map.is_low_point(r, c) {
                println!("low point at {} {}", r, c);
                res += map.rows[r][c] as u64 + 1;
            }
        }
    }
    println!("{}", res);
}

fn main() {
    println!("Part 1");
    part1("day09-input-test.txt");
    part1("day09-input.txt");
}
