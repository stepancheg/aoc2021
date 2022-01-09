use std::path::Path;

use aoc2021::vents::Vents;
use aoc2021::vents::VentsGrid;

fn run(filename: &str) {
    println!("{}", filename);
    let vents = Vents::parse(Path::new(filename));
    let (max_x, max_y) = vents.max_xy();
    let mut grid_hv = VentsGrid {
        grid: vec![vec![0; max_x + 1]; max_y + 1],
    };
    let mut grid_hvd = grid_hv.clone();
    for line in &vents.lines {
        if line.is_horiz() || line.is_vert() {
            grid_hv.put_line(line);
            grid_hvd.put_line(line);
        }
        if line.is_diag() {
            grid_hvd.put_line(line);
        }
    }
    println!("hv: {}", grid_hv.count_gt_1());
    println!("hvd: {}", grid_hvd.count_gt_1());
}

fn main() {
    run("day05-input-test.txt");
    run("day05-input.txt");
}
