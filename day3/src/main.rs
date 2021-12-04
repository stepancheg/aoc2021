use std::fs;

fn run(filename: &str) {
    let context = fs::read_to_string(filename).unwrap();
    let lines: Vec<&str> = context.lines().collect();
    let bit_count = lines[0].len();
    let mut one_count_by_index = vec![0; bit_count];
    for line in &lines {
        assert_eq!(bit_count, line.len());
        for i in 0..bit_count {
            match line.as_bytes()[i] {
                b'1' => one_count_by_index[i] += 1,
                b'0' => {}
                _ => panic!("Invalid character"),
            }
        }
    }

    let mut gamma = 0;
    let mut epsilon = 0;

    for i in 0..bit_count {
        let the_most_common_is_one = one_count_by_index[i] >= lines.len() / 2;
        let the_most_common = if the_most_common_is_one { 1 } else { 0 };
        let the_least_common = if the_most_common_is_one { 0 } else { 1 };
        let power = 1 << (bit_count - i - 1);
        gamma += the_most_common * power;
        epsilon += the_least_common * power;
    }

    println!(
        "gamma={} epsilon={} power={}",
        gamma,
        epsilon,
        gamma * epsilon
    );
}

fn main() {
    run("input-test.txt");
    run("input.txt");
}
