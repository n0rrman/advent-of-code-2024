fn right_check(i: usize, string: &Vec<char>, target: &Vec<char>) -> bool {
    if i + (target.len() - 1) >= string.len() {
        return false;
    }

    for x in 0..target.len() {
        if string[i + x] != target[x] {
            return false;
        }
    }
    return true;
}

fn down_check(i: usize, string: &Vec<char>, target: &Vec<char>, width: usize) -> bool {
    if i + (target.len() - 1) * width >= string.len() {
        return false;
    }

    for x in 0..target.len() {
        if string[i + x * width] != target[x] {
            return false;
        }
    }
    return true;
}

fn diag_right_check(i: usize, string: &Vec<char>, target: &Vec<char>, width: usize) -> bool {
    if i + (target.len() - 1) * (width + 1) >= string.len() {
        return false;
    }

    for x in 0..target.len() {
        if string[i + x * (width + 1)] != target[x] {
            return false;
        }
    }
    return true;
}

fn diag_left_check(i: usize, string: &Vec<char>, target: &Vec<char>, width: usize) -> bool {
    if i + (target.len() - 1) * (width - 1) >= string.len() {
        return false;
    }

    for x in 0..target.len() {
        if string[i + x * (width - 1)] != target[x] {
            return false;
        }
    }
    return true;
}

fn xmas_check(i: usize, c: char, string: &Vec<char>, width: usize) -> i32 {
    let mut count = 0;
    let xmas = &vec!['X', 'M', 'A', 'S'];
    let samx = &vec!['S', 'A', 'M', 'X'];

    if c == 'X' {
        if right_check(i, string, xmas) {
            count += 1
        }
        if down_check(i, string, xmas, width) {
            count += 1
        }
        if diag_right_check(i, string, xmas, width) {
            count += 1
        }
        if diag_left_check(i, string, xmas, width) {
            count += 1
        }
    }
    if c == 'S' {
        if right_check(i, string, samx) {
            count += 1
        }
        if down_check(i, string, samx, width) {
            count += 1
        }
        if diag_right_check(i, string, samx, width) {
            count += 1
        }
        if diag_left_check(i, string, samx, width) {
            count += 1
        }
    }

    return count;
}

fn mas_check(i: usize, c: char, string: &Vec<char>, width: usize) -> i32 {
    if i < width - 1                        // First row
        || i > (string.len() - width)       // Last row
        || i % (width ) == 0             // First col
        || i % (width ) == (width - 2)   // Last col
        || i % (width ) == (width - 1)   // Newline col
        || c != 'A'
    // Middle not A
    {
        return 0;
    }

    let mas = &vec!['M', 'A', 'S'];
    let sam = &vec!['S', 'A', 'M'];
    if (diag_left_check(i - width + 1, string, mas, width)
        || diag_left_check(i - width + 1, string, sam, width))
        && (diag_right_check(i - width - 1, string, mas, width)
            || diag_right_check(i - width - 1, string, sam, width))
    {
        return 1;
    }
    return 0;
}

fn a() -> i32 {
    let input_string = String::from_utf8_lossy(include_bytes!("data.txt"));
    let width = input_string.find("\n").unwrap() + 1;
    let input_vec: &Vec<char> = &input_string.chars().collect();

    return input_string
        .chars()
        .enumerate()
        .map(|(i, x)| xmas_check(i, x, &input_vec, width))
        .sum::<i32>();
}

fn b() -> i32 {
    let input_string = String::from_utf8_lossy(include_bytes!("data.txt"));
    let width = input_string.find("\n").unwrap() + 1;
    let input_vec: &Vec<char> = &input_string.chars().collect();

    return input_string
        .chars()
        .enumerate()
        .map(|(i, x)| mas_check(i, x, &input_vec, width))
        .sum::<i32>();
}

fn main() {
    print!("Part one: {}\nPart two: {}\n", a(), b());
}
