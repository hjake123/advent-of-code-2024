use std::{collections::{HashMap, HashSet}, iter};

use common::*;

#[derive(Debug, Copy, Clone)]
struct Reindeer {
    pos: Point<usize>,
    facing: Direction
}

fn search_maze(maze: &Grid<char>, reindeer: Reindeer, steps_so_far: usize, minima: &mut Box<HashMap<Point<usize>, usize>>) -> Option<usize> {
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
        let steps = search_maze(maze, Reindeer{pos:front_pos, facing: reindeer.facing}, steps_so_far + 1, minima);
        if steps.is_some() {
            min = steps;
        }
    }

    let left_of_facing = reindeer.facing.left_from();
    let left_pos = left_of_facing.offset_point(reindeer.pos).expect("Out of bounds!");
    if left_pos.in_bounds(maze.width(), maze.height()) && maze[left_pos] != '#' {
        let steps = search_maze(maze, Reindeer{pos:left_pos, facing: left_of_facing}, steps_so_far + 1001, minima);
        if steps.is_some_and(|stepval| min.is_none_or(|minval | minval > stepval)) {
            min = steps;
        }
    }

    let right_of_facing = reindeer.facing.right_from();
    let right_pos = right_of_facing.offset_point(reindeer.pos).expect("Out of bounds!");
    if right_pos.in_bounds(maze.width(), maze.height()) && maze[right_pos] != '#' {
        let steps = search_maze(maze, Reindeer{pos:right_pos, facing: right_of_facing}, steps_so_far + 1001, minima);
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
    let mut minima = Box::new(HashMap::new());
    search_maze(&grid, reindeer, 0, &mut minima).expect("No valid path through maze!")
}

fn search_maze_b(maze: &Grid<char>, reindeer: Reindeer, score_so_far: usize, target_score: usize, path: &HashSet<Point<usize>>) -> Option<HashSet<Point<usize>>> {
    if score_so_far > target_score {
        return None
    }

    let mut my_path = Box::new(path.clone());
    my_path.insert(reindeer.pos);
    let mut any_path_good = false;

    if maze[reindeer.pos] == 'E' {
        return Some(*my_path)
    }

    let front_pos = reindeer.facing.offset_point(reindeer.pos).expect("Out of bounds!");
    if front_pos.in_bounds(maze.width(), maze.height()) && maze[front_pos] != '#' {
        let steps = search_maze_b(maze, Reindeer{pos:front_pos, facing: reindeer.facing}, score_so_far + 1, target_score, &my_path);
        if steps.is_some() {
            my_path.extend(steps.unwrap());
            any_path_good = true;
        }
    }

    let left_of_facing = reindeer.facing.left_from();
    let left_pos = left_of_facing.offset_point(reindeer.pos).expect("Out of bounds!");
    if left_pos.in_bounds(maze.width(), maze.height()) && maze[left_pos] != '#' {
        let steps = search_maze_b(maze, Reindeer{pos:left_pos, facing: left_of_facing}, score_so_far + 1001, target_score, &my_path);
        if steps.is_some() {
            my_path.extend(steps.unwrap());
            any_path_good = true;
        }
    }

    let right_of_facing = reindeer.facing.right_from();
    let right_pos = right_of_facing.offset_point(reindeer.pos).expect("Out of bounds!");
    if right_pos.in_bounds(maze.width(), maze.height()) && maze[right_pos] != '#' {
        let steps = search_maze_b(maze, Reindeer{pos:right_pos, facing: right_of_facing}, score_so_far + 1001, target_score, &my_path);
        if steps.is_some() {
            my_path.extend(steps.unwrap());
            any_path_good = true;
        }
    }

    if any_path_good {
        return Some(*my_path)
    }
    None
}


pub fn run_b(input: &str) -> usize {
    let grid: Grid<char> = Grid::parse(input);
    let reindeer = Reindeer { 
        pos: grid.find(&'S').expect("No start tile!"),
        facing: Direction::Right
    };
    let minimal_score = search_maze(&grid, reindeer, 0, &mut Box::new(HashMap::new())).expect("No valid path through maze!");
    search_maze_b(&grid, reindeer, 0, minimal_score, &mut HashSet::new()).unwrap().len()
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
    fn b_tiny() {
        let result = run_b(&fs::read_to_string("./tiny.txt").expect("No test file!"));
        assert_eq!(result, 5);
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