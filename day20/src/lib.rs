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

fn surrounding_points(point: Point<usize>, radius: isize) -> Vec<Option<Point<usize>>> {
    let mut vec = Vec::new();
    for yoff in 0..=radius {
        for xoff in 0..=radius {
            if xoff == 0 && yoff == 0 {
                continue;
            }
            if xoff + yoff > radius {
                continue;
            }
            vec.push(point.offset_by(xoff, yoff));
            if xoff != 0 {
                vec.push(point.offset_by(-xoff, yoff));
            }
            if yoff != 0 {
                vec.push(point.offset_by(xoff, -yoff));
                if xoff != 0 {
                    vec.push(point.offset_by(-xoff, -yoff));
                }
            }
        }
    }
    
    vec
}

pub fn run_inner(input: &str, threshold: u64, radius: isize) -> u64 {
    let grid = Grid::parse(input);
    let markers = generate_distance_markers(&grid);
    let mut count = 0;
    for (marker, current_dist) in &markers {
        for possible_point in surrounding_points(*marker, radius) {
            if possible_point.is_some_and(|point| {
                let man_dist: u64 = (marker.x.abs_diff(point.x) + marker.y.abs_diff(point.y)).try_into().unwrap();
                point.in_bounds(grid.width(), grid.height()) 
                && markers.contains_key(&point) 
                && markers[&point] > *current_dist 
                && markers[&point] - *current_dist - man_dist >= threshold
            }) {
                count += 1;
            }
        }
    }
    count   
}

pub fn run_a(input: &str) -> u64 {
    run_inner(input, 100, 2)
}

pub fn run_b(input: &str) -> u64 {
    run_inner(input, 100, 20)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn a() {
        let result = run_inner(&fs::read_to_string("./test.txt").expect("No test file!"), 20, 2);
        assert_eq!(result, 5);
    }

    #[test]
    fn a2() {
        let result = run_inner(&fs::read_to_string("./test.txt").expect("No test file!"), 10, 2);
        assert_eq!(result, 10);
    }

    #[test]
    fn b() {
        let result = run_inner(&fs::read_to_string("./test.txt").expect("No test file!"), 70, 20);
        assert_eq!(result, 41);
    }
}