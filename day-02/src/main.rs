
fn reactor_conditions(list: &Vec<i32>) -> bool {
    let not_empty: bool = list.len() > 0;
    let strict_ascending = list.windows(2).all(|x| x[0] < x[1]);
    let strict_descending = list.windows(2).all(|x| x[0] > x[1]);
    let max_distance = list.windows(2).all(|x| (x[0] - x[1]).abs() <= 3 );
    return not_empty && max_distance && (strict_ascending || strict_descending);
}

fn relaxed_conditions(list: &Vec<i32>) -> bool {
    for i in 0..list.len() {
        let mut new_list = list.clone();
        new_list.remove(i);
        if reactor_conditions(&new_list) {
            return true
        }
    }
    return false
}

fn scan_reports(conditions: fn(&Vec<i32>) -> bool) -> i32 {
    let input_string = String::from_utf8_lossy(include_bytes!("data.txt"));
    let count = input_string
        .lines()
        .map(|x| x.split_whitespace()
            .filter_map(|y| y.parse::<i32>().ok())
            .collect::<Vec<i32>>()
        )
        .filter(|x| conditions(&x))
        .count();
    return count as i32;
}

fn a() -> i32 {
    return scan_reports(reactor_conditions)
}


fn b() -> i32 {
    return scan_reports(relaxed_conditions)
}


fn main() {
    print!("Part one: {}\nPart two: {}\n", a(), b());
}

