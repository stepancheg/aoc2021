use std::collections::HashMap;
use std::ops::Add;

#[derive(Eq, PartialEq, Debug, Hash, Clone, Copy)]
struct Player {
    pos: u32,
    score: u32,
}

#[derive(Eq, PartialEq, Debug, Hash, Clone, Copy)]
struct GameState {
    p1: Player,
    p2: Player,
}

impl GameState {
    fn flip(&self) -> GameState {
        GameState {
            p1: self.p2,
            p2: self.p1,
        }
    }
}

#[derive(Copy, Clone, Debug, Default)]
struct Wins {
    p1: u64,
    p2: u64,
}

impl Add for Wins {
    type Output = Wins;

    fn add(self, rhs: Wins) -> Wins {
        Wins {
            p1: self.p1 + rhs.p1,
            p2: self.p2 + rhs.p2,
        }
    }
}

impl Wins {
    fn flip(&self) -> Wins {
        Wins {
            p1: self.p2,
            p2: self.p1,
        }
    }
}

struct Solution {
    memo: HashMap<GameState, Wins>,
}

impl Solution {
    // 111 -> 3
    // 112 -> 4
    // 113 -> 5
    // 121 -> 4
    // 122 -> 5
    // 123 -> 6
    // 131 -> 5
    // 132 -> 6
    // 133 -> 7
    // 211 -> 4
    // 212 -> 5
    // 213 -> 6
    // 221 -> 5
    // 222 -> 6
    // 223 -> 7
    // 231 -> 6
    // 232 -> 7
    // 233 -> 8
    // 311 -> 5
    // 312 -> 6
    // 313 -> 7
    // 321 -> 6
    // 322 -> 7
    // 323 -> 8
    // 331 -> 7
    // 332 -> 8
    // 333 -> 9
    const DICE_ROLLS: [u32; 27] = [
        3, 4, 5, 4, 5, 6, 5, 6, 7, 4, 5, 6, 5, 6, 7, 6, 7, 8, 5, 6, 7, 6, 7, 8, 7, 8, 9,
    ];

    fn solve(&mut self, game: &GameState) -> Wins {
        if self.memo.contains_key(game) {
            return *self.memo.get(game).unwrap();
        }

        let mut wins = Wins::default();

        for roll in Self::DICE_ROLLS {
            let mut game = game.clone();
            assert!(game.p1.score < 21);
            assert!(game.p2.score < 21);

            game.p1.pos = (game.p1.pos - 1 + roll) % 10 + 1;
            game.p1.score += game.p1.pos;
            if game.p1.score >= 21 {
                wins.p1 += 1;
            } else {
                wins = wins + self.solve(&game.flip()).flip();
            }
        }

        self.memo.insert(game.clone(), wins);
        self.memo.get(&game).unwrap().clone()
    }
}

fn part2(p1_start: u32, p2_start: u32) {
    println!("p1 start: {}, p2 start: {}", p1_start, p2_start);
    let mut solution = Solution {
        memo: HashMap::new(),
    };
    let r = solution.solve(&GameState {
        p1: Player {
            pos: p1_start,
            score: 0,
        },
        p2: Player {
            pos: p2_start,
            score: 0,
        },
    });
    println!("{:?}", r);
}

fn main() {
    part2(4, 8);
    part2(8, 10);
}
