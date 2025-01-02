use std::fs;

fn read_data(file_name: &str) -> Vec<i32> {
    fs::read_to_string(file_name).expect("failed to read input file");
    vec![]
}

fn a(_data: &Vec<i32>) -> i32 {
    1
}

fn b(_data: &Vec<i32>) -> i32 {
    2
}

fn main() {
    let data = read_data("data.txt");
    print!("Part one: {}\nPart two: {}\n", a(&data), b(&data));
}
