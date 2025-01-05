use std::collections::HashMap;
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

fn blink_counter(map: HashMap<i64, i64>) -> HashMap<i64, i64> {
    let mut value_map: HashMap<i64, i64> = HashMap::new();

    for (k, v) in map {
        if k == 0 {
            map_edit(&mut value_map, 1, v);
            continue;
        }

        let num_str = k.to_string();
        if num_str.len() % 2 == 0 {
            let a = num_str[0..num_str.len() / 2].parse().unwrap();
            let b = num_str[num_str.len() / 2..].parse().unwrap();
            map_edit(&mut value_map, a, v);
            map_edit(&mut value_map, b, v);
            continue;
        } else {
            let new_key = k * 2024;
            map_edit(&mut value_map, new_key, v);
            continue;
        }
    }
    value_map
}

fn a(data: &Vec<i64>) -> i64 {
    let mut processed: Vec<i64> = data.clone();
    for _ in 0..25 {
        processed = processed.iter().flat_map(|x| blink(x)).collect();
    }
    processed.len() as i64
}

fn b(data: Vec<i64>) -> i64 {
    let mut value_map: HashMap<i64, i64> = HashMap::new();

    // Add values from list
    for val in data {
        map_edit(&mut value_map, val, 1);
    }

    // 75 blinks
    for _ in 0..75 {
        value_map = blink_counter(value_map);
    }

    // Return "len"
    value_map.iter().map(|(_, v)| v).sum()
}

fn map_edit(map: &mut HashMap<i64, i64>, key: i64, val: i64) {
    if map.contains_key(&key) {
        let old_val = map.get(&key).unwrap();
        map.insert(key, old_val + val);
    } else {
        map.insert(key, val);
    }
}

fn main() {
    let data = read_data("data.txt");
    print!("Part one: {}\nPart two: {}\n", a(&data), b(data));
}
