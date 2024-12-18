use std::collections::HashMap;
use std::fs;

fn extract_rules(input_string: &str) -> HashMap<String, Vec<String>> {
    let rule_list = input_string
        .lines()
        .map(|x| x.split("|").map(|y| y.to_string()).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut rules: HashMap<String, Vec<String>> = HashMap::new();

    for rule in rule_list {
        let key = &rule[0];
        let value = rule[1].to_string();

        match rules.get_mut(key) {
            Some(val) => {
                val.push(value);
            }
            None => {
                rules.insert(key.to_string(), vec![value]);
            }
        }
    }

    rules
}

fn extract_protocol(input_string: &str) -> Vec<Vec<String>> {
    input_string
        .lines()
        .map(|x| x.split(",").map(|y| y.to_string()).collect::<Vec<_>>())
        .collect()
}

fn read_data(file_name: &str) -> (HashMap<String, Vec<String>>, Vec<Vec<String>>) {
    let input_string = fs::read_to_string(file_name).expect("failed to read input file");
    let split_string = input_string.split("\n\n").collect::<Vec<&str>>();

    let rules = extract_rules(&split_string[0]);
    let protocol = extract_protocol(&split_string[1]);

    (rules, protocol)
}

fn a(rules: &HashMap<String, Vec<String>>, protocol: &Vec<Vec<String>>) -> i32 {
    let mut score: i32 = 0;

    for line in protocol {
        let mut valid = true;
        for index in 0..line.len() {
            let current_rules = rules.get(&line[index]);
            if !line[index + 1..line.len()]
                .iter()
                .all(|x| current_rules.unwrap_or(&vec![]).contains(x)) 
            {
                valid = false;
                break;
            }
        }
        
        if valid {
            score += line[line.len()/2].parse::<i32>().unwrap_or(0);
        }
    }

    score
}

fn b() -> i32 {
    0
}

fn main() {
    let (rules, protocol) = read_data("data.txt");
    print!("Part one: {}\nPart two: {}\n", a(&rules, &protocol), b());
}
