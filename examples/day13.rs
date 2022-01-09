use std::collections::HashSet;
use std::fs;

#[derive(Copy, Clone, Debug)]
enum Axis {
    X,
    Y,
}

struct Dots {
    dots: Vec<(u32, u32)>,
}

impl Dots {
    fn print(&self) {
        let max_x = self.dots.iter().map(|(x, _)| *x).max().unwrap();
        let max_y = self.dots.iter().map(|(_, y)| *y).max().unwrap();
        for y in 0..=max_y {
            for x in 0..=max_x {
                if self.dots.contains(&(x, y)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    fn fold(&mut self, axis: Axis, v: u32) {
        for dot in &mut self.dots {
            let x = match axis {
                Axis::X => &mut dot.0,
                Axis::Y => &mut dot.1,
            };
            assert!(*x != v);
            if *x < v {
                continue;
            }
            *x = v - (*x - v);
        }
    }

    fn count_unique(&self) -> usize {
        HashSet::<&(_, _)>::from_iter(&self.dots).len()
    }
}

struct Input {
    dots: Dots,
    folds: Vec<(Axis, u32)>,
}

impl Input {
    fn parse(filename: &str) -> Input {
        let content = fs::read_to_string(filename).expect("Something went wrong reading the file");
        let mut dots = Vec::new();
        let mut folds = Vec::new();
        let mut lines = content.lines();
        loop {
            let line = lines.next().unwrap();
            if line.is_empty() {
                break;
            }
            let parts = line.split(",").collect::<Vec<_>>();
            assert_eq!(2, parts.len());
            let x = parts[0].parse::<u32>().unwrap();
            let y = parts[1].parse::<u32>().unwrap();
            dots.push((x, y));
        }
        let dots = Dots { dots };
        for line in lines {
            let prefix = "fold along ";
            assert!(line.starts_with(prefix));
            let line = &line[prefix.len()..];
            let parts: Vec<_> = line.split("=").collect();
            assert_eq!(2, parts.len());
            let axis = match parts[0] {
                "x" => Axis::X,
                "y" => Axis::Y,
                _ => panic!("Unknown axis"),
            };
            let value = parts[1].parse::<u32>().unwrap();
            folds.push((axis, value));
        }
        Input { dots, folds }
    }
}

fn part1(filename: &str) {
    println!("{}", filename);
    let input = Input::parse(filename);
    let mut dots = input.dots;
    // println!();
    // dots.print();
    for fold in &input.folds {
        dots.fold(fold.0, fold.1);
        println!("dots after fold: {}", dots.count_unique());
        // println!();
        // dots.print();
    }
    println!("{}", dots.count_unique());
    dots.print();
}

fn main() {
    part1("day13-input-test.txt");
    part1("day13-input.txt");
}
