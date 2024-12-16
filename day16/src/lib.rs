use std::collections::HashMap;

use common::*;

#[derive(Debug, Copy, Clone)]
struct Reindeer {
    pos: Point<usize>,
    facing: Direction
}

fn search_maze(maze: &Grid<char>, reindeer: Reindeer, steps_so_far: usize, mut minima: &mut HashMap<Point<usize>, usize>) -> Option<usize> {
    if maze[reindeer.pos] == 'E' {
        return Some(steps_so_far)
    }
    if minima.get(&reindeer.pos).is_some_and(|minimum| *minimum <= steps_so_far) {
        return None
    }
    minima.insert(reindeer.pos, steps_so_far);
    let mut min = None;

    let front_pos = reindeer.facing.offset_point(reindeer.pos).expect("Out of bounds!");
    if front_pos.in_bounds(maze.width(), maze.height()) && maze[front_pos] != '#' {
        let steps = search_maze(maze, Reindeer{pos:front_pos, facing: reindeer.facing}, steps_so_far + 1, &mut minima);
        if steps.is_some() {
            min = steps;
        }
    }

    let left_of_facing = reindeer.facing.left_from();
    let left_pos = left_of_facing.offset_point(reindeer.pos).expect("Out of bounds!");
    if left_pos.in_bounds(maze.width(), maze.height()) && maze[left_pos] != '#' {
        let steps = search_maze(maze, Reindeer{pos:left_pos, facing: left_of_facing}, steps_so_far + 1001, &mut minima);
        if steps.is_some_and(|stepval| min.is_none_or(|minval | minval > stepval)) {
            min = steps;
        }
    }

    let right_of_facing = reindeer.facing.right_from();
    let right_pos = right_of_facing.offset_point(reindeer.pos).expect("Out of bounds!");
    if right_pos.in_bounds(maze.width(), maze.height()) && maze[right_pos] != '#' {
        let steps = search_maze(maze, Reindeer{pos:right_pos, facing: right_of_facing}, steps_so_far + 1001, &mut minima);
        if steps.is_some_and(|stepval| min.is_none_or(|minval | minval > stepval)) {
            min = steps;
        }
    }

    min
}

pub fn run_a(input: &str) -> usize {
    let grid: Grid<char> = Grid::parse(input);
    let reindeer = Reindeer { 
        pos: grid.find(&'S').expect("No start tile!"),
        facing: Direction::Right
    };
    search_maze(&grid, reindeer, 0, &mut HashMap::new()).expect("No valid path through maze!")
}

fn search_maze_b(maze: &Grid<char>, reindeer: Reindeer, steps_so_far: usize, mut minima: &mut HashMap<Point<usize>, usize>, target_score: usize) -> Option<usize> {
    if steps_so_far > target_score {
        return None
    }
    if maze[reindeer.pos] == 'E' {
        return Some(steps_so_far)
    }
    if minima.get(&reindeer.pos).is_some_and(|minimum| *minimum <= steps_so_far) {
        return None
    }
    minima.insert(reindeer.pos, steps_so_far);
    let mut min = None;

    let front_pos = reindeer.facing.offset_point(reindeer.pos).expect("Out of bounds!");
    if front_pos.in_bounds(maze.width(), maze.height()) && maze[front_pos] != '#' {
        let steps = search_maze_b(maze, Reindeer{pos:front_pos, facing: reindeer.facing}, steps_so_far + 1, &mut minima, target_score);
        if steps.is_some() {
            min = steps;
        }
    }

    let left_of_facing = reindeer.facing.left_from();
    let left_pos = left_of_facing.offset_point(reindeer.pos).expect("Out of bounds!");
    if left_pos.in_bounds(maze.width(), maze.height()) && maze[left_pos] != '#' {
        let steps = search_maze_b(maze, Reindeer{pos:left_pos, facing: left_of_facing}, steps_so_far + 1001, &mut minima, target_score);
        if steps.is_some_and(|stepval| min.is_none_or(|minval | minval > stepval)) {
            min = steps;
        }
    }

    let right_of_facing = reindeer.facing.right_from();
    let right_pos = right_of_facing.offset_point(reindeer.pos).expect("Out of bounds!");
    if right_pos.in_bounds(maze.width(), maze.height()) && maze[right_pos] != '#' {
        let steps = search_maze_b(maze, Reindeer{pos:right_pos, facing: right_of_facing}, steps_so_far + 1001, &mut minima, target_score);
        if steps.is_some_and(|stepval| min.is_none_or(|minval | minval > stepval)) {
            min = steps;
        }
    }

    min
}


pub fn run_b(input: &str) -> usize {
    let grid: Grid<char> = Grid::parse(input);
    let reindeer = Reindeer { 
        pos: grid.find(&'S').expect("No start tile!"),
        facing: Direction::Right
    };
    let minimal_score = search_maze(&grid, reindeer, 0, &mut HashMap::new()).expect("No valid path through maze!");
    search_maze_b(&grid, reindeer, 0, &mut HashMap::new(), minimal_score).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn a_tiny() {
        let result = run_a(&fs::read_to_string("./tiny.txt").expect("No test file!"));
        assert_eq!(result, 1004);
    }


    #[test]
    fn a() {
        let result = run_a(&fs::read_to_string("./test.txt").expect("No test file!"));
        assert_eq!(result, 7036);
    }

    #[test]
    fn a2() {
        let result = run_a(&fs::read_to_string("./test2.txt").expect("No test file!"));
        assert_eq!(result, 11048);
    }

    #[test]
    fn b() {
        let result = run_b(&fs::read_to_string("./test.txt").expect("No test file!"));
        assert_eq!(result, 45);
    }

    #[test]
    fn b2() {
        let result = run_b(&fs::read_to_string("./test2.txt").expect("No test file!"));
        assert_eq!(result, 64);
    }
}