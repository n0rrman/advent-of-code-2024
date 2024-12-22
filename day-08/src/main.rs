use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

fn read_data(file_name: &str) -> (HashMap<(usize, usize), char>, (isize, isize)) {
    let input_string = fs::read_to_string(file_name).expect("failed to read input file");
    let width = input_string.find("\n").unwrap_or(0) as isize;
    let height = (input_string.replace("\n", "").len() as isize) / width;

    let mut x: usize = 0;
    let mut y: usize = 0;

    let mut data = HashMap::new();

    for char in input_string.chars() {
        match char {
            '\n' => {
                x = 0;
                y += 1;
            }
            '.' => {
                x += 1;
            }
            _ => {
                data.insert((x, y), char);
                x += 1;
            }
        }
    }

    (data, (width, height))
}

fn a(data: &HashMap<(usize, usize), char>, size: &(isize, isize)) -> i32 {
    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();

    for a in data {
        for b in data {
            if a.1 == b.1 {
                let a_x = a.0 .0 as isize;
                let a_y = a.0 .1 as isize;
                let b_x = b.0 .0 as isize;
                let b_y = b.0 .1 as isize;

                let antinode_x = a_x + (a_x - b_x);
                let antinode_y = a_y + (a_y - b_y);

                if antinode_x < 0
                    || antinode_y < 0
                    || antinode_x >= size.0
                    || antinode_y >= size.1
                    || a_x == b_x
                    || a_y == b_y
                {
                    continue;
                }

                antinodes.insert((antinode_x as usize, antinode_y as usize));
            }
        }
    }
    antinodes.len() as i32
}

fn b() -> i32 {
    1
}

fn main() {
    let (data, size) = read_data("data.txt");
    print!("Part one: {}\nPart two: {}\n", a(&data, &size), b());
}
