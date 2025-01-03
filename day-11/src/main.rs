use std::fs;

fn read_data(file_name: &str) -> Vec<i64> {
    fs::read_to_string(file_name)
        .expect("failed to read input file")
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect()
}

fn blink(num: &i64) -> Vec<i64> {
    if num == &0 {
        return vec![1];
    }

    let num_str = num.to_string();

    if num_str.len() % 2 == 0 {
        return vec![
            num_str[0..num_str.len() / 2].parse().unwrap(),
            num_str[num_str.len() / 2..].parse().unwrap(),
        ];
    } else {
        return vec![num * 2024];
    }
}

fn a(data: &Vec<i64>) -> i64 {
    let mut processed: Vec<i64> = data.clone();
    for _ in 0..25 {
        processed = processed.iter().flat_map(|x| blink(x)).collect();
    }
    processed.len() as i64
}

fn b(_data: &Vec<i64>) -> i64 {
    2
}

fn main() {
    let data = read_data("data.txt");
    print!("Part one: {}\nPart two: {}\n", a(&data), b(&data));
}
