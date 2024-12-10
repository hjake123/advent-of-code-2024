type Grid = Vec<Vec<u32>>;

fn parse(input: &str) -> Grid {
    let mut grid: Grid = Vec::new();
    for line in input.lines() {
        let mut row: Vec<u32> = Vec::new();
        for ch in line.chars() {
            row.push(ch.to_digit(10).unwrap());
        }
        grid.push(row);
    }
    grid
}

use std::collections::HashSet;

fn depth_first_score(grid: &Grid, x: usize, y: usize, current: u32) -> HashSet<(usize, usize)> {
    if current == 9 {
        let mut hs = HashSet::new();
        hs.insert((x, y));
        return hs;
    }
    let target = current + 1;
    let mut all_found  = HashSet::new();
    if x+1 < grid[y].len() && grid[y][x+1] == target {
        let found = depth_first_score(&grid, x+1, y, target);
        all_found.extend(found);
    }
    if x != 0 && grid[y][x-1] == target {
        let found = depth_first_score(&grid, x-1, y, target);
        all_found.extend(found);
    }
    if y+1 < grid.len() && grid[y+1][x] == target {
        let found = depth_first_score(&grid, x, y+1, target);
        all_found.extend(found);
    }
    if y != 0 && grid[y-1][x] == target {
        let found = depth_first_score(&grid, x, y-1, target);
        all_found.extend(found);
    }
    all_found
}

pub fn run_a(input: &str) -> usize {
    let grid: Grid = parse(input);
    let mut sum = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == 0 {
                sum += depth_first_score(&grid, x, y, 0).len();
            }
        }
    }
    sum
}

fn depth_first_rating(grid: &Grid, x: usize, y: usize, current: u32) -> u32 {
    if current == 9 {
        return 1;
    }
    let target = current + 1;
    let mut sum  = 0;
    if x+1 < grid[y].len() && grid[y][x+1] == target {
        sum += depth_first_rating(&grid, x+1, y, target);
    }
    if x != 0 && grid[y][x-1] == target {
        sum += depth_first_rating(&grid, x-1, y, target);
    }
    if y+1 < grid.len() && grid[y+1][x] == target {
        sum += depth_first_rating(&grid, x, y+1, target);
    }
    if y != 0 && grid[y-1][x] == target {
        sum += depth_first_rating(&grid, x, y-1, target);
    }
    sum
}


pub fn run_b(input: &str) -> u32 {
    let grid: Grid = parse(input);
    let mut sum = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == 0 {
                sum += depth_first_rating(&grid, x, y, 0);
            }
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn small_a() {
        let result = run_a(&fs::read_to_string("./smalltest.txt").expect("No test file!"));
        assert_eq!(result, 1);
    }

    #[test]
    fn a() {
        let result = run_a(&fs::read_to_string("./test.txt").expect("No test file!"));
        assert_eq!(result, 36);
    }

    #[test]
    fn b() {
        let result = run_b(&fs::read_to_string("./test.txt").expect("No test file!"));
        assert_eq!(result, 81);
    }
}