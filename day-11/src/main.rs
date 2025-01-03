use std::fs;

fn read_data(file_name: &str) -> Vec<i32> {
    fs::read_to_string(file_name)
        .expect("failed to read input file")
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect()
}

fn a(data: &Vec<i32>) -> i32 {
    let processed: Vec<i32> = data
        .iter()
        .flat_map(|x| {
            let x_string = x.to_string();
            match x_string {
            "0" => vec![1],
            x_string.len() % 2 == 0 => vec![1, 7],
            _ => vec![x * 2024],
        }})
        .collect();
    println!("{:#?}", processed);
    processed.len() as i32
}

fn b(_data: &Vec<i32>) -> i32 {
    2
}

fn main() {
    let data = read_data("test_data.txt");
    print!("Part one: {}\nPart two: {}\n", a(&data), b(&data));
}
