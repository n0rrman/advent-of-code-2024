use std::collections::HashSet;
use std::fs;

fn read_data(file_name: &str) -> String {
    fs::read_to_string(file_name).expect("failed to read input file")
}

fn build_grid(data: &String) -> Vec<Vec<char>> {
    data.lines()
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn calc_trails(idx: usize, grid: &Vec<Vec<char>>) -> i32 {
    let width = grid[0].len();
    let x = idx % width;
    let y = idx / width;

    rec_calc_trails(x, y, 0, &grid).len() as i32
}

fn rec_calc_trails(x: usize, y: usize, val: i32, grid: &Vec<Vec<char>>) -> HashSet<(usize, usize)> {
    let mut set: HashSet<(usize, usize)> = HashSet::new();

    if val == 9 && grid[y][x] as i32 - 48 == 9 {
        set.insert((x, y));
        return set;
    }

    let new_val = val + 1;
    if x > 0 && grid[y][x - 1] as i32 - 48 == new_val {
        set.extend(rec_calc_trails(x - 1, y, new_val, &grid).drain());
    }
    if x < grid[0].len() - 1 && grid[y][x + 1] as i32 - 48 == new_val {
        set.extend(rec_calc_trails(x + 1, y, new_val, &grid).drain());
    }
    if y > 0 && grid[y - 1][x] as i32 - 48 == new_val {
        set.extend(rec_calc_trails(x, y - 1, new_val, &grid).drain());
    }
    if y < grid.len() - 1 && grid[y + 1][x] as i32 - 48 == new_val {
        set.extend(rec_calc_trails(x, y + 1, new_val, &grid).drain());
    }

    return set;
}

fn calc_distinct_trails(idx: usize, grid: &Vec<Vec<char>>) -> i32 {
    let width = grid[0].len();
    let x = idx % width;
    let y = idx / width;

    rec_calc_distinct_trails(x, y, 0, &grid).len() as i32
}

fn rec_calc_distinct_trails(
    x: usize,
    y: usize,
    val: i32,
    grid: &Vec<Vec<char>>,
) -> Vec<(usize, usize)> {
    let mut vec = vec![];

    if val == 9 && grid[y][x] as i32 - 48 == 9 {
        vec.push((x, y));
        return vec;
    }

    let new_val = val + 1;
    if x > 0 && grid[y][x - 1] as i32 - 48 == new_val {
        vec.extend(rec_calc_distinct_trails(x - 1, y, new_val, &grid));
    }
    if x < grid[0].len() - 1 && grid[y][x + 1] as i32 - 48 == new_val {
        vec.extend(rec_calc_distinct_trails(x + 1, y, new_val, &grid));
    }
    if y > 0 && grid[y - 1][x] as i32 - 48 == new_val {
        vec.extend(rec_calc_distinct_trails(x, y - 1, new_val, &grid));
    }
    if y < grid.len() - 1 && grid[y + 1][x] as i32 - 48 == new_val {
        vec.extend(rec_calc_distinct_trails(x, y + 1, new_val, &grid));
    }

    return vec;
}

fn a(data: &String, grid: &Vec<Vec<char>>) -> i32 {
    data.replace("\n", "")
        .chars()
        .enumerate()
        .map(|(i, c)| match c {
            '0' => calc_trails(i, &grid),
            _ => 0,
        })
        .sum()
}

fn b(data: &String, grid: &Vec<Vec<char>>) -> i32 {
    data.replace("\n", "")
        .chars()
        .enumerate()
        .map(|(i, c)| match c {
            '0' => calc_distinct_trails(i, &grid),
            _ => 0,
        })
        .sum()
}

fn main() {
    let data = read_data("data.txt");
    let grid = build_grid(&data);
    print!(
        "Part one: {}\nPart two: {}\n",
        a(&data, &grid),
        b(&data, &grid)
    );
}
