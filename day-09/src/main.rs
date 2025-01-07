use std::{fs, ops::IndexMut};

fn read_data(file_name: &str) -> String {
    fs::read_to_string(file_name)
        .expect("failed to read input file")
        .replace("\n", "")
}

fn a(input: &str) -> i64 {
    let mut values: Vec<i64> = Vec::new();
    let mut iter_value: i64 = 0;
    let mut spaces: bool = false;

    // Create Vec with spaces
    for char in input.chars() {
        let count = char as i64 - 48;

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
    let mut processed: Vec<i64> = Vec::new();
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
        .map(|(i, x)| i as i64 * x)
        .sum()
}

fn b(input: &str) -> i64 {
    // Values as vector
    let mut counter: i64 = -1;
    let values = input
        .chars()
        .enumerate()
        .map(|(i, v)| {
            if i % 2 == 0 {
                counter += 1;
                return (counter, v as i64 - 48);
            } else {
                return (-1, v as i64 - 48);
            }
        })
        .collect::<Vec<(i64, i64)>>();

    // Fill -1
    //println!("{:?}", values);
    //let processed = values.iter().collect::<Vec<_>>();
    let mut processed = values.clone();
    let mut end = processed.len() - 1;
    let mut start = 0;
    //for _ in 0..2 {

    while end > 1 {
        //println!("{}", end);
        while processed.get(end).unwrap_or(&(0, 0)).0 == -1 {
            end -= 1;
        }
        while processed.get(start).unwrap().0 != -1 {
            start += 1;
        }

        let start_val = processed.get(start).unwrap();
        let end_val = processed.get(end).unwrap();

        let rest = start_val.1 - end_val.1;

        if rest >= 0 {
            //println!(
            //    "{:?} {:?} {} {} {} ",
            //    processed.get(start).unwrap(),
            //    processed.get(end).unwrap(),
            //    rest,
            //    start,
            //    end
            //);
            *processed.index_mut(start) = processed.get(end).unwrap().clone();
            //*processed.index_mut(start) = (processed.get(end).unwrap().0, rest);
            *processed.index_mut(end) = (-1, processed.get(end).unwrap().1);
            start += 1;
            processed.insert(start, (-1, rest));

            //println!("{:?}\n", processed);
        } else {
            start += 1;
        }

        if start >= end {
            start = 0;
            end -= 1;
        }
    }

    println!(
        "{:?}",
        processed.iter().filter(|&x| x.1 != 0).collect::<Vec<_>>()
    );

    // Calculate sum
    processed
        .iter()
        .flat_map(|(k, v)| {
            return vec![k; *v as usize];
        })
        .enumerate()
        .map(|(i, var)| {
            if *var == -1 {
                return 0;
            } else {
                return *var * i as i64;
            }
        })
        .sum()
}

fn main() {
    let data = read_data("data.txt");
    print!("Part one: {}\nPart two: {}\n", a(&data), b(&data));
}
