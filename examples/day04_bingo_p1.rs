use std::iter;

use aoc2021::bingo::BingoBoardWithMarks;
use aoc2021::bingo::BingoFile;
use aoc2021::bingo::BingoMarks;

fn run(filename: &str) {
    println!("run {}", filename);
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

fn main() {
    run("day04-input-test.txt");
    run("day04-input.txt");
}
