use std::fmt;
use std::fs;

#[derive(Clone, Debug, Copy, Eq, PartialEq)]
enum Loc {
    Empty,
    South,
    East,
}

impl fmt::Display for Loc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Loc::Empty => write!(f, "."),
            Loc::South => write!(f, "v"),
            Loc::East => write!(f, ">"),
        }
    }
}

impl Default for Loc {
    fn default() -> Self {
        Loc::Empty
    }
}

impl Loc {
    fn parse(c: char) -> Loc {
        match c {
            '.' => Loc::Empty,
            'v' => Loc::South,
            '>' => Loc::East,
            _ => panic!("Unknown char {}", c),
        }
    }
}

struct Map {
    data: Vec<Vec<Loc>>,
}

impl Map {
    fn rows(&self) -> usize {
        self.data.len()
    }

    fn cols(&self) -> usize {
        self.data[0].len()
    }

    fn print(&self) {
        for row in &self.data {
            for loc in row {
                print!("{}", loc);
            }
            println!();
        }
    }

    fn parse(s: &str) -> Map {
        let data = s
            .lines()
            .map(|line| line.chars().map(|c| Loc::parse(c)).collect())
            .collect();
        Map { data }
    }

    fn parse_file(filename: &str) -> Map {
        Self::parse(&fs::read_to_string(filename).unwrap())
    }

    fn step_east(&self) -> (Map, bool) {
        let mut any_moved = false;
        let mut map = Map {
            data: vec![vec![Loc::Empty; self.cols()]; self.rows()],
        };
        for row in 0..self.rows() {
            for col in 0..self.cols() {
                if self.data[row][col] == Loc::East {
                    let col_1 = (col + 1) % self.cols();
                    let can_move = self.data[row][col_1] == Loc::Empty;
                    let new_col = if can_move { col_1 } else { col };
                    map.data[row][new_col] = Loc::East;
                    any_moved |= can_move;
                } else if self.data[row][col] == Loc::South {
                    map.data[row][col] = Loc::South;
                }
            }
        }
        (map, any_moved)
    }

    fn step_south(&self) -> (Map, bool) {
        let mut any_moved = false;
        let mut map = Map {
            data: vec![vec![Loc::Empty; self.cols()]; self.rows()],
        };
        for row in 0..self.rows() {
            for col in 0..self.cols() {
                if self.data[row][col] == Loc::South {
                    let row_1 = (row + 1) % self.rows();
                    let can_move = self.data[row_1][col] == Loc::Empty;
                    let new_row = if can_move { row_1 } else { row };
                    map.data[new_row][col] = Loc::South;
                    any_moved |= can_move;
                } else if self.data[row][col] == Loc::East {
                    map.data[row][col] = Loc::East;
                }
            }
        }
        (map, any_moved)
    }

    fn step(&self) -> (Map, bool) {
        let map = self;
        let (map, m_e) = map.step_east();
        // println!();
        // println!("after east:");
        // map.print();
        let (map, m_s) = map.step_south();
        (map, m_e || m_s)
    }
}

fn part1(filename: &str, test: bool) {
    println!("{}", filename);
    let mut map = Map::parse_file(filename);
    for i in 0.. {
        // println!();
        // println!("{}", i);
        // map.print();
        let (new_map, moved) = map.step();
        if !moved {
            println!("move stopped at step {}", i + 1);
            break;
        }
        map = new_map;
    }
}

fn main() {
    part1("day25-input-test.txt", true);
    part1("day25-input.txt", false);
}
