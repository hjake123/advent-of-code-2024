use std::collections::HashMap;

use common::{Grid, Point, Direction};

fn find_next_open(grid: &Grid<char>, pos: Point<usize>) -> Point<usize> {
    let mut dir = Direction::Up;
    for _ in 0..4 {
        if dir.offset_point(pos).is_some_and(|offset| offset.in_bounds(grid.width(), grid.height()) && grid[offset] != '#') {
            return dir.offset_point(pos).unwrap()
        }
        dir = dir.left_from();
    }
    panic!("No opening from {}", pos)
}

fn generate_distance_markers(grid: &Grid<char>) -> HashMap<Point<usize>, u64> {
    let mut markers = HashMap::new();
    let mut grid = grid.clone();
    let mut cursor = grid.find(&'S').expect("No starting position!");
    let mut i = 0;
    while grid[cursor] != 'E' {
        grid[cursor] = '#';
        markers.insert(cursor, i);
        i += 1;
        cursor = find_next_open(&grid, cursor);
    }
    markers.insert(cursor, i);
    markers
}   

fn surrounding_points(point: Point<usize>) -> Vec<Option<Point<usize>>> {
    let mut vec = Vec::new();
    vec.push(point.offset_by(0, 1));
    vec.push(point.offset_by(0, 2));
    vec.push(point.offset_by(0, -1));
    vec.push(point.offset_by(0, -2));

    vec.push(point.offset_by(1, 0));
    vec.push(point.offset_by(2, 0));
    vec.push(point.offset_by(-1, 0));
    vec.push(point.offset_by(-2, 0));

    vec.push(point.offset_by(1, 1));
    vec.push(point.offset_by(-1, 1));
    vec.push(point.offset_by(1, -1));
    vec.push(point.offset_by(-1, -1));

    vec
}

pub fn run_a_inner(input: &str, threshold: u64) -> u64 {
    let grid = Grid::parse(input);
    let markers = generate_distance_markers(&grid);
    let mut count = 0;
    for (marker, current_dist) in &markers {
        for possible_point in surrounding_points(*marker) {
            if possible_point.is_some_and(|point| {
                point.in_bounds(grid.width(), grid.height()) 
                && markers.contains_key(&point) 
                && markers[&point] > *current_dist 
                && markers[&point] - *current_dist >= threshold
            }) {
                // println!("{} -> {} : {} - {} = {}", marker, possible_point.unwrap(), markers[&possible_point.unwrap()], *current_dist, markers[&possible_point.unwrap()] - *current_dist);
                count += 1;
            }
        }
    }
    count   
}

pub fn run_a(input: &str) -> u64 {
    run_a_inner(input, 100)
}

pub fn run_b(input: &str) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn a() {
        let result = run_a_inner(&fs::read_to_string("./test.txt").expect("No test file!"), 20);
        assert_eq!(result, 5);
    }

    #[test]
    fn b() {
        let result = run_b(&fs::read_to_string("./test.txt").expect("No test file!"));
        assert_eq!(result, 1);
    }
}

// 1341 is too high