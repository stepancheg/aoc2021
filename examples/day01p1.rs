use std::fs;

fn main() {
    let input = fs::read_to_string("day1-input.txt").expect("Failed to read input.txt");
    let lines: Vec<_> = input.lines().collect();
    let mut prev = None;
    let mut more = 0;
    for line in lines {
        let depth = line.parse::<u32>().unwrap();
        if let Some(prev) = prev {
            if depth > prev {
                more += 1;
            }
        }
        prev = Some(depth);
    }
    println!("{}", more);
}
