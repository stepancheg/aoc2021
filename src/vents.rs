use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Copy)]
pub struct VentCoord {
    pub x: usize,
    pub y: usize,
}

impl VentCoord {
    fn parse(s: &str) -> VentCoord {
        let mut iter = s.split(',');
        let x = iter.next().unwrap().parse::<usize>().unwrap();
        let y = iter.next().unwrap().parse::<usize>().unwrap();
        assert!(iter.next().is_none());
        VentCoord { x, y }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct VentLine {
    pub from: VentCoord,
    pub to: VentCoord,
}

impl VentLine {
    fn parse(s: &str) -> VentLine {
        let parts: Vec<&str> = s.split(" -> ").collect();
        assert_eq!(2, parts.len());
        let from = VentCoord::parse(parts[0]);
        let to = VentCoord::parse(parts[1]);
        VentLine { from, to }
    }

    pub fn is_vert(&self) -> bool {
        self.from.x == self.to.x
    }

    pub fn is_horiz(&self) -> bool {
        self.from.y == self.to.y
    }

    pub fn is_diag(&self) -> bool {
        (self.from.x as i32 - self.to.x as i32).abs()
            == (self.from.y as i32 - self.to.y as i32).abs()
    }
}

pub struct Vents {
    pub lines: Vec<VentLine>,
}

impl Vents {
    pub fn parse(filename: &Path) -> Vents {
        let content = fs::read_to_string(filename).unwrap();
        let lines = content.lines().map(|s| VentLine::parse(s)).collect();
        Vents { lines }
    }

    pub fn take_vert_or_horiz(&self) -> Vents {
        let mut lines = Vec::new();
        for line in &self.lines {
            if line.is_vert() {
                lines.push(line.clone());
            } else if line.is_horiz() {
                lines.push(line.clone());
            }
        }
        Vents { lines }
    }

    pub fn max_xy(&self) -> (usize, usize) {
        let mut max_x = 0;
        let mut max_y = 0;
        for line in &self.lines {
            max_x = max_x.max(line.from.x);
            max_x = max_x.max(line.to.x);
            max_y = max_y.max(line.from.y);
            max_y = max_y.max(line.to.y);
        }
        (max_x, max_y)
    }
}

#[derive(Clone)]
pub struct VentsGrid {
    pub grid: Vec<Vec<u32>>,
}

impl VentsGrid {
    fn put_point(&mut self, x: usize, y: usize) {
        self.grid[y][x] += 1;
    }

    fn between(a: usize, b: usize) -> impl Iterator<Item = usize> {
        if a <= b {
            a..=b
        } else {
            b..=a
        }
    }

    pub fn put_line(&mut self, line: &VentLine) {
        if line.from.x == line.to.x {
            for y in VentsGrid::between(line.from.y, line.to.y) {
                self.put_point(line.from.x, y);
            }
        } else if line.from.y == line.to.y {
            for x in VentsGrid::between(line.from.x, line.to.x) {
                self.put_point(x, line.from.y);
            }
        } else if line.is_diag() {
            let len = (line.from.x as i32 - line.to.x as i32).abs() as usize;
            let xs = if line.to.x > line.from.x { 1 } else { -1 };
            let ys = if line.to.y > line.from.y { 1 } else { -1 };
            for i in 0..=len {
                let x = line.from.x as i32 + (i as i32) * xs;
                let y = line.from.y as i32 + (i as i32) * ys;
                self.put_point(x as usize, y as usize);
            }
        } else {
            panic!("unexpected line type");
        }
    }

    pub fn count_gt_1(&self) -> usize {
        self.grid
            .iter()
            .map(|row| row.iter().filter(|&&v| v > 1).count())
            .sum()
    }
}
