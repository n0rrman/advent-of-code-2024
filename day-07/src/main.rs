use std::fs;

fn read_data(file_name: &str) -> Vec<Vec<i64>> {
    let input_string = fs::read_to_string(file_name).expect("failed to read input file");

    input_string
        .replace(":", "")
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|val| val.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<_>>()
}

fn check_line(target: &i64, current: &i64, line: &[i64]) -> bool {
    if line.len() == 0 {
        return target == current;
    }

    check_line(target, &(current * line[0]), &line[1..])
        || check_line(target, &(current + line[0]), &line[1..])
}

fn a(data: &Vec<Vec<i64>>) -> i64 {
    let mut calibration_res = 0;

    for line in data {
        if check_line(&line[0], &line[1], &line[2..]) {
            calibration_res += line[0];
        }
    }

    calibration_res
}
fn b() -> i64 {
    0
}

fn main() {
    let data = read_data("data.txt");
    print!("Part one: {}\nPart two: {}\n", a(&data), b(),);
}
