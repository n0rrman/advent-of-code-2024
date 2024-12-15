use std::collections::HashMap;

fn read_data() -> (Vec<(str, str)>, Vec<Vec<str>>) {
    let input_string = String::from_utf8_lossy(include_bytes!("test_data.txt"));

    let split = input_string.split("\n\n");

    let rules = split.clone().collect::<Vec<_>>()[0]
        .lines()
        .map(|x| {
            let rule = x.split("|").collect::<Vec<&str>>();
            (&rule[0], &rule[1])
        })
        .collect::<Vec<_>>();

    let vals = &split.collect::<Vec<&str>>()[1]
        .lines()
        .map(|x| x.split(",").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    (rules, vals)
}

fn a(rules: &Vec<(String, String)>, vals: &Vec<Vec<String>>) -> i32 {
    println!("{:?}", rules);
    println!("{:?}", vals);

    let _rule_map: HashMap<String, Vec<String>>;

    for val in vals {
        let max_len = val.len();
        for i in 0..max_len {
            print!("{:?} ", val[i]);
        }
        println!("middle: {:?}", val[max_len / 2]);
    }

    0
}

fn b() -> i32 {
    0
}

fn main() {
    let (rules, vals) = read_data();
    print!("Part one: {}\nPart two: {}\n", a(&rules, &vals), b());
}
