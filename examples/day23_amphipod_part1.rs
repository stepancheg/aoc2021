#![feature(map_first_last)]

use std::collections::BTreeSet;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
enum Amphipod {
    A,
    B,
    C,
    D,
}

impl Amphipod {
    const ALL: [Amphipod; 4] = [Amphipod::A, Amphipod::B, Amphipod::C, Amphipod::D];

    fn room_i(&self) -> usize {
        match self {
            Amphipod::A => 0,
            Amphipod::B => 1,
            Amphipod::C => 2,
            Amphipod::D => 3,
        }
    }

    // Per step.
    fn energy(&self) -> u64 {
        match self {
            Amphipod::A => 1,
            Amphipod::B => 10,
            Amphipod::C => 100,
            Amphipod::D => 1000,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct RoomInit {
    amphipods: [Amphipod; 2],
}

struct Input {
    rooms: [RoomInit; 4],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Room {
    amphipods: [Option<Amphipod>; 2],
}

impl Room {
    const COUNT: usize = 4;

    fn new(init: RoomInit) -> Self {
        Room {
            amphipods: [Some(init.amphipods[0]), Some(init.amphipods[1])],
        }
    }

    fn is_full_of(&self, amphipod: Amphipod) -> bool {
        self.amphipods
            .iter()
            .all(|&amphipod_opt| amphipod_opt == Some(amphipod))
    }

    fn is_full(&self) -> bool {
        self.amphipods.iter().all(|amphipod| amphipod.is_some())
    }

    fn is_empty(&self) -> bool {
        self.amphipods.iter().all(|amphipod| amphipod.is_none())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
struct Hallway {
    places: [Option<Amphipod>; Hallway::LEN],
}

impl Hallway {
    const LEN: usize = 11;

    fn room_to_hallway(room_i: Amphipod) -> usize {
        let hallway = room_i.room_i() * 2 + 2;
        assert!(hallway < Hallway::LEN);
        hallway
    }

    fn can_stay_at(i: usize) -> bool {
        assert!(i < Hallway::LEN);
        i != 2 && i != 4 && i != 6 && i != 8
    }

    fn can_move_to_from_room(&self, target_i: usize, room_i: Amphipod) -> Option<usize> {
        assert!(target_i < Hallway::LEN);
        let room_hallway_i = Hallway::room_to_hallway(room_i);
        if !Hallway::can_stay_at(target_i) {
            return None;
        }
        if target_i < room_hallway_i {
            for i in (target_i..=room_hallway_i).rev() {
                if self.places[i].is_some() {
                    return None;
                }
            }
            Some(room_hallway_i - target_i)
        } else if room_hallway_i < target_i {
            for i in room_hallway_i..=target_i {
                if self.places[i].is_some() {
                    return None;
                }
            }
            Some(target_i - room_hallway_i)
        } else {
            unreachable!()
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
struct State {
    rooms: [Room; Room::COUNT],
    hallway: Hallway,
}

impl State {
    fn print(&self) {
        for _ in 0..Hallway::LEN + 2 {
            print!("#");
        }
        println!();
        print!("#");
        for hallway_i in 0..Hallway::LEN {
            if let Some(an) = self.hallway.places[hallway_i] {
                print!("{:?}", an);
            } else {
                print!(".");
            }
        }
        print!("#");
        println!();
        for i in (0..=1).rev() {
            print!("{}", if i == 0 { "  #" } else { "###" });
            for room_i in 0..Room::COUNT {
                if let Some(an) = self.rooms[room_i].amphipods[i] {
                    print!("{:?}", an);
                } else {
                    print!(".");
                }
                print!("#");
            }
            print!("{}", if i == 0 { "" } else { "##" });
            println!();
        }
        println!("  #########");
    }

    fn is_final(&self) -> bool {
        self.rooms[0].is_full_of(Amphipod::A)
            && self.rooms[1].is_full_of(Amphipod::B)
            && self.rooms[2].is_full_of(Amphipod::C)
            && self.rooms[3].is_full_of(Amphipod::D)
    }

    fn next(&self) -> Vec<(State, u64)> {
        let mut next = Vec::new();
        for room_i in Amphipod::ALL {
            let room = &self.rooms[room_i.room_i()];
            if room.is_full_of(room_i) {
                continue;
            }
            if let Some(an) = room.amphipods[1] {
                assert!(room.amphipods[0].is_some());
                for i in 0..Hallway::LEN {
                    if let Some(len) = self.hallway.can_move_to_from_room(i, room_i) {
                        let len = len + 1;
                        let mut new_state = self.clone();
                        new_state.hallway.places[i] = Some(an);
                        new_state.rooms[room_i.room_i()].amphipods[1] = None;
                        next.push((new_state, len as u64 * an.energy()));
                    }
                }
            } else if let Some(an) = room.amphipods[0] {
                assert!(room.amphipods[1].is_none());
                for i in 0..Hallway::LEN {
                    if let Some(len) = self.hallway.can_move_to_from_room(i, room_i) {
                        let len = len + 2;
                        let mut new_state = self.clone();
                        new_state.hallway.places[i] = Some(an);
                        new_state.rooms[room_i.room_i()].amphipods[0] = None;
                        next.push((new_state, len as u64 * an.energy()));
                    }
                }
            }
        }
        'next_hallway: for hallway_i in 0..Hallway::LEN {
            if let Some(an) = self.hallway.places[hallway_i] {
                let target_room = &self.rooms[an.room_i()];
                let len;
                let room_pos;
                if target_room.is_empty() {
                    room_pos = 0;
                } else if target_room.is_full() {
                    continue;
                } else {
                    assert!(target_room.amphipods[1].is_none());
                    let an_0 = target_room.amphipods[0].unwrap();
                    if an_0 != an {
                        continue;
                    }
                    room_pos = 1;
                }

                let room_hallway_i = Hallway::room_to_hallway(an);
                if hallway_i < room_hallway_i {
                    for i in hallway_i + 1..=room_hallway_i {
                        if self.hallway.places[i].is_some() {
                            continue 'next_hallway;
                        }
                    }
                } else if hallway_i > room_hallway_i {
                    for i in room_hallway_i..hallway_i {
                        if self.hallway.places[i].is_some() {
                            continue 'next_hallway;
                        }
                    }
                } else {
                    unreachable!();
                }

                let mut new_state = self.clone();
                new_state.hallway.places[hallway_i] = None;
                new_state.rooms[an.room_i()].amphipods[room_pos] = Some(an);

                len = ((hallway_i as i64) - (Hallway::room_to_hallway(an) as i64)).abs()
                    + (2 - room_pos as i64);

                next.push((new_state, an.energy() * (len as u64)));
            }
        }

        // println!("for state:");
        // self.print();
        // println!();
        // println!("next states:");
        // for (state, en) in &next {
        //     println!("en {}:", en);
        //     state.print();
        // }

        next
    }
}

#[derive(Default)]
struct Solution {}

impl Solution {
    fn solve(&mut self, init: &State) -> u64 {
        // init.print();
        let mut visited: HashMap<State, (u64, State)> = HashMap::new();
        let mut border: BTreeSet<(u64, State)> = BTreeSet::new();

        border.insert((0, init.clone()));
        loop {
            let (en, state) = border.pop_first().unwrap();
            if state.is_final() {
                println!();

                let mut path = Vec::new();
                let mut state = state;
                path.push(state.clone());
                while &state != init {
                    state = visited.get(&state).unwrap().1.clone();
                    path.push(state);
                }

                println!("path:");
                for state in path.iter().rev() {
                    println!();
                    state.print();
                }
                return en;
            }

            // println!("iter {} {} {}", border.len(), visited.len(), en);

            for (next_state, step_en) in state.next() {
                assert!(step_en > 0);
                match visited.get(&next_state) {
                    None => {
                        visited.insert(next_state.clone(), (en + step_en, state.clone()));
                        border.insert((en + step_en, next_state));
                    }
                    Some(&(visited_en, _)) if visited_en >= en + step_en => {
                        visited.insert(next_state.clone(), (en + step_en, state.clone()));
                        border.insert((en + step_en, next_state));
                    }
                    Some(_) => {}
                }
            }
        }
    }
}

fn run(input: &Input) {
    let mut counts = HashMap::new();
    for room in &input.rooms {
        for amphipod in &room.amphipods {
            *counts.entry(amphipod).or_insert(0) += 1;
        }
    }
    for amphipod in &Amphipod::ALL {
        assert_eq!(2, counts[&amphipod]);
    }
    let energy = Solution::default().solve(&State {
        rooms: input.rooms.map(|room| Room::new(room)),
        hallway: Hallway { places: [None; 11] },
    });
    println!("min energy: {}", energy);
}

fn main() {
    println!("Test");
    run(&Input {
        rooms: [
            RoomInit {
                amphipods: [Amphipod::A, Amphipod::B],
            },
            RoomInit {
                amphipods: [Amphipod::D, Amphipod::C],
            },
            RoomInit {
                amphipods: [Amphipod::C, Amphipod::B],
            },
            RoomInit {
                amphipods: [Amphipod::A, Amphipod::D],
            },
        ],
    });
    println!("Real");
    run(&Input {
        rooms: [
            RoomInit {
                amphipods: [Amphipod::D, Amphipod::C],
            },
            RoomInit {
                amphipods: [Amphipod::A, Amphipod::C],
            },
            RoomInit {
                amphipods: [Amphipod::B, Amphipod::B],
            },
            RoomInit {
                amphipods: [Amphipod::A, Amphipod::D],
            },
        ],
    });
}
