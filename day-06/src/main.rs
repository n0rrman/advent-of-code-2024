use std::collections::HashSet;
use std::fs;

fn build_grid(file_name: &str) -> Vec<Vec<char>> {
    let input_string = fs::read_to_string(file_name).expect("failed to read input file");

    input_string
        .lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>()
}

fn find_start(grid: &Vec<Vec<char>>) -> (usize, usize) {
    let mut y = 0;
    let mut x = 0;
    for row in grid {
        x = 0;
        for val in row {
            if *val == '^' {
                return (x, y);
            }
            x += 1;
        }
        y += 1;
    }
    (x, y)
}

fn count_visited(grid: &Vec<Vec<char>>) -> i32 {
    grid.iter()
        .map(|row| {
            row.iter()
                .map(|x| match x {
                    'X' => 1,
                    _ => 0,
                })
                .sum::<i32>()
        })
        .sum()
}

fn traverse_grid(grid: &mut Vec<Vec<char>>) -> Option<&Vec<Vec<char>>> {
    let mut end_reached = false;
    let mut direction = 'N';
    let mut visited: HashSet<(usize, usize, char)> = HashSet::new();

    let (mut x, mut y) = find_start(&grid);

    while !end_reached {
        if visited.contains(&(y, x, direction)) {
            return None;
        }

        grid[y][x] = 'X';
        visited.insert((y, x, direction));

        match direction {
            'N' => {
                if y == 0 {
                    end_reached = true;
                } else if grid[y - 1][x] == '#' {
                    direction = 'E';
                } else {
                    y -= 1;
                }
            }
            'E' => {
                if x == grid[0].len() - 1 {
                    end_reached = true;
                } else if grid[y][x + 1] == '#' {
                    direction = 'S';
                } else {
                    x += 1;
                }
            }
            'S' => {
                if y == grid.len() - 1 {
                    end_reached = true;
                } else if grid[y + 1][x] == '#' {
                    direction = 'W';
                } else {
                    y += 1;
                }
            }

            'W' => {
                if x == 0 {
                    end_reached = true;
                } else if grid[y][x - 1] == '#' {
                    direction = 'N';
                } else {
                    x -= 1;
                }
            }
            _ => {}
        }
    }

    Some(grid)
}

fn a(grid: &mut Vec<Vec<char>>) -> i32 {
    let visited = traverse_grid(grid).unwrap();
    count_visited(visited)
}

fn b(grid: &mut Vec<Vec<char>>) -> i32 {
    let _visited = traverse_grid(grid).unwrap();
    0
}

fn main() {
    let grid = build_grid("test_data.txt");
    print!(
        "Part one: {}\nPart two: {}\n",
        a(&mut grid.clone()),
        b(&mut grid.clone()),
    );
}
