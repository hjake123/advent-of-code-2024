use std::collections::HashMap;

use common::*;

fn search_maze(maze: &Grid<char>, cursor: Point<usize>, steps_so_far: usize, minima: &mut Box<HashMap<Point<usize>, usize>>) -> Option<usize> {
    if !cursor.in_bounds(maze.width(), maze.height()){
        return None
    }
    if maze[cursor] == '#' {
        return None
    }
    if maze[cursor] == 'E' {
        return Some(steps_so_far)
    }
    if minima.get(&cursor).is_some_and(|minimum| *minimum <= steps_so_far) {
        return None
    }
    minima.insert(cursor, steps_so_far);
    let mut min = None;
    
    let left = Direction::Left.offset_point(cursor).and_then(|offset| search_maze(maze, offset, steps_so_far+1, minima));
    if left.is_some_and(|nval | min.is_none_or(|minval| nval < minval)) {
        min = left;
    }

    let right = Direction::Right.offset_point(cursor).and_then(|offset| search_maze(maze, offset, steps_so_far+1, minima));
    if right.is_some_and(|nval | min.is_none_or(|minval| nval < minval)) {
        min = right;
    }

    let up = Direction::Up.offset_point(cursor).and_then(|offset| search_maze(maze, offset, steps_so_far+1, minima));
    if up.is_some_and(|nval | min.is_none_or(|minval| nval < minval)) {
        min = up;
    }

    let down = Direction::Down.offset_point(cursor).and_then(|offset| search_maze(maze, offset, steps_so_far+1, minima));
    if down.is_some_and(|nval | min.is_none_or(|minval| nval < minval)) {
        min = down;
    }

    min
}

pub fn run_a(input: &str) -> usize {
    run_a_inner(input, 71, 71, 1024)
}

pub fn run_a_inner(input: &str, width: usize, height: usize, iters: usize) -> usize {
    let mut grid = Grid::of_size(width, height, '.');
    let mut count = 0;
    for line in input.lines() {
        let numbers = common::extract_numbers(line);
        grid[(numbers[0], numbers[1])] = '#';
        count += 1;
        if count >= iters{
            break;
        }
    }
    grid[(width-1, height-1)] = 'E';
    let grid = grid;
    search_maze(&grid, Point{x:0, y:0}, 0, &mut Box::new(HashMap::new())).expect("No result!")
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
        let result = run_a_inner(&fs::read_to_string("./test.txt").expect("No test file!"), 7, 7, 12);
        assert_eq!(result, 22);
    }

    #[test]
    fn b() {
        let result = run_b(&fs::read_to_string("./test.txt").expect("No test file!"));
        assert_eq!(result, 1);
    }
}