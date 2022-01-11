struct Dice {
    how_many_times: u32,
    next: u32,
}

impl Dice {
    fn next(&mut self) -> u32 {
        self.how_many_times += 1;
        let r = self.next;
        self.next = if self.next == 100 { 1 } else { self.next + 1 };
        r
    }
}

struct Player {
    pos: u32,
    score: u32,
}

impl Player {
    fn turn(&mut self, dice: &mut Dice) {
        let mut sum = dice.next() + dice.next() + dice.next();
        self.pos = ((self.pos - 1) + sum) % 10 + 1;
        self.score += self.pos;
    }
}

fn part1(p1_start: u32, p2_start: u32) {
    println!("p1 start: {}, p2 start: {}", p1_start, p2_start);

    let mut dice = Dice {
        how_many_times: 0,
        next: 1,
    };

    let mut p1 = Player {
        pos: p1_start,
        score: 0,
    };
    let mut p2 = Player {
        pos: p2_start,
        score: 0,
    };

    loop {
        p1.turn(&mut dice);
        if p1.score >= 1000 {
            println!("p1 won");
            println!("p2 score: {}", p2.score);
            println!("dice rolled: {}", dice.how_many_times);
            println!("output: {}", p2.score * dice.how_many_times);
            break;
        }
        p2.turn(&mut dice);
        if p2.score >= 1000 {
            println!("p2 won");
            break;
        }
    }
}

fn main() {
    part1(4, 8);
    part1(8, 10);
}
