use std::collections::HashSet;
use std::fs;
use std::iter;

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

fn first_to_win(filename: &str) {
    println!("run first to win {}", filename);
    let file = BingoFile::parse(filename);
    let mut marks = iter::repeat(BingoMarks::default())
        .take(file.boards.len())
        .collect::<Vec<_>>();
    let mut boards_with_marks = file
        .boards
        .iter()
        .zip(marks.iter_mut())
        .map(|(board, marks)| BingoBoardWithMarks { board, marks })
        .collect::<Vec<_>>();

    for &n in &file.numbers {
        for board in &mut boards_with_marks {
            board.mark(n);
        }
        let mut win = false;
        for board in &boards_with_marks {
            if board.marks.is_win() {
                assert!(!win, "most than one board wins");
                println!("win {}", n);
                println!(
                    "sum of all unmarked numbers: {}",
                    board.sum_of_all_unmarked_numbers()
                );
                println!("{}", n * board.sum_of_all_unmarked_numbers());
                win = true;
            }
        }
        if win {
            return;
        }
    }
    panic!("no win");
}

fn last_to_win(filename: &str) {
    println!("run last to win {}", filename);
    let file = BingoFile::parse(filename);
    let mut marks = iter::repeat(BingoMarks::default())
        .take(file.boards.len())
        .collect::<Vec<_>>();
    let mut boards_with_marks = file
        .boards
        .iter()
        .zip(marks.iter_mut())
        .map(|(board, marks)| BingoBoardWithMarks { board, marks })
        .collect::<Vec<_>>();

    let mut remaining_boards = HashSet::<usize>::from_iter(0..file.boards.len());
    let mut last_to_win = None;

    for &n in &file.numbers {
        for (i, board) in boards_with_marks.iter_mut().enumerate() {
            board.mark(n);
            if board.marks.is_win() {
                remaining_boards.remove(&i);
            }
        }

        if remaining_boards.len() == 1 {
            last_to_win = Some(*remaining_boards.iter().next().unwrap());
        }

        if remaining_boards.len() == 0 {
            let last_to_win = last_to_win.unwrap();
            println!("last to win board: {}", last_to_win);
            println!("n: {}", n);
            let board = &boards_with_marks[last_to_win];
            println!(
                "sum of all unmarked numbers: {}",
                board.sum_of_all_unmarked_numbers()
            );
            println!("{}", n * board.sum_of_all_unmarked_numbers());
            return;
        }
    }

    panic!();
}

fn main() {
    println!("run first to win");
    first_to_win("day04-input-test.txt");
    first_to_win("day04-input.txt");

    println!();
    println!("run last to win");
    last_to_win("day04-input-test.txt");
    last_to_win("day04-input.txt");
}
