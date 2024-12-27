use std::fs;

fn read_data(file_name: &str) -> String {
    fs::read_to_string(file_name)
        .expect("failed to read input file")
        .replace("\n", "")
}

fn a(input: &str) -> i32 {
    let mut values: Vec<i32> = Vec::new();
    let mut iter_value: i32 = 0;
    let mut spaces: bool = false;

    // Create Vec with spaces
    for char in input.chars() {
        let count = char as i32 - 48;

        for _ in 0..count {
            if spaces {
                values.push(-1);
            } else {
                values.push(iter_value);
            }
        }
        if !spaces {
            iter_value += 1;
        }
        spaces = !spaces;
    }

    // Fill spaces
    let mut processed: Vec<i32> = Vec::new();
    while let Some(..) = values.get(0) {
        let val = values.remove(0);
        if val == -1 {
            while let Some(last) = values.last() {
                if *last == -1 {
                    values.pop();
                } else {
                    break;
                }
            }
            if let Some(replacement) = values.pop() {
                processed.push(replacement);
            }
        } else {
            processed.push(val);
        }
    }

    // Calculate sum
    processed
        .iter()
        .enumerate()
        .map(|(i, x)| i as i32 * x)
        .sum()
}

fn b(input: &str) -> i32 {
    0
}

fn main() {
    let data = read_data("data.txt");
    print!("Part one: {}\nPart two: {}\n", a(&data), b(&data));
}
