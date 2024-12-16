use std::collections::{HashMap, HashSet, VecDeque};

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

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct Path {
    points: Box<HashSet<Point<usize>>>,
    score: usize,
    cursor: Reindeer
}

#[allow(dead_code)]
fn find_paths(maze: &Grid<char>, reindeer: Reindeer, target_score: usize) -> VecDeque<Path> {
    let mut paths = VecDeque::new();
    let mut terminal_paths = VecDeque::new();

    paths.push_back(Path { points: Box::new(HashSet::new()), score: 0, cursor: reindeer });
    loop {
        let path = paths.pop_front();
        if path.is_none() {
            break;
        }
        let path = path.unwrap();

        if path.score > target_score {
            // Move on without it, destroying this path.
            continue;
        }

        if path.points.contains(&path.cursor.pos) {
            // No self-intersecting paths!
            continue;
        }

        if maze[path.cursor.pos] == 'E' {
            let mut new_path = path.clone();
            new_path.points.insert(path.cursor.pos);
            terminal_paths.push_back(new_path);
            continue;
        }

        let front_pos = path.cursor.facing.offset_point(path.cursor.pos).unwrap_or(Point{x:0, y:0});
        if maze[front_pos] != '#' {
            let mut new_path = path.clone();
            new_path.cursor.pos = front_pos;
            new_path.score += 1;
            new_path.points.insert(path.cursor.pos);
            paths.push_back(new_path);
        }

        let left_pos = path.cursor.facing.left_from().offset_point(path.cursor.pos).unwrap_or(Point{x:0, y:0});
        if maze[left_pos] != '#' {
            let mut new_path = path.clone();
            new_path.cursor.pos = left_pos;
            new_path.cursor.facing = path.cursor.facing.left_from();
            new_path.score += 1001;
            new_path.points.insert(path.cursor.pos);
            paths.push_back(new_path);
        }

        let right_pos = path.cursor.facing.right_from().offset_point(path.cursor.pos).unwrap_or(Point{x:0, y:0});
        if maze[right_pos] != '#' {
            let mut new_path = path.clone();
            new_path.cursor.pos = right_pos;
            new_path.cursor.facing = path.cursor.facing.right_from();
            new_path.score += 1001;
            new_path.points.insert(path.cursor.pos);
            paths.push_back(new_path);
        }
    }
    terminal_paths
}

pub fn run_b(_input: &str) -> &'static str {
    // let grid: Grid<char> = Grid::parse(input);
    // let reindeer = Reindeer { 
    //     pos: grid.find(&'S').expect("No start tile!"),
    //     facing: Direction::Right
    // };
    // let minimal_score = search_maze(&grid, reindeer, 0, &mut Box::new(HashMap::new())).expect("No valid path through maze!");
    // let mut visited: HashSet<Point<usize>> = HashSet::new();
    // for path in find_paths(&grid, reindeer, minimal_score){
    //     visited.extend(path.points.iter());
    // }
    // visited.len()
    "(not finished)"
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

    // #[test]
    // fn b_tiny() {
    //     let result = run_b(&fs::read_to_string("./tiny.txt").expect("No test file!"));
    //     assert_eq!(result, 5);
    // }

    // #[test]
    // fn b() {
    //     let result = run_b(&fs::read_to_string("./test.txt").expect("No test file!"));
    //     assert_eq!(result, 45);
    // }

    // #[test]
    // fn b2() {
    //     let result = run_b(&fs::read_to_string("./test2.txt").expect("No test file!"));
    //     assert_eq!(result, 64);
    // }
}