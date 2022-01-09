const TEST_INPUT: &str = "3,4,3,1,2";
const INPUT: &str = "3,5,4,1,2,1,5,5,1,1,1,1,4,1,4,5,4,5,1,3,1,1,1,4,1,1,3,1,1,5,3,1,1,3,1,3,1,1,1,4,1,2,5,3,1,4,2,3,1,1,2,1,1,1,4,1,1,1,1,2,1,1,1,3,1,1,4,1,4,1,5,1,4,2,1,1,5,4,4,4,1,4,1,1,1,1,3,1,5,1,4,5,3,1,4,1,5,2,2,5,1,3,2,2,5,4,2,3,4,1,2,1,1,2,1,1,5,4,1,1,1,1,3,1,5,4,1,5,1,1,4,3,4,3,1,5,1,1,2,1,1,5,3,1,1,1,1,1,5,1,1,1,1,1,1,1,2,2,5,5,1,2,1,2,1,1,5,1,3,1,5,2,1,4,1,5,3,1,1,1,2,1,3,1,4,4,1,1,5,1,1,4,1,4,2,3,5,2,5,1,3,1,2,1,4,1,1,1,1,2,1,4,1,3,4,1,1,1,1,1,1,1,2,1,5,1,1,1,1,2,3,1,1,2,3,1,1,3,1,1,3,1,3,1,3,3,1,1,2,1,3,2,3,1,1,3,5,1,1,5,5,1,2,1,2,2,1,1,1,5,3,1,1,3,5,1,3,1,5,3,4,2,3,2,1,3,1,1,3,4,2,1,1,3,1,1,1,1,1,1";

#[derive(Default)]
struct State {
    count_by_day: [u64; 9],
}

impl State {
    fn count(&self) -> u64 {
        self.count_by_day.iter().map(|x| *x as u64).sum()
    }
}

fn update_state(state: &State) -> State {
    let mut r = State::default();
    for (x, &count) in state.count_by_day.iter().enumerate() {
        if x != 0 {
            r.count_by_day[x - 1] += count;
        } else {
            r.count_by_day[6] += count;
            r.count_by_day[8] += count;
        }
    }
    r
}

fn run(input: &str) {
    let state_raw = input
        .split(",")
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    let mut state = State::default();
    for &x in &state_raw {
        state.count_by_day[x as usize] += 1;
    }

    for _ in 0..80 {
        state = update_state(&state);
    }
    println!("day 80: {}", state.count());
    for _ in 80..256 {
        state = update_state(&state);
    }
    println!("day 256: {}", state.count());
}

fn main() {
    run(TEST_INPUT);
    run(INPUT);
}
