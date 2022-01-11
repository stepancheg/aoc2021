use std::collections::HashSet;
use std::fmt;
use std::fs;
use std::ops::Add;
use std::ops::Mul;
use std::ops::Neg;
use std::ops::Sub;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Axis {
    X,
    Y,
    Z,
}

impl Default for Axis {
    fn default() -> Self {
        Axis::X
    }
}

impl Axis {
    const ALL: [Axis; 3] = [Axis::X, Axis::Y, Axis::Z];

    const fn id(&self) -> Vector {
        match self {
            Axis::X => Vector::new(1, 0, 0),
            Axis::Y => Vector::new(0, 1, 0),
            Axis::Z => Vector::new(0, 0, 1),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Angle90 {
    A0,
    A90,
    A180,
    A270,
}

impl Default for Angle90 {
    fn default() -> Self {
        Angle90::A0
    }
}

impl Add for Angle90 {
    type Output = Angle90;

    fn add(self, rhs: Angle90) -> Angle90 {
        let sum = (self as u32 + rhs as u32) % 4;
        match sum {
            0 => Angle90::A0,
            1 => Angle90::A90,
            2 => Angle90::A180,
            3 => Angle90::A270,
            _ => unreachable!(),
        }
    }
}

impl Neg for Angle90 {
    type Output = Angle90;

    fn neg(self) -> Self::Output {
        match self {
            Angle90::A0 => Angle90::A0,
            Angle90::A90 => Angle90::A270,
            Angle90::A180 => Angle90::A180,
            Angle90::A270 => Angle90::A90,
        }
    }
}

impl Angle90 {
    const ALL: [Angle90; 4] = [Angle90::A0, Angle90::A90, Angle90::A180, Angle90::A270];

    fn sin(&self) -> i64 {
        match self {
            Angle90::A0 => 0,
            Angle90::A90 => 1,
            Angle90::A180 => 0,
            Angle90::A270 => -1,
        }
    }

    fn cos(&self) -> i64 {
        match self {
            Angle90::A0 => 1,
            Angle90::A90 => 0,
            Angle90::A180 => -1,
            Angle90::A270 => 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
struct Matrix {
    data: [[i64; 3]; 3],
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Default, Hash)]
struct Vector {
    data: [i64; 3],
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{},{}", self.x(), self.y(), self.z())
    }
}

impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Vector {
        Vector {
            data: [-self.x(), -self.y(), -self.z()],
        }
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Vector {
        Vector {
            data: [self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z()],
        }
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Vector {
        Vector {
            data: [self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z()],
        }
    }
}

impl Vector {
    const ZERO: Vector = Vector { data: [0, 0, 0] };

    fn x(&self) -> i64 {
        self.data[0]
    }

    fn y(&self) -> i64 {
        self.data[1]
    }

    fn z(&self) -> i64 {
        self.data[2]
    }

    const fn new(x: i64, y: i64, z: i64) -> Vector {
        Vector { data: [x, y, z] }
    }

    fn rot_z(&self, angle: Angle90) -> Vector {
        Matrix::rot_z(angle).mul(*self)
    }

    fn rot_x(&self, angle: Angle90) -> Vector {
        Matrix::rot_x(angle).mul(*self)
    }

    fn rot_y(&self, angle: Angle90) -> Vector {
        Matrix::rot_y(angle).mul(*self)
    }

    fn rot(&self, axis: Axis, angle: Angle90) -> Vector {
        match axis {
            Axis::X => self.rot_x(angle),
            Axis::Y => self.rot_y(angle),
            Axis::Z => self.rot_z(angle),
        }
    }

    fn dot_product(&self, rhs: Vector) -> i64 {
        self.data.iter().zip(rhs.data).map(|(a, b)| a * b).sum()
    }

    fn parse(s: &str) -> Vector {
        let mut parts = s.split(",");
        let x = parts.next().unwrap().parse::<i64>().unwrap();
        let y = parts.next().unwrap().parse::<i64>().unwrap();
        let z = parts.next().unwrap().parse::<i64>().unwrap();
        Vector::new(x, y, z)
    }

    fn test() {
        assert_eq!(
            Vector::new(-2, 1, 3),
            Vector::new(1, 2, 3).rot_z(Angle90::A90)
        );
    }
}

impl Mul<i64> for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: i64) -> Matrix {
        Matrix {
            data: [
                [
                    self.data[0][0] * rhs,
                    self.data[0][1] * rhs,
                    self.data[0][2] * rhs,
                ],
                [
                    self.data[1][0] * rhs,
                    self.data[1][1] * rhs,
                    self.data[1][2] * rhs,
                ],
                [
                    self.data[2][0] * rhs,
                    self.data[2][1] * rhs,
                    self.data[2][2] * rhs,
                ],
            ],
        }
    }
}

impl Mul<Vector> for Matrix {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Vector {
        Vector {
            data: [
                self.data[0][0] * rhs.data[0]
                    + self.data[0][1] * rhs.data[1]
                    + self.data[0][2] * rhs.data[2],
                self.data[1][0] * rhs.data[0]
                    + self.data[1][1] * rhs.data[1]
                    + self.data[1][2] * rhs.data[2],
                self.data[2][0] * rhs.data[0]
                    + self.data[2][1] * rhs.data[1]
                    + self.data[2][2] * rhs.data[2],
            ],
        }
    }
}

impl Mul<Matrix> for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Matrix) -> Matrix {
        let mut result = Matrix::default();
        for i in 0..3 {
            for j in 0..3 {
                result.data[i][j] = 0
                    + self.data[i][0] * rhs.data[0][j]
                    + self.data[i][1] * rhs.data[1][j]
                    + self.data[i][2] * rhs.data[2][j];
            }
        }
        result
    }
}

impl Matrix {
    const ID: Matrix = Matrix {
        data: [[1, 0, 0], [0, 1, 0], [0, 0, 1]],
    };

    fn det(&self) -> i64 {
        self.data[0][0] * self.data[1][1] * self.data[2][2]
            + self.data[0][1] * self.data[1][2] * self.data[2][0]
            + self.data[0][2] * self.data[1][0] * self.data[2][1]
            - self.data[0][2] * self.data[1][1] * self.data[2][0]
            - self.data[0][1] * self.data[1][0] * self.data[2][2]
            - self.data[0][0] * self.data[1][2] * self.data[2][1]
    }

    fn new_det_1(data: [[i64; 3]; 3]) -> Matrix {
        let matrix = Matrix { data };
        assert_eq!(1, matrix.det());
        matrix
    }

    fn rot_x(angle: Angle90) -> Matrix {
        Matrix::new_det_1([
            [1, 0, 0],
            [0, angle.cos(), -angle.sin()],
            [0, angle.sin(), angle.cos()],
        ])
    }

    fn rot_y(angle: Angle90) -> Matrix {
        Matrix::new_det_1([
            [angle.cos(), 0, angle.sin()],
            [0, 1, 0],
            [-angle.sin(), 0, angle.cos()],
        ])
    }

    fn rot_z(angle: Angle90) -> Matrix {
        Matrix::new_det_1([
            [angle.cos(), -angle.sin(), 0],
            [angle.sin(), angle.cos(), 0],
            [0, 0, 1],
        ])
    }

    fn rot_x_to(axis: Axis) -> Matrix {
        match axis {
            Axis::X => Matrix::ID,
            Axis::Y => Matrix::rot_z(Angle90::A90),
            Axis::Z => Matrix::rot_y(Angle90::A270),
        }
    }

    fn rot_to_x(axis: Axis) -> Matrix {
        match axis {
            Axis::X => Matrix::ID,
            Axis::Y => Matrix::rot_z(Angle90::A270),
            Axis::Z => Matrix::rot_y(Angle90::A90),
        }
    }

    fn all_24() -> [Matrix; 24] {
        Rot::ALL_24.map(|rot| rot.matrix())
    }

    fn test() {
        assert_eq!(24, HashSet::<Matrix>::from_iter(Self::all_24()).len());

        for axis in Axis::ALL {
            let rot_x = Matrix::rot_x_to(axis);
            let rot_to_x = Matrix::rot_to_x(axis);
            assert_eq!(Matrix::ID, rot_x * rot_to_x);
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct VectorList {
    data: Vec<Vector>,
}

impl VectorList {
    fn print(&self) {
        for vector in &self.data {
            println!("{}", vector);
        }
    }

    fn shift(&self, offset: Vector) -> VectorList {
        VectorList {
            data: self.data.iter().map(|vector| *vector + offset).collect(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct Rot {
    x_axis_rot: Angle90,
    axis: Axis,
    forward: bool,
}

impl Mul<Vector> for Rot {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Vector {
        self.matrix() * rhs
    }
}

impl Mul<Rot> for Rot {
    type Output = Rot;

    fn mul(self, rhs: Rot) -> Rot {
        Rot::from_matrix(&(self.matrix() * rhs.matrix()))
    }
}

impl Rot {
    fn from_matrix(matrix: &Matrix) -> Rot {
        for rot in Rot::ALL_24 {
            if matrix == &rot.matrix() {
                return rot;
            }
        }
        panic!()
    }

    fn matrix(&self) -> Matrix {
        let forward = match self.forward {
            true => Matrix::ID,
            false => Matrix::rot_y(Angle90::A180),
        };

        Matrix::rot_x(self.x_axis_rot) * forward * Matrix::rot_x_to(self.axis)
    }

    fn matrix_rev(&self) -> Matrix {
        let forward = match self.forward {
            true => Matrix::ID,
            false => Matrix::rot_y(Angle90::A180),
        };

        Matrix::rot_to_x(self.axis) * forward * Matrix::rot_x(-self.x_axis_rot)
    }

    const ALL_24: [Rot; 24] = [
        Rot {
            x_axis_rot: Angle90::A0,
            axis: Axis::X,
            forward: true,
        },
        Rot {
            x_axis_rot: Angle90::A0,
            axis: Axis::X,
            forward: false,
        },
        Rot {
            x_axis_rot: Angle90::A90,
            axis: Axis::X,
            forward: true,
        },
        Rot {
            x_axis_rot: Angle90::A90,
            axis: Axis::X,
            forward: false,
        },
        Rot {
            x_axis_rot: Angle90::A180,
            axis: Axis::X,
            forward: true,
        },
        Rot {
            x_axis_rot: Angle90::A180,
            axis: Axis::X,
            forward: false,
        },
        Rot {
            x_axis_rot: Angle90::A270,
            axis: Axis::X,
            forward: true,
        },
        Rot {
            x_axis_rot: Angle90::A270,
            axis: Axis::X,
            forward: false,
        },
        Rot {
            x_axis_rot: Angle90::A0,
            axis: Axis::Y,
            forward: true,
        },
        Rot {
            x_axis_rot: Angle90::A0,
            axis: Axis::Y,
            forward: false,
        },
        Rot {
            x_axis_rot: Angle90::A90,
            axis: Axis::Y,
            forward: true,
        },
        Rot {
            x_axis_rot: Angle90::A90,
            axis: Axis::Y,
            forward: false,
        },
        Rot {
            x_axis_rot: Angle90::A180,
            axis: Axis::Y,
            forward: true,
        },
        Rot {
            x_axis_rot: Angle90::A180,
            axis: Axis::Y,
            forward: false,
        },
        Rot {
            x_axis_rot: Angle90::A270,
            axis: Axis::Y,
            forward: true,
        },
        Rot {
            x_axis_rot: Angle90::A270,
            axis: Axis::Y,
            forward: false,
        },
        Rot {
            x_axis_rot: Angle90::A0,
            axis: Axis::Z,
            forward: true,
        },
        Rot {
            x_axis_rot: Angle90::A0,
            axis: Axis::Z,
            forward: false,
        },
        Rot {
            x_axis_rot: Angle90::A90,
            axis: Axis::Z,
            forward: true,
        },
        Rot {
            x_axis_rot: Angle90::A90,
            axis: Axis::Z,
            forward: false,
        },
        Rot {
            x_axis_rot: Angle90::A180,
            axis: Axis::Z,
            forward: true,
        },
        Rot {
            x_axis_rot: Angle90::A180,
            axis: Axis::Z,
            forward: false,
        },
        Rot {
            x_axis_rot: Angle90::A270,
            axis: Axis::Z,
            forward: true,
        },
        Rot {
            x_axis_rot: Angle90::A270,
            axis: Axis::Z,
            forward: false,
        },
    ];

    fn test() {
        assert_eq!(Matrix::ID, Rot::ALL_24[0].matrix());

        for rot in Rot::ALL_24 {
            let id = rot.matrix() * rot.matrix_rev();
            assert_eq!(Matrix::ID, id);
        }
    }
}

#[derive(Clone, PartialEq)]
struct Scanner {
    beacons: Vec<Vector>,
    set: HashSet<Vector>,
}

impl Scanner {
    fn new(beacons: Vec<Vector>) -> Scanner {
        let set = HashSet::<Vector>::from_iter(beacons.iter().copied());
        Scanner { beacons, set }
    }

    fn offset(&self, offset: Vector) -> Scanner {
        Scanner::new(self.beacons.iter().map(|b| *b + offset).collect())
    }

    fn rot(&self, matrix: Matrix) -> Scanner {
        Scanner::new(self.beacons.iter().map(|v| matrix * *v).collect())
    }

    fn intersect(&self, other: &Scanner) -> Option<VectorList> {
        if self.set.intersection(&other.set).count() >= 12 {
            Some(VectorList {
                data: self
                    .beacons
                    .iter()
                    .filter(|b| other.set.contains(b))
                    .copied()
                    .collect(),
            })
        } else {
            None
        }
    }
}

struct ScannerOffset {
    scanner: Scanner,
    offset: Vector,
}

impl ScannerOffset {
    fn orig(&self) -> Scanner {
        self.scanner.offset(self.offset)
    }

    fn new(scanner: &Scanner, offset: Vector) -> ScannerOffset {
        let scanner = scanner.offset(-offset);
        ScannerOffset { scanner, offset }
    }

    fn intersect(&self, other: &ScannerOffset) -> Option<VectorList> {
        self.scanner
            .intersect(&other.scanner)
            .map(|vs| vs.shift(-self.offset))
    }
}

struct ScannerOffsets {
    offsets: Vec<ScannerOffset>,
}

impl ScannerOffsets {
    fn new(scanner: &Scanner) -> ScannerOffsets {
        let offsets = scanner
            .beacons
            .iter()
            .map(|b| ScannerOffset::new(scanner, *b))
            .collect();
        ScannerOffsets { offsets }
    }

    fn intersect(&self, other: &ScannerOffsets) -> Option<(Vector, VectorList)> {
        for self_offset in &self.offsets {
            let offsets: Vec<(Vector, VectorList)> = other
                .offsets
                .iter()
                .filter_map(|o| {
                    if let Some(intersect) = self_offset.intersect(o) {
                        Some((self_offset.offset - o.offset, intersect))
                    } else {
                        None
                    }
                })
                .collect();
            assert!(offsets.len() <= 1);
            if let Some(x) = offsets.into_iter().next() {
                return Some(x);
            }
        }
        None
    }

    fn intersect_any(&self, other: &ScannerRotsOffsets) -> Option<(Transform, VectorList)> {
        let mut offset = None;
        for (that_scanner, rot) in other.scanners_by_rot.iter().zip(Rot::ALL_24) {
            let next = self.intersect(that_scanner);
            if let Some((next, beacons)) = next {
                assert!(offset.is_none());
                offset = Some((Transform { offset: next, rot }, beacons));
            }
        }
        offset
    }
}

struct ScannerRotsOffsets {
    scanners_by_rot: [ScannerOffsets; 24],
}

impl ScannerRotsOffsets {
    fn new(scanner: &Scanner) -> ScannerRotsOffsets {
        assert_eq!(Matrix::ID, Matrix::all_24()[0]);
        ScannerRotsOffsets {
            scanners_by_rot: Matrix::all_24().map(|m| ScannerOffsets::new(&scanner.rot(m))),
        }
    }
}

struct Input {
    scanners: Vec<Scanner>,
    scanner_rots_offsets: Vec<ScannerRotsOffsets>,
}

impl Input {
    fn max_beacons(&self) -> usize {
        self.scanners.iter().map(|s| s.beacons.len()).max().unwrap()
    }

    fn parse_scanners(filename: &str, check_index: bool) -> Vec<Scanner> {
        let content = fs::read_to_string(filename).expect("Something went wrong reading the file");
        let mut scanners: Vec<Scanner> = Vec::new();
        let mut lines = content.lines();

        let mut i = 0;
        'full: loop {
            let line = lines.next().unwrap();
            assert!(line.starts_with("--- scanner "));
            let line = &line["--- scanner ".len()..];
            assert!(line.ends_with(" ---"));
            let line = &line[..line.len() - " ---".len()];
            let scanner_id: u32 = line.to_string().parse().unwrap();
            assert_eq!(if check_index { i } else { 0 }, scanner_id);

            let mut beacons: Vec<Vector> = Vec::new();

            loop {
                match lines.next() {
                    None => {
                        assert!(beacons.len() > 0);
                        scanners.push(Scanner::new(beacons));
                        break 'full;
                    }
                    Some(line) if line.is_empty() => {
                        assert!(beacons.len() > 0);
                        scanners.push(Scanner::new(beacons));
                        break;
                    }
                    Some(line) => {
                        let beacon = Vector::parse(line);
                        beacons.push(beacon);
                    }
                }
            }

            i += 1;
        }

        assert!(scanners.len() > 0);
        scanners
    }

    fn parse(filename: &str) -> Input {
        let scanners = Input::parse_scanners(filename, true);

        let scanner_rots = scanners.iter().map(ScannerRotsOffsets::new).collect();

        Input {
            scanners,
            scanner_rots_offsets: scanner_rots,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
struct Transform {
    offset: Vector,
    rot: Rot,
}

impl Mul<Vector> for Transform {
    type Output = Vector;

    fn mul(self, v: Vector) -> Vector {
        let v = self.rot * v;
        v + self.offset
    }
}

impl Mul<Transform> for Transform {
    type Output = Transform;

    fn mul(self, rhs: Transform) -> Transform {
        Transform {
            offset: self.offset + self.rot * rhs.offset,
            rot: self.rot * rhs.rot,
        }
    }
}

impl Transform {
    fn test_mul_vec() {
        assert_eq!(
            Vector::new(12, 23, 31),
            Transform {
                offset: Vector::new(10, 20, 30),
                rot: Rot {
                    x_axis_rot: Angle90::A90,
                    axis: Axis::Y,
                    forward: false,
                },
            } * Vector::new(1, 2, 3)
        );
    }

    fn test_mul_tr() {
        let tr1 = Transform {
            offset: Vector::new(10, 20, 30),
            rot: Rot::from_matrix(&Matrix::rot_z(Angle90::A90)),
        };
        let tr2 = Transform {
            offset: Vector::new(1, 2, 3),
            rot: Rot::from_matrix(&Matrix::rot_y(Angle90::A270)),
        };
        let tr3 = tr1 * tr2;
        // let expected = Transform {
        //     offset: Vector::new(10, 20, 30),
        //     rot: Rot {
        //
        //     },
        // };
        // assert_eq!(expected, tr3);
    }

    fn test() {
        Self::test_mul_vec();
        Self::test_mul_tr();
    }
}

fn test_test_input(input: &Input) {
    let offset_0_1 = input.scanner_rots_offsets[0].scanners_by_rot[0]
        .intersect_any(&input.scanner_rots_offsets[1])
        .unwrap();
    // println!("{:?}", offset_0_1);
    // offset_0_1.2.print();
    // println!("XX");
    // println!();
    // println!("actual:");
    // println!("{}", offset_0_1.0);
    // println!();
    // println!("expected:");
    // println!("{}", Vector::new(68, -1246, -43));
    // println!();
    assert_eq!(Vector::new(68, -1246, -43), offset_0_1.0.offset);

    let offset_1_4 = input.scanner_rots_offsets[1].scanners_by_rot[0]
        .intersect_any(&input.scanner_rots_offsets[4])
        .unwrap();
}

fn part1(filename: &str, test: bool) {
    println!("{}", filename);
    let input = Input::parse(filename);

    if test {
        test_test_input(&input);
    }
    // return;

    println!("max beacons: {}", input.max_beacons());

    let mut resolved_scanners: Vec<Option<Transform>> = vec![None; input.scanners.len()];
    resolved_scanners[0] = Some(Transform {
        offset: Vector::ZERO,
        rot: Rot::default(),
    });

    let mut pairs_checked = HashSet::new();

    while resolved_scanners.iter().any(|r| r.is_none()) {
        for i in 0..input.scanners.len() {
            println!(
                "rem {}; i: {}",
                resolved_scanners.iter().filter(|r| r.is_none()).count(),
                i
            );
            let i_resolved = match resolved_scanners[i].clone() {
                Some(r) => r,
                None => {
                    // println!("resolved");
                    continue;
                }
            };
            for j in 0..input.scanners.len() {
                if resolved_scanners[j].is_some() {
                    // println!("skipping some {} {}", i, j);
                    continue;
                }

                if !pairs_checked.insert((i, j)) {
                    // println!("skipping checked {} {}", i, j);
                    continue;
                }

                // println!("i: {}, j: {}", i, j);
                if let Some((tr, _beacons)) = input.scanner_rots_offsets[i].scanners_by_rot[0]
                    .intersect_any(&input.scanner_rots_offsets[j])
                {
                    // println!("hit");
                    resolved_scanners[j] = Some(i_resolved * tr);
                }
                // println!("i: {}, j: {}.", i, j);
            }
            // println!("i: {}.", i);
        }
    }

    let mut all_beacons: HashSet<Vector> = HashSet::new();

    for i in 0..input.scanners.len() {
        let resolved = resolved_scanners[i].unwrap();
        let beacons = &input.scanners[i];
        for &beacon in &beacons.beacons {
            let beacon = resolved * beacon;
            all_beacons.insert(beacon);
        }
    }

    println!("{}", all_beacons.len());
    assert!(all_beacons.len() == 79 || all_beacons.len() == 390);
}

fn test_sample_rots() {
    let mut scanners = Input::parse_scanners("day19-sample-rots.txt", false);
    let first = scanners.swap_remove(0);
    for rot in Rot::ALL_24 {
        let rotated = first.rot(rot.matrix());
        if let Some(i) = scanners.iter().position(|s| s == &rotated) {
            scanners.swap_remove(i);
        }
    }
    assert!(scanners.is_empty());
}

fn main() {
    Vector::test();
    Matrix::test();
    Rot::test();
    Transform::test();
    test_sample_rots();

    println!("Part 1");
    part1("day19-input-test.txt", true);
    part1("day19-input.txt", false);
}
