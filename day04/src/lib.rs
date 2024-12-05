pub fn run_a(input: &str) -> i32 {
    let grid = gridify(input);
    let mut sum = horizontal_matches(&grid);
    dbg!("vert");
    sum += vertical_matches(&grid);
    dbg!("diag");
    sum += back_diagonal_matches(&grid);
    dbg!("diag2");
    sum += front_diagonal_matches(&grid);
    sum
}

fn gridify(input: &str) -> Vec<Vec<char>> {
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        let gridline: Vec<char> = line.chars().collect();
        grid.push(gridline);
    }
    grid
}

fn horizontal_matches(grid: &Vec<Vec<char>>) -> i32 {
    let mut sum = 0;
    for y in 0..grid.len() {
        let line = &grid[y];
        let mut x = 0;
        while x + 3 < line.len() {
            if line[x] == 'X' && line[x+1] == 'M' && line[x+2] == 'A' && line[x+3] == 'S' 
            || line[x] == 'S' && line[x+1] == 'A' && line[x+2] == 'M' && line[x+3] == 'X' {
                dbg!(x);
                dbg!(y);
                println!();
                sum += 1;
            }
            x += 1;
        }
    }
    sum
}

fn vertical_matches(grid: &Vec<Vec<char>>) -> i32 {
    let mut sum = 0;
    let mut y = 0;
    while y + 3 < grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == 'X' && grid[y+1][x] == 'M' && grid[y+2][x] == 'A' && grid[y+3][x] == 'S' 
            || grid[y][x] == 'S' && grid[y+1][x] == 'A' && grid[y+2][x] == 'M' && grid[y+3][x] == 'X' {
                dbg!(x);
                dbg!(y);
                println!();
                sum += 1;
            }
        }
        y += 1;
    }
    sum
}

fn back_diagonal_matches(grid: &Vec<Vec<char>>) -> i32 {
    let mut sum = 0;
    let mut y = 0;
    while y + 3 < grid.len() {
        let mut x = 0;
        while x + 3 < grid[y].len() {
            if grid[y][x] == 'X' && grid[y+1][x+1] == 'M' && grid[y+2][x+2] == 'A' && grid[y+3][x+3] == 'S' 
            || grid[y][x] == 'S' && grid[y+1][x+1] == 'A' && grid[y+2][x+2] == 'M' && grid[y+3][x+3] == 'X' {
                dbg!(x);
                dbg!(y);
                println!();
                sum += 1;
            }
            x += 1;
        }
        y += 1;
    }
    sum
}

fn front_diagonal_matches(grid: &Vec<Vec<char>>) -> i32 {
    let mut sum = 0;
    let mut y = 0;
    while y + 3 < grid.len() {
        let mut x = 0;
        while x + 3 < grid[y].len() {
            if grid[y+3][x] == 'X' && grid[y+2][x+1] == 'M' && grid[y+1][x+2] == 'A' && grid[y][x+3] == 'S' 
            || grid[y+3][x] == 'S' && grid[y+2][x+1] == 'A' && grid[y+1][x+2] == 'M' && grid[y][x+3] == 'X' {
                dbg!(x);
                dbg!(y);
                println!();
                sum += 1;
            }
            x += 1;
        }
        y += 1;
    }
    sum
}

pub fn run_b(input: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn a() {
        let result = run_a(&fs::read_to_string("./test.txt").unwrap());
        assert_eq!(result, 18);
    }

    #[test]
    fn b() {
        let result = run_b(&fs::read_to_string("./test.txt").unwrap());
        assert_eq!(result, 9);
    }
}