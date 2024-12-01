fn get_lists() -> (Vec<i32>, Vec<i32>) {
    let input_string = String::from_utf8_lossy(include_bytes!("data.txt"));
    let all: Vec<i32> = input_string.split_whitespace()
            .filter_map(|val| val.parse::<i32>().ok())
            .collect();

    let list_a: Vec<i32> = all.iter()
            .step_by(2)
            .cloned()
            .collect();

    let list_b: Vec<i32> = all.iter()
            .skip(1)
            .step_by(2)
            .cloned()
            .collect();
    return (list_a, list_b)
}


fn a() -> i32 {
    let lists = get_lists();
    let mut list_a = lists.0;
    let mut list_b = lists.1;
    list_a.sort();
    list_b.sort();

    return list_a.iter()
        .zip(list_b.iter())
        .map(|(a,b)| (a - b).abs())
        .sum::<i32>();
}


fn b() -> i32 {
    let lists = get_lists();
    let list_a = lists.0;
    let list_b = lists.1;

    return list_a.iter()
        .map(|&x| list_b.iter().filter(|&&y| x == y).count() as i32 * x)
        .sum::<i32>();
}


fn main() {
    print!("Part one: {}\nPart two: {}\n", a(), b());
}

