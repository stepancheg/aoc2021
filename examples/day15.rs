#![feature(map_first_last)]

use std::collections::BTreeMap;
use std::collections::HashMap;
use std::fs;

struct Map {
    rows: Vec<Vec<u8>>,
}

impl Map {
    fn cols(&self) -> usize {
        self.rows[0].len()
    }

    fn parse(filename: &str) -> Map {
        let content = fs::read_to_string(filename).unwrap();
        let mut rows = Vec::new();
        for line in content.lines() {
            let mut row = Vec::new();
            for c in line.chars() {
                row.push(c.to_string().parse().unwrap());
            }
            rows.push(row);
        }
        Map { rows }
    }

    fn mult_5(&self) -> Map {
        let mut rows = vec![vec![0; self.cols() * 5]; self.rows.len() * 5];
        for i in 0..5 {
            for j in 0..5 {
                for r in 0..self.rows.len() {
                    for c in 0..self.cols() {
                        rows[i * self.cols() + r][j * self.rows.len() + c] =
                            (self.rows[r][c] + i as u8 + j as u8 - 1) % 9 + 1;
                    }
                }
            }
        }
        Map { rows }
    }

    fn print(&self) {
        for row in &self.rows {
            for c in row {
                print!("{}", c);
            }
            println!();
        }
    }
}

struct RiskMap2<'a> {
    map: &'a Map,
    min_dist: HashMap<(usize, usize), u64>,
    border: BTreeMap<(usize, usize), u64>,
}

impl<'a> RiskMap2<'a> {
    fn try_walk_to(&mut self, r: usize, c: usize, len: u64) {
        let len_adj = len + self.map.rows[r][c] as u64;
        match self.min_dist.get(&(r, c)) {
            None => {
                self.min_dist.insert((r, c), len_adj);
                self.border.insert((r, c), len_adj);
            }
            Some(min_dist) => {
                if len_adj < *min_dist {
                    self.min_dist.insert((r, c), len_adj);
                    self.border.insert((r, c), len_adj);
                }
            }
        }
    }

    fn search(&mut self) -> u64 {
        self.border.insert((0, 0), 0);
        loop {
            let ((r, c), len) = self.border.pop_first().unwrap();
            if (r, c) == (self.map.rows.len() - 1, self.map.cols() - 1) {
                return len;
            }
            if r > 0 {
                self.try_walk_to(r - 1, c, len);
            }
            if r < self.map.rows.len() - 1 {
                self.try_walk_to(r + 1, c, len);
            }
            if c > 0 {
                self.try_walk_to(r, c - 1, len);
            }
            if c < self.map.cols() - 1 {
                self.try_walk_to(r, c + 1, len);
            }
        }
    }
}

fn run(filename: &str) {
    println!("{}", filename);
    let map = Map::parse(filename);
    let mut risk_map_2 = RiskMap2 {
        map: &map,
        min_dist: HashMap::new(),
        border: BTreeMap::new(),
    };
    println!("{}", risk_map_2.search());
    let map = map.mult_5();
    // map.print();
    let mut risk_map_2 = RiskMap2 {
        map: &map,
        min_dist: HashMap::new(),
        border: BTreeMap::new(),
    };
    println!("{}", risk_map_2.search());
}

fn main() {
    run("day15-input-test.txt");
    run("day15-input.txt");
}
