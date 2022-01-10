use std::cmp;
use std::collections::HashSet;

struct Input {
    x_range: (i64, i64),
    y_range: (i64, i64),
}

const INPUTS: [Input; 2] = [
    Input {
        x_range: (20, 30),
        y_range: (-10, -5),
    },
    Input {
        x_range: (94, 151),
        y_range: (-156, -103),
    },
];

fn part1(input: &Input) {
    let mut total_max_y = i64::MIN;
    let mut uniq_init_velocities = HashSet::new();

    for v_x_init in 1..=input.x_range.1 {
        println!("v_x: {}", v_x_init);
        for v_y_init in input.y_range.0..10000 {
            // println!("v_y: {}", v_y);
            let mut v_x = v_x_init;
            let mut v_y = v_y_init;

            let mut x = 0;
            let mut y = 0;

            let mut max_y = y;

            while x <= input.x_range.1 {
                // println!("{} {}", x, y);

                max_y = cmp::max(max_y, y);

                if x >= input.x_range.0
                    && x <= input.x_range.1
                    && y >= input.y_range.0
                    && y <= input.y_range.1
                {
                    total_max_y = cmp::max(total_max_y, max_y);
                    uniq_init_velocities.insert((v_x_init, v_y_init));
                    break;
                }

                if y < input.y_range.0 {
                    break;
                }

                x += v_x;
                y += v_y;
                v_x = if v_x > 0 {
                    v_x - 1
                } else if v_x < 0 {
                    v_x + 1
                } else {
                    v_x
                };
                v_y -= 1;
            }
        }
    }
    println!("{}", total_max_y);
    println!("{}", uniq_init_velocities.len());
    assert!(total_max_y == 45 || total_max_y == 12090);
    assert!(uniq_init_velocities.len() == 112 || uniq_init_velocities.len() == 5059);
}

fn main() {
    println!("Part 1");
    for (i, input) in INPUTS.iter().enumerate() {
        println!("{}", i);
        part1(input);
    }
}
