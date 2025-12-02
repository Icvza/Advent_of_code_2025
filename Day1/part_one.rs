use std::fs;

fn main() {
    let data = fs::read_to_string("input.txt").expect("Unable to read file");
    let mut total: i32 = 50;

    let mut seen_end: i32 = 0;
    let mut seen_all: i32 = 0;

    for token in data.split_whitespace() {
        let mut chars = token.chars();

        if let Some(direction) = chars.next() {
            let rest: String = chars.collect();
            let distance = rest.parse::<i32>().expect("Invalid distance");

            if direction == 'L' {
                let steps_to_zero = if total == 0 { 100 } else { total };
                if distance >= steps_to_zero {
                    seen_all += 1 + (distance - steps_to_zero) / 100;
                }
                total -= distance;
            } else if direction == 'R' {
                let steps_to_zero = if total == 0 { 100 } else { 100 - total };
                if distance >= steps_to_zero {
                    seen_all += 1 + (distance - steps_to_zero) / 100;
                }
                total += distance;
            } else {
                panic!("Unexpected direction");
            }

            total = ((total % 100) + 100) % 100;

            if total == 0 {
                seen_end += 1;
            }
        }
    }
    println!("Part 1: {}", seen_end);
    println!("Part 2: {}", seen_all);
}
