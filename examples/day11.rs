use std::collections::HashSet;
use std::fs;

struct Octopuses {
    rows: Vec<Vec<u32>>,
}

impl Octopuses {
    fn print(&self) {
        for row in &self.rows {
            for cell in row {
                print!("{}", cell);
            }
            println!();
        }
    }

    fn parse(filename: &str) -> Octopuses {
        let content = fs::read_to_string(filename).expect("Something went wrong reading the file");
        let mut rows = Vec::new();
        for line in content.lines() {
            let row: Vec<u32> = line
                .chars()
                .map(|x| x.to_string().parse().unwrap())
                .collect();
            rows.push(row);
        }
        Octopuses { rows }
    }

    fn inc1(&mut self) {
        for row in &mut self.rows {
            for c in row {
                *c += 1;
            }
        }
    }

    fn reset0(&mut self) {
        for row in &mut self.rows {
            for c in row {
                if *c > 9 {
                    *c = 0;
                }
            }
        }
    }

    fn check_flash(
        &mut self,
        r: usize,
        c: usize,
        flashes: &mut u64,
        flashed: &mut HashSet<(usize, usize)>,
    ) {
        if flashed.contains(&(r, c)) {
            return;
        }

        if self.rows[r][c] <= 9 {
            return;
        }

        *flashes += 1;
        flashed.insert((r, c));
        if r > 0 {
            self.rows[r - 1][c] += 1;
            self.check_flash(r - 1, c, flashes, flashed);
        }
        if r < self.rows.len() - 1 {
            self.rows[r + 1][c] += 1;
            self.check_flash(r + 1, c, flashes, flashed);
        }
        if c > 0 {
            self.rows[r][c - 1] += 1;
            self.check_flash(r, c - 1, flashes, flashed);
        }
        if c < self.rows[r].len() - 1 {
            self.rows[r][c + 1] += 1;
            self.check_flash(r, c + 1, flashes, flashed);
        }
        if r > 0 && c > 0 {
            self.rows[r - 1][c - 1] += 1;
            self.check_flash(r - 1, c - 1, flashes, flashed);
        }
        if r > 0 && c < self.rows[r].len() - 1 {
            self.rows[r - 1][c + 1] += 1;
            self.check_flash(r - 1, c + 1, flashes, flashed);
        }
        if r < self.rows.len() - 1 && c > 0 {
            self.rows[r + 1][c - 1] += 1;
            self.check_flash(r + 1, c - 1, flashes, flashed);
        }
        if r < self.rows.len() - 1 && c < self.rows[r].len() - 1 {
            self.rows[r + 1][c + 1] += 1;
            self.check_flash(r + 1, c + 1, flashes, flashed);
        }
    }

    fn count(&self) -> usize {
        self.rows.len() * self.rows[0].len()
    }
}

fn run(filename: &str) {
    println!("{}", filename);
    let mut oct = Octopuses::parse(filename);
    let mut total_flashes = 0;
    for step in 0.. {
        // println!("Step {}", step);
        // oct.print();

        let mut flashes = 0;
        let mut flashed = HashSet::new();
        oct.inc1();
        for r in 0..oct.rows.len() {
            for c in 0..oct.rows[r].len() {
                oct.check_flash(r, c, &mut flashes, &mut flashed);
            }
        }
        oct.reset0();
        total_flashes += flashes;

        if step == 99 {
            println!("{}", total_flashes);
            assert!(total_flashes == 1656 || total_flashes == 1562);
        }

        if flashes == oct.count() as u64 {
            println!("All flashed on step {}", step + 1);
            break;
        }
    }
}

fn main() {
    run("day11-input-test.txt");
    run("day11-input.txt");
}
