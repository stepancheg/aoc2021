use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input.txt");
    let lines: Vec<_> = input.lines().collect();
    let mut prev = None;
    let mut more = 0;
    for i in 3..=lines.len() {
        let depth = lines[i - 3..i].iter().map(|line| line.parse::<u32>().unwrap()).sum::<u32>();
        if let Some(prev) = prev {
            if depth > prev {
                more += 1;
            }
        }
        prev = Some(depth);
    }
    println!("{}", more);
}
