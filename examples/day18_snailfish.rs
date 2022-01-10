use std::cmp;
use std::fmt;
use std::fmt::Formatter;
use std::fs;

#[derive(PartialEq, Debug, Clone)]
enum Elem {
    Number(u64),
    Pair(Box<Pair>),
}

#[derive(PartialEq, Debug, Clone)]
struct Pair {
    elems: [Elem; 2],
}

impl fmt::Display for Elem {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Elem::Number(n) => write!(f, "{}", n),
            Elem::Pair(p) => write!(f, "{}", p),
        }
    }
}

impl fmt::Display for Pair {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{},{}]", self.elems[0], self.elems[1])
    }
}

impl Elem {
    fn parse(input: &str) -> Elem {
        let mut parser = Parser { input, pos: 0 };
        let elem = parser.next_elem();
        assert!(parser.eof(), "while parsing: `{}`", input);
        elem
    }

    fn number(&self) -> u64 {
        match self {
            Elem::Number(n) => *n,
            Elem::Pair(..) => panic!(),
        }
    }

    fn most_value(&mut self, index: usize) -> &mut u64 {
        match self {
            Elem::Number(n) => n,
            Elem::Pair(p) => p.most_value(index),
        }
    }

    fn explode_impl(
        &mut self,
        outer: u32,
        left: Option<&mut u64>,
        right: Option<&mut u64>,
    ) -> bool {
        // println!("explode_impl {} {}", self, outer);
        match self {
            Elem::Number(_) => false,
            Elem::Pair(pair) if outer == 3 => {
                let pair = pair.two_numbers();
                if let Some(left) = left {
                    *left += pair[0];
                }
                if let Some(right) = right {
                    *right += pair[1];
                }
                *self = Elem::Number(0);
                true
            }
            Elem::Pair(pair) => pair.explode_impl(outer + 1, left, right),
        }
    }

    fn split(&mut self) -> bool {
        match self {
            Elem::Number(n) if *n >= 10 => {
                *self = Elem::Pair(Box::new(Pair {
                    elems: [Elem::Number(*n / 2), Elem::Number((*n + 1) / 2)],
                }));
                true
            }
            Elem::Number(_) => false,
            Elem::Pair(p) => p.split(),
        }
    }

    fn magnitude(&self) -> u64 {
        match self {
            Elem::Number(n) => *n,
            Elem::Pair(p) => p.magnitude(),
        }
    }
}

impl Pair {
    fn parse(input: &str) -> Pair {
        let mut parser = Parser { input, pos: 0 };
        let pair = parser.next_pair();
        assert!(parser.eof());
        pair
    }

    fn two_numbers(&self) -> [u64; 2] {
        [self.elems[0].number(), self.elems[1].number()]
    }

    fn most_value(&mut self, index: usize) -> &mut u64 {
        self.elems[index].most_value(index)
    }

    fn explode_impl(
        &mut self,
        outer: u32,
        left: Option<&mut u64>,
        right: Option<&mut u64>,
    ) -> bool {
        let [e_0, e_1] = &mut self.elems;
        if e_0.explode_impl(outer, left, Some(e_1.most_value(0))) {
            return true;
        }
        if e_1.explode_impl(outer, Some(e_0.most_value(1)), right) {
            return true;
        }
        false
    }

    fn explode(&mut self) -> bool {
        self.explode_impl(0, None, None)
    }

    fn split(&mut self) -> bool {
        if self.elems[0].split() {
            return true;
        }
        if self.elems[1].split() {
            return true;
        }
        false
    }

    fn add(a: &Pair, b: &Pair) -> Pair {
        let mut sum = Pair {
            elems: [
                Elem::Pair(Box::new(a.clone())),
                Elem::Pair(Box::new(b.clone())),
            ],
        };
        loop {
            if sum.explode() {
                continue;
            }
            if sum.split() {
                continue;
            }
            break;
        }
        sum
    }

    fn magnitude(&self) -> u64 {
        let [l, r] = &self.elems;
        l.magnitude() * 3 + r.magnitude() * 2
    }
}

struct Parser<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> Parser<'a> {
    fn eof(&self) -> bool {
        self.pos == self.input.len()
    }

    fn peek(&self) -> char {
        self.input[self.pos..].chars().next().unwrap()
    }

    fn next(&mut self) -> char {
        let ch = self.peek();
        self.pos += ch.len_utf8();
        ch
    }

    fn next_elem(&mut self) -> Elem {
        if self.peek() == '[' {
            Elem::Pair(Box::new(self.next_pair()))
        } else {
            let mut n: u64 = self.next().to_string().parse().unwrap();
            while !self.eof() && self.peek().is_ascii_digit() {
                n = n * 10 + self.next().to_string().parse::<u64>().unwrap();
            }
            Elem::Number(n)
        }
    }

    fn next_pair(&mut self) -> Pair {
        assert_eq!('[', self.next());
        let l = self.next_elem();
        assert_eq!(',', self.next());
        let r = self.next_elem();
        assert_eq!(']', self.next());
        Pair { elems: [l, r] }
    }
}

fn test_explode_impl(input: &str, expected: &str) {
    let mut actual = Pair::parse(input);
    let expected = Pair::parse(expected);

    actual.explode();
    assert_eq!(
        expected, actual,
        "\ninput:    {}\nexpected: {}\nactual:   {}",
        input, expected, actual
    );
}

fn test_explode() {
    test_explode_impl("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]");
    test_explode_impl("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]");
    test_explode_impl("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]");
    test_explode_impl(
        "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
        "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
    );
    test_explode_impl(
        "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
        "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
    );
}

fn test_split_impl(input: &str, expected: &str) {
    let mut actual = Elem::parse(input);
    actual.split();
    assert_eq!(
        expected,
        actual.to_string(),
        "\ninput:    {}\nexpected: {}\nactual:   {}",
        input,
        expected,
        actual
    );
}

fn test_split() {
    test_split_impl("10", "[5,5]");
    test_split_impl("11", "[5,6]");
    test_split_impl("12", "[6,6]");
}

fn test_sum_impl(a: &str, b: &str, c: &str) {
    let actual = Pair::add(&Pair::parse(a), &Pair::parse(b));
    assert_eq!(
        c,
        actual.to_string(),
        "\ninput:    {} + {}\nexpected: {}\nactual:   {}",
        a,
        b,
        c,
        actual
    );
}

fn test_sum_list_impl(args: &[&str], expected: &str) {
    let actual = args
        .iter()
        .map(|a| Pair::parse(a))
        .reduce(|a, b| Pair::add(&a, &b))
        .unwrap();
    assert_eq!(expected, actual.to_string());
}

fn test_sum() {
    test_sum_impl(
        "[[[[3,3],4],4],[7,[[8,4],9]]]",
        "[1,1]",
        "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]",
    );
    test_sum_list_impl(
        &["[1,1]", "[2,2]", "[3,3]", "[4,4]"],
        "[[[[1,1],[2,2]],[3,3]],[4,4]]",
    );
    test_sum_list_impl(
        &["[1,1]", "[2,2]", "[3,3]", "[4,4]", "[5,5]"],
        "[[[[3,0],[5,3]],[4,4]],[5,5]]",
    );
    test_sum_list_impl(
        &["[1,1]", "[2,2]", "[3,3]", "[4,4]", "[5,5]", "[6,6]"],
        "[[[[5,0],[7,4]],[5,5]],[6,6]]",
    );
    test_sum_list_impl(
        &[
            "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]",
            "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]",
            "[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]",
            "[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]",
            "[7,[5,[[3,8],[1,4]]]]",
            "[[2,[2,2]],[8,[8,1]]]",
            "[2,9]",
            "[1,[[[9,3],9],[[9,0],[0,7]]]]",
            "[[[5,[7,4]],7],1]",
            "[[[[4,2],2],6],[8,7]]",
        ],
        "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
    );
}

fn test_magnitude() {
    assert_eq!(143, Pair::parse("[[1,2],[[3,4],5]]").magnitude());
    assert_eq!(
        1384,
        Pair::parse("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").magnitude()
    );
    assert_eq!(
        445,
        Pair::parse("[[[[1,1],[2,2]],[3,3]],[4,4]]").magnitude()
    );
    assert_eq!(
        791,
        Pair::parse("[[[[3,0],[5,3]],[4,4]],[5,5]]").magnitude()
    );
    assert_eq!(
        1137,
        Pair::parse("[[[[5,0],[7,4]],[5,5]],[6,6]]").magnitude()
    );
    assert_eq!(
        3488,
        Pair::parse("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]").magnitude()
    );
}

fn part1(filename: &str) {
    println!("{}", filename);
    let content = fs::read_to_string(filename).unwrap();
    let mut pairs = content.lines().map(|s| Pair::parse(s));
    let mut sum = pairs.next().unwrap();
    for pair in pairs {
        sum = Pair::add(&sum, &pair);
    }
    println!("{}", sum.magnitude());
}

fn part2(filename: &str) {
    println!("{}", filename);
    let content = fs::read_to_string(filename).unwrap();
    let pairs: Vec<_> = content.lines().map(|s| Pair::parse(s)).collect();
    let mut largest_magnitude = u64::MIN;
    for a in &pairs {
        for b in &pairs {
            let magnitude = Pair::add(a, b).magnitude();
            largest_magnitude = cmp::max(largest_magnitude, magnitude);
        }
    }
    println!("{}", largest_magnitude);
}

fn main() {
    test_explode();
    test_split();
    test_sum();
    test_magnitude();

    println!("part 1");
    part1("day18-input-test.txt");
    part1("day18-input.txt");

    println!();
    println!("part 2");
    part2("day18-input-test.txt");
    part2("day18-input.txt");
}
