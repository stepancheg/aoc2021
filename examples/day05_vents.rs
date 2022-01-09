use std::path::Path;

use aoc2021::vents::Vents;
use aoc2021::vents::VentsGrid;

fn run(filename: &str) {
    println!("{}", filename);
    let vents = Vents::parse(Path::new(filename));
    let vents = vents.take_vert_or_horiz();
    let (max_x, max_y) = vents.max_xy();
    let mut grid = VentsGrid {
        grid: vec![vec![0; max_x + 1]; max_y + 1],
    };
    for line in &vents.lines {
        grid.put_line(line);
    }
    println!("{}", grid.count_gt_1());
}

fn main() {
    run("day05-input-test.txt");
    run("day05-input.txt");
}
