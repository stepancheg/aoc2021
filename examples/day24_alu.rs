use std::cmp;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::fmt::write;
use std::fs;
use std::mem;

use rand::Rng;
use rand::SeedableRng;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Reg {
    X,
    Y,
    Z,
    W,
}

impl fmt::Display for Reg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Reg::X => "x",
                Reg::Y => "y",
                Reg::Z => "z",
                Reg::W => "w",
            }
        )
    }
}

impl Reg {
    const COUNT: usize = 4;

    fn index(&self) -> usize {
        match self {
            Reg::X => 0,
            Reg::Y => 1,
            Reg::Z => 2,
            Reg::W => 3,
        }
    }

    fn parse(s: &str) -> Reg {
        match s {
            "x" => Reg::X,
            "y" => Reg::Y,
            "z" => Reg::Z,
            "w" => Reg::W,
            s => panic!("Unknown variable name: {}", s),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Arg {
    VarName(Reg),
    Literal(i64),
}

impl fmt::Display for Arg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Arg::VarName(v) => write!(f, "{}", v),
            Arg::Literal(n) => write!(f, "{}", n),
        }
    }
}

impl Arg {
    fn parse(s: &str) -> Arg {
        match s {
            "x" | "y" | "z" | "w" => Arg::VarName(Reg::parse(s)),
            s => Arg::Literal(s.parse::<i64>().unwrap()),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum Opcode {
    Inp,
    Add,
    Mul,
    Div,
    Mod,
    Eql,
}

#[derive(Copy, Clone)]
enum Instr {
    Inp(Reg),
    Add(Reg, Arg),
    Mul(Reg, Arg),
    Div(Reg, Arg),
    Mod(Reg, Arg),
    Eql(Reg, Arg),
}

impl fmt::Display for Instr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Instr::Inp(v) => write!(f, "inp {}", v),
            Instr::Add(v, a) => write!(f, "add {} {}", v, a),
            Instr::Mul(v, a) => write!(f, "mul {} {}", v, a),
            Instr::Div(v, a) => write!(f, "div {} {}", v, a),
            Instr::Mod(v, a) => write!(f, "mod {} {}", v, a),
            Instr::Eql(v, a) => write!(f, "eql {} {}", v, a),
        }
    }
}

impl Instr {
    fn parse(s: &str) -> Instr {
        let parts: Vec<_> = s.split(" ").collect();
        if parts[0] == "inp" {
            assert_eq!(2, parts.len());
            let var_name = Reg::parse(parts[1]);
            Instr::Inp(var_name)
        } else {
            assert_eq!(3, parts.len());
            let var_name = Reg::parse(parts[1]);
            let arg = Arg::parse(parts[2]);

            let instr = match parts[0] {
                "add" => Instr::Add(var_name, arg),
                "mul" => Instr::Mul(var_name, arg),
                "div" => Instr::Div(var_name, arg),
                "mod" => Instr::Mod(var_name, arg),
                "eql" => Instr::Eql(var_name, arg),
                s => panic!("Unknown instruction: {}", s),
            };

            match instr {
                Instr::Div(_, Arg::VarName(_)) => panic!("Division by variable not supported"),
                Instr::Mod(_, Arg::VarName(_)) => panic!("Modulo by variable not supported"),
                _ => {}
            }

            instr
        }
    }

    fn eval_bin_op(&self, reg: &mut Registers, var: Reg, arg: Arg, op: impl Fn(i64, i64) -> i64) {
        let arg0 = reg.get(var);
        let arg1 = match arg {
            Arg::VarName(var) => reg.get(var),
            Arg::Literal(value) => value,
        };
        let res = op(arg0, arg1);
        reg.set(var, res);
    }

    fn eval(&self, reg: &mut Registers, input: &mut Input) {
        match self {
            Instr::Inp(var) => reg.set(*var, input.next()),
            Instr::Add(var, arg) => self.eval_bin_op(reg, *var, *arg, |a, b| a + b),
            Instr::Mul(var, arg) => self.eval_bin_op(reg, *var, *arg, |a, b| a * b),
            Instr::Div(var, arg) => self.eval_bin_op(reg, *var, *arg, |a, b| a / b),
            Instr::Mod(var, arg) => self.eval_bin_op(reg, *var, *arg, |a, b| {
                assert!(a >= 0);
                assert!(b > 0);
                a % b
            }),
            Instr::Eql(var, arg) => {
                self.eval_bin_op(reg, *var, *arg, |a, b| if a == b { 1 } else { 0 })
            }
        }
    }

    fn out_reg(&self) -> Reg {
        match self {
            Instr::Inp(var) => *var,
            Instr::Add(var, _) => *var,
            Instr::Mul(var, _) => *var,
            Instr::Div(var, _) => *var,
            Instr::Mod(var, _) => *var,
            Instr::Eql(var, _) => *var,
        }
    }

    fn opcode(&self) -> Opcode {
        match self {
            Instr::Inp(_) => Opcode::Inp,
            Instr::Add(_, _) => Opcode::Add,
            Instr::Mul(_, _) => Opcode::Mul,
            Instr::Div(_, _) => Opcode::Div,
            Instr::Mod(_, _) => Opcode::Mod,
            Instr::Eql(_, _) => Opcode::Eql,
        }
    }
}

#[derive(Default, Eq, PartialEq, Debug)]
struct Registers {
    registers: [i64; 4],
}

impl Registers {
    fn get(&self, register: Reg) -> i64 {
        self.registers[register.index()]
    }

    fn set(&mut self, register: Reg, value: i64) {
        self.registers[register.index()] = value;
    }
}

struct Input {
    numbers: Vec<i64>,
    pos: usize,
}

impl Input {
    fn parse_digits_from_dec(s: &str) -> Input {
        Self::parse_digits(&s.to_string())
    }

    fn parse_digits(input: &str) -> Input {
        let mut numbers = Vec::new();
        for c in input.chars() {
            numbers.push(c.to_string().parse().unwrap());
        }
        Input { numbers, pos: 0 }
    }

    fn next(&mut self) -> i64 {
        let r = self.numbers[self.pos];
        self.pos += 1;
        r
    }

    fn eof(&self) -> bool {
        self.pos == self.numbers.len()
    }
}

#[derive(Debug)]
struct Range {
    from: i64,
    to_incl: i64,
}

impl Range {
    fn new(from: i64, to_incl: i64) -> Range {
        assert!(from <= to_incl);
        Range { from, to_incl }
    }

    fn shift(&self, shift: i64) -> Range {
        Range::new(self.from + shift, self.to_incl + shift)
    }

    fn may_intersect(a: &Range, b: &Range) -> bool {
        let max_from = cmp::max(a.from, b.from);
        let min_to_incl = cmp::min(a.to_incl, b.to_incl);
        max_from <= min_to_incl
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
enum Expr {
    Literal(i64),
    Inp(usize),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Mod(Box<Expr>, Box<Expr>),
    Eql(Box<Expr>, Box<Expr>),
}

impl Expr {
    fn range(&self) -> Option<Range> {
        match self {
            Expr::Literal(n) => Some(Range::new(*n, *n)),
            Expr::Inp(_) => Some(Range::new(1, 9)),
            Expr::Eql(..) => Some(Range::new(0, 1)),
            Expr::Mod(a, b) => match (a.lit(), b.lit()) {
                (Some(a), Some(b)) => {
                    assert!(a >= 0);
                    assert!(b > 0);
                    Some(Range::new(0, cmp::min(a, b - 1)))
                }
                (Some(a), None) => {
                    assert!(a >= 0);
                    Some(Range::new(0, a))
                }
                (None, Some(b)) => {
                    assert!(b > 0);
                    Some(Range::new(0, b - 1))
                }
                (None, None) => None,
            },
            Expr::Add(a, b) => {
                if let (Some(a_range), Some(b_range)) = (a.range(), b.range()) {
                    Some(Range::new(
                        a_range.from + b_range.from,
                        a_range.to_incl + b_range.to_incl,
                    ))
                } else {
                    None
                }
            }
            Expr::Mul(a, b) => {
                if let (Some(a_range), Some(b_range)) = (a.range(), b.range()) {
                    if a_range.from >= 0 && b_range.from >= 0 {
                        return Some(Range::new(
                            a_range.from * b_range.from,
                            a_range.to_incl * b_range.to_incl,
                        ));
                    }
                }
                None
            }
            Expr::Div(a, b) => {
                if let (Some(a), Some(b)) = (a.range(), b.lit()) {
                    assert!(a.from >= 0);
                    assert!(b > 0);
                    Some(Range::new(a.from / b, a.to_incl / b))
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn lit(&self) -> Option<i64> {
        if let Some(range) = self.range() {
            if range.from == range.to_incl {
                Some(range.from)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn is_lit(&self, value: i64) -> bool {
        match self.lit() {
            Some(v) => v == value,
            None => false,
        }
    }

    fn is_add_lit(&self) -> Option<(&Expr, i64)> {
        if let Expr::Add(a, b) = self {
            if let Some(a) = a.lit() {
                return Some((b, a));
            }
            if let Some(b) = b.lit() {
                return Some((a, b));
            }
        }
        None
    }

    fn is_input(&self) -> bool {
        match self {
            Expr::Inp(_) => true,
            _ => false,
        }
    }

    fn eql(a: Expr, b: Expr) -> Expr {
        if a == b {
            return Expr::Literal(1);
        }
        if let (Some(a_range), Some(b_range)) = (a.range(), b.range()) {
            if !Range::may_intersect(&a_range, &b_range) {
                // println!("cannot {:?} {:?}", a_range, b_range);
                return Expr::Literal(0);
            }
        }
        if let (Some(a), Some(b)) = (a.lit(), b.lit()) {
            assert_ne!(a, b);
            return Expr::Literal(0);
        }

        if let (Some((a_e, a_c)), Some((b_e, b_c))) = (a.is_add_lit(), b.is_add_lit()) {
            if a_c == b_c {
                return Expr::eql(a_e.clone(), b_e.clone());
            }
        }

        if let (Some((a_e, a_c)), Some(b)) = (a.is_add_lit(), b.lit()) {
            return Self::eql(a_e.clone(), Expr::Literal(b - a_c));
        }
        if let (Some(a), Some((b_e, b_c))) = (a.lit(), b.is_add_lit()) {
            return Self::eql(Expr::Literal(a - b_c), b_e.clone());
        }

        Expr::Eql(Box::new(a), Box::new(b))
    }

    fn opt(self) -> Expr {
        match self {
            Expr::Add(a, b) => match (a.lit(), b.lit()) {
                (Some(0), _) => *b,
                (_, Some(0)) => *a,
                (Some(a), Some(b)) => Expr::Literal(a + b),
                _ => Expr::Add(a, b),
            },
            Expr::Mul(a, b) if a.is_lit(1) => *b,
            Expr::Mul(a, b) if b.is_lit(1) => *a,
            Expr::Mul(a, b) if a.is_lit(0) || b.is_lit(0) => Expr::Literal(0),
            Expr::Div(a, b) => {
                if a.is_lit(0) {
                    return Expr::Literal(0);
                }
                if b.is_lit(1) {
                    return *a;
                }
                Expr::Div(a, b)
            }
            Expr::Mod(a, b) => {
                if a.is_lit(0) {
                    return Expr::Literal(0);
                }
                if b.is_lit(1) {
                    return Expr::Literal(0);
                }
                Expr::Mod(a, b)
            }
            Expr::Eql(a, b) => Self::eql(*a, *b),
            e => e,
        }
    }

    fn print_tree_impl(&self, indent: usize) {
        match self {
            Expr::Literal(value) => println!("{:>width$}{}", "", value, width = indent),
            Expr::Inp(i) => println!("{:>width$}input[{}]", "", i, width = indent),
            e => {
                let (op, a, b) = match e {
                    Expr::Literal(..) | Expr::Inp(..) => unreachable!(),
                    Expr::Add(a, b) => ("+", a, b),
                    Expr::Mul(a, b) => ("*", a, b),
                    Expr::Div(a, b) => ("/", a, b),
                    Expr::Mod(a, b) => ("%", a, b),
                    Expr::Eql(a, b) => ("==", a, b),
                };
                println!("{:>width$}{} ({:?})", "", op, self.range(), width = indent);
                a.print_tree_impl(indent + 2);
                b.print_tree_impl(indent + 2);
            }
        }
    }

    fn print_tree(&self) {
        self.print_tree_impl(0)
    }

    fn expr_type(&self) -> String {
        match self {
            Expr::Literal(..) | Expr::Inp(..) => self.to_string(),
            Expr::Add(..) => "+".to_string(),
            Expr::Mul(..) => "*".to_string(),
            Expr::Div(..) => "/".to_string(),
            Expr::Mod(..) => "%".to_string(),
            Expr::Eql(..) => "==".to_string(),
        }
    }

    fn short(&self) -> String {
        format!("{} {:?}", self.expr_type(), self.range())
    }

    fn args(&self) -> (&Expr, &Expr) {
        match self {
            Expr::Inp(..) => panic!(),
            Expr::Literal(..) => panic!(),
            Expr::Add(a, b) => (a, b),
            Expr::Mul(a, b) => (a, b),
            Expr::Div(a, b) => (a, b),
            Expr::Mod(a, b) => (a, b),
            Expr::Eql(a, b) => (a, b),
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Inp(i) => write!(f, "input[{}]", i),
            Expr::Literal(v) => write!(f, "{}", v),
            Expr::Add(l, r) => write!(f, "({} + {})", l, r),
            Expr::Mul(l, r) => write!(f, "({} * {})", l, r),
            Expr::Div(l, r) => write!(f, "({} / {})", l, r),
            Expr::Mod(l, r) => write!(f, "({} % {})", l, r),
            Expr::Eql(l, r) => write!(f, "({} == {})", l, r),
        }
    }
}

enum ExprBool {
    Eq0(Expr),
    Or(Box<ExprBool>, Box<ExprBool>),
    Not(Box<ExprBool>),
    True,
    False,
}

impl ExprBool {
    fn is_true(&self) -> bool {
        match self {
            ExprBool::True => true,
            _ => false,
        }
    }

    fn is_false(&self) -> bool {
        match self {
            ExprBool::False => true,
            _ => false,
        }
    }

    fn or(a: ExprBool, b: ExprBool) -> ExprBool {
        if a.is_true() {
            ExprBool::True
        } else if b.is_true() {
            ExprBool::True
        } else if a.is_false() {
            b
        } else if b.is_false() {
            a
        } else {
            ExprBool::Or(Box::new(a), Box::new(b))
        }
    }

    fn not(a: ExprBool) -> ExprBool {
        if a.is_false() {
            ExprBool::True
        } else if a.is_true() {
            ExprBool::False
        } else {
            ExprBool::Not(Box::new(a))
        }
    }

    fn eq_0(expr: &Expr) -> ExprBool {
        if let Some(range) = expr.range() {
            if range.from > 0 {
                return ExprBool::False;
            }
            if range.to_incl < 0 {
                return ExprBool::False;
            }
        }
        match expr {
            Expr::Literal(v) => match v {
                0 => ExprBool::True,
                _ => ExprBool::False,
            },
            Expr::Eql(a, b) => {
                if a.is_lit(0) {
                    Self::eq_0(b)
                } else if b.is_lit(0) {
                    Self::eq_0(a)
                } else {
                    ExprBool::Eq0(expr.clone())
                }
            }
            Expr::Add(a, b) => {
                if let (Some(a_range), Some(b_range)) = (a.range(), b.range()) {
                    if a_range.from == 0 && b_range.from == 0 {
                        return Self::or(Self::eq_0(a), Self::eq_0(b));
                    }
                }
                ExprBool::Eq0(expr.clone())
            }
            Expr::Mul(a, b) => Self::or(Self::eq_0(a), Self::eq_0(b)),
            Expr::Div(a, b) => {
                if let Some(b_range) = b.range() {
                    if b_range.from != 0 {
                        return Self::eq_0(a);
                    } else {
                        panic!();
                    }
                } else {
                    panic!();
                }
                ExprBool::Eq0(expr.clone())
            }
            expr => ExprBool::Eq0(expr.clone()),
        }
    }

    fn print_tree_impl(&self, indent: usize) {
        match self {
            ExprBool::True => println!("{:>width$}{}", "", "true", width = indent),
            ExprBool::False => println!("{:>width$}{}", "", "false", width = indent),
            ExprBool::Eq0(e) => {
                println!("{:>width$}{}", "", "== 0", width = indent);
                e.print_tree_impl(indent + 2);
            }
            ExprBool::Not(e) => {
                println!("{:>width$}{}", "", "!", width = indent);
                e.print_tree_impl(indent + 2);
            }
            ExprBool::Or(a, b) => {
                println!("{:>width$}{}", "", "||", width = indent);
                a.print_tree_impl(indent + 2);
                b.print_tree_impl(indent + 2);
            }
        }
    }

    fn print_tree(&self) {
        self.print_tree_impl(0)
    }
}

impl fmt::Display for ExprBool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExprBool::True => write!(f, "true"),
            ExprBool::False => write!(f, "false"),
            ExprBool::Eq0(e) => write!(f, "{}", e),
            ExprBool::Or(a, b) => write!(f, "({} || {})", a, b),
            ExprBool::Not(e) => write!(f, "!({})", e),
        }
    }
}

#[derive(Default)]
struct KnownRegs {
    regs: [Option<i64>; Reg::COUNT],
}

impl KnownRegs {
    fn get_arg(&self, arg: Arg) -> Option<i64> {
        match arg {
            Arg::VarName(reg) => self.regs[reg.index()],
            Arg::Literal(value) => Some(value),
        }
    }
}

#[derive(Default)]
struct ProgramBuilder {
    instrs: Vec<Instr>,
    known_regs: KnownRegs,
}

struct Program {
    instrs: Vec<Instr>,
}

impl ProgramBuilder {
    fn push(&mut self, instr: Instr) {
        self.instrs.push(instr);
    }

    fn remove_stores(&mut self, reg: Reg) {
        while let Some(last) = self.instrs.last() {
            if let Instr::Inp(..) = last {
                break;
            }
            if last.out_reg() != reg {
                break;
            }
            self.instrs.pop().unwrap();
        }
    }

    fn set(&mut self, reg: Reg, value: i64) {
        if let Some(known) = self.known_regs.regs[reg.index()] {
            if known == value {
                return;
            }
        }

        self.remove_stores(reg);

        self.push(Instr::Mul(reg, Arg::Literal(0)));
        if value != 0 {
            self.push(Instr::Add(reg, Arg::Literal(value)));
        }
        self.known_regs.regs[reg.index()] = Some(value);
    }
}

impl Program {
    fn print(&self) {
        for instr in &self.instrs {
            println!("{}", instr);
        }
    }

    fn parse(prog: &str) -> Program {
        let instrs: Vec<_> = prog.lines().map(|l| Instr::parse(l)).collect();
        Program { instrs }
    }

    fn parse_file(filename: &str) -> Program {
        let content = fs::read_to_string(filename).unwrap();
        Program::parse(&content)
    }

    fn eval(&self, mut input: Input) -> Registers {
        let mut reg = Registers::default();
        for instr in &self.instrs {
            instr.eval(&mut reg, &mut input);
        }
        assert!(input.eof());
        reg
    }

    fn opt(&self) -> (Program, KnownRegs) {
        let mut instrs = ProgramBuilder {
            instrs: Vec::new(),
            known_regs: KnownRegs {
                regs: [Some(0), Some(0), Some(0), Some(0)],
            },
        };
        for &instr in &self.instrs {
            if let Instr::Inp(reg) = instr {
                instrs.push(Instr::Inp(reg));
                instrs.known_regs.regs[reg.index()] = None;
                continue;
            }

            let (reg, arg) = match instr {
                Instr::Inp(_reg) => unreachable!(),
                Instr::Add(reg, arg) => (reg, arg),
                Instr::Mul(reg, arg) => (reg, arg),
                Instr::Div(reg, arg) => (reg, arg),
                Instr::Mod(reg, arg) => (reg, arg),
                Instr::Eql(reg, arg) => (reg, arg),
            };

            let known_reg = instrs.known_regs.regs[reg.index()];
            let known_arg = instrs.known_regs.get_arg(arg);

            match instr {
                Instr::Inp(..) => unreachable!(),
                Instr::Add(..) => {
                    if let Some(0) = known_arg {
                        continue;
                    }
                }
                Instr::Mul(..) => {
                    if let Some(0) = known_arg {
                        instrs.set(reg, 0);
                        continue;
                    }
                    if let Some(1) = known_arg {
                        continue;
                    }
                }
                Instr::Div(..) => {
                    if let Some(0) = known_reg {
                        continue;
                    }
                    if let Some(1) = known_arg {
                        continue;
                    }
                }
                Instr::Mod(..) => {
                    if let Some(0) = known_reg {
                        continue;
                    }
                }
                Instr::Eql(..) => {}
            }

            if let (Some(known_reg), Some(known_arg)) = (known_reg, known_arg) {
                let r = match instr {
                    Instr::Inp(..) => unreachable!(),
                    Instr::Add(..) => known_reg + known_arg,
                    Instr::Mul(..) => known_reg * known_arg,
                    Instr::Div(..) => known_reg / known_arg,
                    Instr::Mod(..) => known_reg % known_arg,
                    Instr::Eql(..) => {
                        if known_reg == known_arg {
                            1
                        } else {
                            0
                        }
                    }
                };
                instrs.set(reg, r);
            } else {
                instrs.push(instr.clone());
                instrs.known_regs.regs[reg.index()] = None;
            }
        }
        (
            Program {
                instrs: instrs.instrs,
            },
            instrs.known_regs,
        )
    }

    fn opt_first_inp(&self, d: i64) -> (Program, KnownRegs) {
        let (index, reg) = self
            .instrs
            .iter()
            .copied()
            .enumerate()
            .find_map(|(i, instr)| match instr {
                Instr::Inp(reg) => Some((i, reg)),
                _ => None,
            })
            .unwrap();
        let mut instrs = Vec::new();
        instrs.extend(&self.instrs[..index]);
        instrs.push(Instr::Mul(reg, Arg::Literal(0)));
        instrs.push(Instr::Add(reg, Arg::Literal(d)));
        instrs.extend(&self.instrs[index + 1..]);
        Program { instrs }.opt()
    }

    fn to_expr(&self) -> Expr {
        let mut i = 0;
        let mut regs = [(); Reg::COUNT].map(|_| Expr::Literal(0));
        for (ip, &instr) in self.instrs.iter().enumerate() {
            println!("{}: {}", ip, instr);
            if let Instr::Inp(reg) = instr {
                let expr = Expr::Inp(i);
                println!("{} = {}", reg, expr);
                regs[reg.index()] = expr;
                i += 1;
                continue;
            }
            let (reg, arg) = match instr {
                Instr::Inp(..) => unreachable!(),
                Instr::Add(reg, arg) => (reg, arg),
                Instr::Mul(reg, arg) => (reg, arg),
                Instr::Div(reg, arg) => (reg, arg),
                Instr::Mod(reg, arg) => (reg, arg),
                Instr::Eql(reg, arg) => (reg, arg),
            };
            let reg_expr = Box::new(regs[reg.index()].clone());
            let arg_expr = Box::new(match arg {
                Arg::VarName(reg) => regs[reg.index()].clone(),
                Arg::Literal(value) => Expr::Literal(value),
            });
            let expr = match instr {
                Instr::Inp(..) => unreachable!(),
                Instr::Add(..) => Expr::Add(reg_expr, arg_expr),
                Instr::Mul(..) => Expr::Mul(reg_expr, arg_expr),
                Instr::Div(..) => Expr::Div(reg_expr, arg_expr),
                Instr::Mod(..) => Expr::Mod(reg_expr, arg_expr),
                Instr::Eql(..) => Expr::Eql(reg_expr, arg_expr),
            };
            println!("{} = {}", reg, expr);
            let expr = expr.opt();
            println!("{} = {}", reg, expr);
            // if expr.to_string().len() > 1000 {
            //     panic!();
            // }
            regs[reg.index()] = expr;
        }
        mem::replace(&mut regs[Reg::Z.index()], Expr::Literal(0))
    }
}

fn test_program(prog: &str, input: &str, expected: Registers) {
    let prog = prog.replace("; ", "\n");
    let prog = Program::parse(&prog);
    let input = Input::parse_digits(input);
    let reg = prog.eval(input);
    assert_eq!(expected, reg);
}

fn test() {
    test_program(
        "inp x; mul x -1",
        "8",
        Registers {
            registers: [-8, 0, 0, 0],
        },
    );
}

struct IncOpt {
    i: usize,
    digits: Vec<u32>,
}

impl IncOpt {
    fn run(&mut self, prog: &Program) {
        if self.digits.len() == 14 {
            unreachable!();
        } else {
            for d in (1..=9).rev() {
                self.digits.push(d);
                let (prog, known_regs) = prog.opt_first_inp(d as i64);
                let z = known_regs.regs[Reg::Z.index()];
                if let Some(z) = z {
                    // println!("ZZZZ");
                    // prog.print();
                    // panic!();
                    if self.i % 10_000 == 0 || self.digits.len() != 14 {
                        println!();
                        for digit in &self.digits {
                            print!("{}", digit);
                        }
                        println!();
                        println!("{}", z);
                    }
                    self.i += 1;

                    assert!(z != 0);
                    self.digits.pop().unwrap();
                    continue;
                }

                if self.digits.len() == 14 {
                    assert!(z.is_some());
                } else {
                    self.run(&prog);
                }
                self.digits.pop().unwrap();
            }
        }
    }
}

fn inc_opt(prog: &Program) {
    IncOpt {
        digits: Vec::new(),
        i: 0,
    }
    .run(prog);
}

#[derive(Debug)]
struct Block {
    params: [i64; 3],
}

impl Block {
    fn new(instrs: &[Instr]) -> Block {
        match instrs {
            [Instr::Inp(Reg::W), Instr::Mul(Reg::X, Arg::Literal(0)), Instr::Add(Reg::X, Arg::VarName(Reg::Z)), Instr::Mod(Reg::X, Arg::Literal(26)), Instr::Div(Reg::Z, Arg::Literal(p0)), Instr::Add(Reg::X, Arg::Literal(p1)), Instr::Eql(Reg::X, Arg::VarName(Reg::W)), Instr::Eql(Reg::X, Arg::Literal(0)), Instr::Mul(Reg::Y, Arg::Literal(0)), Instr::Add(Reg::Y, Arg::Literal(25)), Instr::Mul(Reg::Y, Arg::VarName(Reg::X)), Instr::Add(Reg::Y, Arg::Literal(1)), Instr::Mul(Reg::Z, Arg::VarName(Reg::Y)), Instr::Mul(Reg::Y, Arg::Literal(0)), Instr::Add(Reg::Y, Arg::VarName(Reg::W)), Instr::Add(Reg::Y, Arg::Literal(p2)), Instr::Mul(Reg::Y, Arg::VarName(Reg::X)), Instr::Add(Reg::Z, Arg::VarName(Reg::Y))] =>
            {
                assert!(*p0 == 1 || *p0 == 26);
                assert!(*p2 >= 4);
                Block {
                    params: [*p0, *p1, *p2],
                }
            }
            _ => panic!(),
        }
    }

    fn eval_full(&self, z: i64, w: i64) -> i64 /* z */ {
        // Instr::Mul(Reg::X, Arg::Literal(0)),
        let x = 0;
        // Instr::Add(Reg::X, Arg::VarName(Reg::Z)),
        let x = x + z;
        // Instr::Mod(Reg::X, Arg::Literal(26)),
        let x = x % 26;
        // Instr::Div(Reg::Z, Arg::Literal(p0)),
        let z = z / self.params[0];
        // Instr::Add(Reg::X, Arg::Literal(p1)),
        let x = x + self.params[1];
        // Instr::Eql(Reg::X, Arg::VarName(Reg::W)),
        let x = (x == w) as i64;
        // Instr::Eql(Reg::X, Arg::Literal(0)),
        let x = (x == 0) as i64;
        // Instr::Mul(Reg::Y, Arg::Literal(0)),
        let y = 0;
        // Instr::Add(Reg::Y, Arg::Literal(25)),
        let y = y + 25;
        // Instr::Mul(Reg::Y, Arg::VarName(Reg::X)),
        let y = y * x;
        // Instr::Add(Reg::Y, Arg::Literal(1)),
        let y = y + 1;
        // Instr::Mul(Reg::Z, Arg::VarName(Reg::Y)),
        let z = z * y;
        // Instr::Mul(Reg::Y, Arg::Literal(0)),
        let y = 0;
        // Instr::Add(Reg::Y, Arg::VarName(Reg::W)),
        let y = y + w;
        // Instr::Add(Reg::Y, Arg::Literal(p2)),
        let y = y + self.params[2];
        // Instr::Mul(Reg::Y, Arg::VarName(Reg::X)),
        let y = y * x;
        // Instr::Add(Reg::Z, Arg::VarName(Reg::Y)),
        let z = z + y;
        z
    }

    fn eval(&self, z: i64, w: i64) -> i64 /* z */ {
        let expected = self.eval_full(z, w);

        // Instr::Mul(Reg::X, Arg::Literal(0)),
        // Instr::Add(Reg::X, Arg::VarName(Reg::Z)),
        // Instr::Mod(Reg::X, Arg::Literal(26)),
        // Instr::Div(Reg::Z, Arg::Literal(p0)),
        // Instr::Add(Reg::X, Arg::Literal(p1)),
        // Instr::Eql(Reg::X, Arg::VarName(Reg::W)),
        // Instr::Eql(Reg::X, Arg::Literal(0)),
        // Instr::Mul(Reg::Y, Arg::Literal(0)),
        // Instr::Add(Reg::Y, Arg::Literal(25)),
        // Instr::Mul(Reg::Y, Arg::VarName(Reg::X)),
        // Instr::Add(Reg::Y, Arg::Literal(1)),
        // Instr::Mul(Reg::Z, Arg::VarName(Reg::Y)),
        // Instr::Mul(Reg::Y, Arg::Literal(0)),
        // Instr::Add(Reg::Y, Arg::VarName(Reg::W)),
        // Instr::Add(Reg::Y, Arg::Literal(p2)),
        // Instr::Mul(Reg::Y, Arg::VarName(Reg::X)),
        // Instr::Add(Reg::Z, Arg::VarName(Reg::Y)),
        let zd = z / self.params[0];
        let z = if (z % 26 + self.params[1]) != w {
            // Always > 0
            zd * 26 + w + self.params[2]
        } else {
            // println!("XXX {} {}", z, zd);
            zd
        };

        assert_eq!(expected, z);

        z
    }

    fn lower_bound_for_z(&self, z: i64) -> i64 {
        let zd = z / self.params[0];
        let t = zd * 26 + 1 + self.params[1];
        let e = zd;
        cmp::max(0, cmp::min(t, e))
    }
}

struct Blocks {
    blocks: Vec<Block>,
}

impl Blocks {
    fn eval(&self, ws: &[i64]) -> i64 {
        assert_eq!(ws.len(), self.blocks.len());

        let mut z = 0;

        // println!("start");
        for (w, block) in ws.iter().zip(&self.blocks) {
            z = block.eval(z, *w);
            // println!("z = {}", z);
        }

        z
    }

    fn eval_str(&self, input: &str) -> i64 {
        let ws: Vec<i64> = input
            .chars()
            .map(|c| c.to_string().parse().unwrap())
            .collect();
        self.eval(&ws)
    }
}

struct BlocksCalc {
    blocks: Vec<Block>,
    block_lower_bound_matches_by_index_by_z: HashMap<(usize, i64), bool>,

    digits: Vec<i64>,

    progress: usize,
}

impl BlocksCalc {
    fn lower_bound_matches(&mut self, digit: usize, z: i64) -> bool {
        assert!(digit < self.blocks.len());

        if self
            .block_lower_bound_matches_by_index_by_z
            .contains_key(&(digit, z))
        {
            return self.block_lower_bound_matches_by_index_by_z[&(digit, z)];
        }

        let new_z = self.blocks[digit].lower_bound_for_z(z);
        let r = if digit == self.blocks.len() - 1 {
            // println!("last digit new_z {}", new_z);
            new_z <= 0
        } else {
            self.lower_bound_matches(digit + 1, new_z)
        };

        self.block_lower_bound_matches_by_index_by_z
            .insert((digit, z), r);
        r
    }

    fn digits_str(&self) -> String {
        self.digits
            .iter()
            .map(|d| d.to_string())
            .collect::<Vec<String>>()
            .join("")
    }

    fn run_up(&mut self, z: i64) {
        if !self.lower_bound_matches(self.digits.len(), z) {
            // println!("{}", self.digits_str());
            // panic!();
            return;
        }

        for d in (1..=9).rev() {
            self.digits.push(d);
            let new_z = self.blocks[self.digits.len() - 1].eval(z, d);
            if self.digits.len() == self.blocks.len() {
                if self.progress % 10_000_000 == 0 {
                    println!("{} {}", self.digits_str(), new_z);
                }
                assert_ne!(new_z, 0, "at: {}", self.digits_str());
                self.progress += 1;
            } else {
                self.run_up(new_z);
            }
            self.digits.pop().unwrap();
        }
    }

    fn run_down(&mut self, z: i64) {
        self.progress += 1;
        if self.progress % 1000_000 == 0 {
            println!("{}", self.digits_str());
        }

        if !self.lower_bound_matches(self.digits.len(), z) {
            // println!("{}", self.digits_str());
            // panic!();
            return;
        }

        for d in (1..=9) {
            self.digits.push(d);
            let new_z = self.blocks[self.digits.len() - 1].eval(z, d);
            if self.digits.len() == self.blocks.len() {
                // if self.progress % 1 == 0 {
                println!("{} {}", self.digits_str(), new_z);
                // }
                assert_ne!(new_z, 0, "at: {}", self.digits_str());
                self.progress += 1;
            } else {
                self.run_down(new_z);
            }
            self.digits.pop().unwrap();
        }
    }
}

fn run() {
    let prog_orig = Program::parse_file("day24-input.txt");

    println!("program {}:", prog_orig.instrs.len());
    // prog_orig.print();

    let prog = prog_orig.opt().0;
    println!();
    println!(
        "opt {} (from {}):",
        prog.instrs.len(),
        prog_orig.instrs.len()
    );
    // prog.print();
    // let prog = prog_orig;

    println!();

    let z = prog.eval(Input::parse_digits("13579246899999")).get(Reg::Z);
    println!("z = {}", z);
    assert_eq!(
        z,
        prog_orig
            .eval(Input::parse_digits("13579246899999"))
            .get(Reg::Z)
    );

    let z = prog.eval(Input::parse_digits("99999999999999")).get(Reg::Z);
    println!("z = {}", z);
    assert_eq!(
        z,
        prog_orig
            .eval(Input::parse_digits("99999999999999"))
            .get(Reg::Z)
    );

    let mut blocks = Vec::new();
    for instr in &prog_orig.instrs {
        if let Instr::Inp(..) = instr {
            blocks.push(Vec::new());
        }
        blocks.last_mut().unwrap().push(instr.clone());
    }

    let part_sigs: Vec<Vec<Opcode>> = blocks
        .iter()
        .map(|part| part.iter().map(|instr| instr.opcode()).collect())
        .collect::<Vec<_>>();

    let sig = HashSet::<Vec<Opcode>>::from_iter(part_sigs);
    assert_eq!(1, sig.len());
    let sig = sig.into_iter().next().unwrap();

    println!("part len: {}", sig.len());
    println!("sig:      {:?}", sig);

    let blocks = blocks
        .iter()
        .map(|part| Block::new(part))
        .collect::<Vec<_>>();

    assert_eq!(14, blocks.len());

    for block in &blocks {
        println!("block: {:?}", block);
    }

    let blocks = Blocks { blocks };

    assert_eq!(76981240, blocks.eval_str("13579246899999"));
    assert_eq!(4545011352, blocks.eval_str("99999999999999"));

    println!("AAAAAAA");
    println!("{}", blocks.eval_str("99919692496939"));
    println!("AAAAAAA");
    println!("{}", blocks.eval_str("81914111161714"));

    let mut rng = rand::rngs::StdRng::seed_from_u64(1);

    // let mut i: u64 = 0;
    // loop {
    //     let v = rng.gen_range(11111_11111_1111u64..=99999_99999_9999);
    //     let s = v.to_string();
    //     let r = blocks.eval_str(&s);
    //     if i % 100_000 == 0 {
    //         println!("{} {} {}", i, v, r);
    //     }
    //     assert!(r != 0);
    //     i += 1;
    // }

    // for i in (11111_11111_1111u64..=99999_99999_9999).rev() {
    //     let s = i.to_string();
    //     let r = blocks.eval_str(&s);
    //     println!("{} {}", i, r);
    // }

    BlocksCalc {
        blocks: blocks.blocks,
        block_lower_bound_matches_by_index_by_z: HashMap::new(),
        digits: Vec::new(),
        progress: 0,
    }
    .run_down(0);

    // inc_opt(&prog);

    // println!();
    // let expr = prog_orig.to_expr();
    // println!("{}", expr);
    // println!();
    // // expr.print_tree();
    // println!();
    // println!("{}", expr.short());
    //
    // println!();
    // let (a, b) = expr.args();
    // println!(".0 = {}", a.short());
    // println!(".1 = {}", b.short());
    //
    // println!();
    // let (a, b) = a.args();
    // println!(".0.0 = {}", a.short());
    // println!(".0.1 = {}", b.short());
    //
    // let eqz = ExprBool::eq_0(&expr);
    //
    // println!("{}", eqz);
    // println!();
    // println!("eqz:");
    // println!();
    // eqz.print_tree();
    // println!();
    // println!("EOF");

    return;
    // let prog = prog_orig;

    let mut i = 0;
    for n in (0..=99999_99999_9999u64).rev() {
        let input = Input::parse_digits_from_dec(&n.to_string());
        if input.numbers.contains(&0) {
            continue;
        }
        let z = prog.eval(input).get(Reg::Z);
        if z == 0 {
            println!("n = {}", n);
            break;
        }
        if i % 100_000 == 0 {
            println!("i = {}, n = {}", i, n);
        }
        i += 1;
    }
}

fn main() {
    test();

    run();
}
