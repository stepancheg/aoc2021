use std::collections::HashSet;
use std::iter;

use aoc2021::bingo::BingoBoardWithMarks;
use aoc2021::bingo::BingoFile;
use aoc2021::bingo::BingoMarks;

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
