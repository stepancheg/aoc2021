use std::fs;

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let mut pos = 0;
    let mut depth = 0;
    let mut depth_part_2 = 0;
    let mut aim = 0;
    for line in content.lines() {
        let parts: Vec<&str> = line.split(" ").collect();
        let (command, param) = match parts.as_slice() {
            [command, param] => (command, param.parse::<u32>().unwrap()),
            _ => panic!("Invalid input"),
        };
        match *command {
            "up" => {
                depth -= param;
                aim -= param;
            }
            "down" => {
                depth += param;
                aim += param;
            }
            "forward" => {
                pos += param;
                depth_part_2 += aim * param;
            }
            _ => panic!("Invalid input"),
        }
    }
    println!("{}", pos * depth);
    println!("{}", pos * depth_part_2);
}
