use regex::Regex;

fn calc_sum(input_string: &str) -> i32 {
    let re = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)").unwrap();
    return re
        .find_iter(input_string)
        .map(|x| x.as_str())
        .map(|x| &x[4..(x.len()-1)])
        .map(|x| {
            let vals = x.split(",").collect::<Vec<&str>>();
            return vals[0].parse::<i32>().unwrap() * vals[1].parse::<i32>().unwrap();
        })
        .sum::<i32>();
}

fn a() -> i32 {
    let input_string = String::from_utf8_lossy(include_bytes!("data.txt"));
    return calc_sum(&input_string)
}

fn b() -> i32 {
    let disabled = Regex::new(r"don't\(\)(.*?)do\(\)").unwrap();
    let start = String::from("do()");
    let end = String::from("don't()");
    let input_string = String::from_utf8_lossy(include_bytes!("data.txt"))
        .replace("\n", "x");
    let full_string = start + &input_string + &end;
    let new_string = disabled.replace_all(&full_string, "") ;
    
    return calc_sum(&new_string);
}


fn main() {
    print!("Part one: {}\nPart two: {}\n", a(), b());
}

