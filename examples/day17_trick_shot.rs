use std::cmp;

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

    for v_x in 1..=input.x_range.1 {
        println!("v_x: {}", v_x);
        for v_y in input.y_range.0..10000 {
            // println!("v_y: {}", v_y);
            let mut v_x = v_x;
            let mut v_y = v_y;

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
    assert!(total_max_y == 45 || total_max_y == 12090);
}

fn main() {
    println!("Part 1");
    for (i, input) in INPUTS.iter().enumerate() {
        println!("{}", i);
        part1(input);
    }
}
