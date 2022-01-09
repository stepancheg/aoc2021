use std::collections::HashSet;
use std::fs;

#[derive(Debug, Clone)]
pub struct BingoBoard {
    pub rows: [[u32; 5]; 5],
}

impl BingoBoard {
    pub fn find_number(&self, number: u32) -> Vec<(usize, usize)> {
        let mut result = Vec::new();
        for i in 0..5 {
            for j in 0..5 {
                if self.rows[i][j] == number {
                    result.push((i, j));
                }
            }
        }
        result
    }
}

#[derive(Debug, Clone)]
pub struct BingoFile {
    pub numbers: Vec<u32>,
    pub boards: Vec<BingoBoard>,
}

impl BingoFile {
    pub fn parse(filename: &str) -> BingoFile {
        let content = fs::read_to_string(filename).unwrap();
        let lines: Vec<&str> = content.lines().collect();

        let numbers: Vec<u32> = lines[0]
            .split(",")
            .map(|s| s.parse::<u32>().unwrap())
            .collect();
        assert_eq!(
            numbers.len(),
            HashSet::<u32>::from_iter(numbers.iter().copied()).len(),
            "must be unique"
        );

        let mut boards = Vec::new();

        let mut i = 1;
        while i != lines.len() {
            assert_eq!("", lines[i]);
            i += 1;
            let mut rows = [[0; 5]; 5];
            for j in 0..5 {
                let mut k = 0;
                for s in lines[i].split(" ") {
                    if s.is_empty() {
                        continue;
                    }
                    rows[j][k] = s.parse::<u32>().unwrap();
                    k += 1;
                }
                assert_eq!(5, k);
                i += 1;
            }
            boards.push(BingoBoard { rows });
        }

        BingoFile { numbers, boards }
    }
}

#[derive(Debug, Clone, Default)]
pub struct BingoMarks {
    pub rows: [[bool; 5]; 5],
}

impl BingoMarks {
    pub fn is_win(&self) -> bool {
        for i in 0..5 {
            if self.rows[i].iter().all(|&b| b) {
                return true;
            }
        }
        for j in 0..5 {
            if self.rows.iter().map(|row| row[j]).all(|b| b) {
                return true;
            }
        }
        false
    }
}

pub struct BingoBoardWithMarks<'a> {
    pub board: &'a BingoBoard,
    pub marks: &'a mut BingoMarks,
}

impl<'a> BingoBoardWithMarks<'a> {
    pub fn mark(&mut self, number: u32) {
        for (i, j) in self.board.find_number(number) {
            self.marks.rows[i][j] = true;
        }
    }

    pub fn sum_of_all_unmarked_numbers(&self) -> u32 {
        let mut result = 0;
        for i in 0..5 {
            for j in 0..5 {
                if !self.marks.rows[i][j] {
                    result += self.board.rows[i][j];
                }
            }
        }
        result
    }
}
