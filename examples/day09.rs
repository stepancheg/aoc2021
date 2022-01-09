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
                res += map.rows[r][c] as u64 + 1;
            }
        }
    }
    println!("{}", res);
    assert!(res == 15 || res == 588);
}

fn part2(filename: &str) {
    println!("{}", filename);
    let map = Map::parse(filename);

    let mut basin_ids_by_cell = HashMap::new();

    let mut basins = Vec::new();

    fn find_basin_id(
        map: &Map,
        r: usize,
        c: usize,
        basin_id: u32,
        basins: &mut HashMap<(usize, usize), u32>,
        current_basin: &mut u32,
    ) {
        assert_ne!(map.rows[r][c], 9);

        if let Some(..) = basins.get(&(r, c)) {
            return;
        }

        basins.insert((r, c), basin_id);
        *current_basin += 1;

        if r > 0 && map.rows[r - 1][c] != 9 {
            find_basin_id(map, r - 1, c, basin_id, basins, current_basin);
        }
        if r < map.rows.len() - 1 && map.rows[r + 1][c] != 9 {
            find_basin_id(map, r + 1, c, basin_id, basins, current_basin);
        }
        if c > 0 && map.rows[r][c - 1] != 9 {
            find_basin_id(map, r, c - 1, basin_id, basins, current_basin);
        }
        if c < map.rows[r].len() - 1 && map.rows[r][c + 1] != 9 {
            find_basin_id(map, r, c + 1, basin_id, basins, current_basin);
        }
    }

    for r in 0..map.rows.len() {
        for c in 0..map.cols() {
            if let Some(..) = basin_ids_by_cell.get(&(r, c)) {
                continue;
            }
            if map.rows[r][c] == 9 {
                continue;
            }
            let basin_id = basins.len() as u32;
            basins.push(0);
            let current_basin = basins.last_mut().unwrap();
            find_basin_id(&map, r, c, basin_id, &mut basin_ids_by_cell, current_basin);
        }
    }
    println!("n basins: {}", basins.len());
    basins.sort_by_key(|&x| u32::MAX - x);
    println!("{:?}", basins);
    basins.drain(3..);
    println!("{}", basins.iter().copied().product::<u32>());
}

fn main() {
    println!("Part 1");
    part1("day09-input-test.txt");
    part1("day09-input.txt");

    println!();
    println!("Part 2");
    part2("day09-input-test.txt");
    part2("day09-input.txt");
}
