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

fn calc_trails(_idx: usize, grid: &Vec<Vec<char>>) -> i32 {
    let width = grid[0].len();
    rec_calc_trails().len() as i32

}

fn rec_calc_trails() -> HashSet<(usize, usize)> {
    let mut set: HashSet<(usize, usize)> = HashSet::new();
    set.insert((0, 0));
    return set;
}

fn a(data: &String, grid: &Vec<Vec<char>>) -> i32 {
    data.chars()
        .enumerate()
        .map(|(i, c)| match c {
            '0' => calc_trails(i, &grid),
            _ => 0,
        })
        .sum()
}

fn b(_data: &String, grid: &Vec<Vec<char>>) -> i32 {
    grid.iter().for_each(|x| println!("{:?}", x));
    0
}

fn main() {
    let data = read_data("test_data.txt");
    let grid = build_grid(&data);
    print!(
        "Part one: {}\nPart two: {}\n",
        a(&data, &grid),
        b(&data, &grid)
    );
}
