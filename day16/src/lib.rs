use std::{collections::{BinaryHeap, HashMap, HashSet}, fmt::Display, hash::Hash};

use common::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
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
#[derive(Debug, Clone, Hash)]
struct PathNode {
    cursor: Reindeer,
    score: usize
}

impl PartialEq for PathNode {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score && self.cursor.pos == other.cursor.pos
    }
}

impl Eq for PathNode {}

impl PartialOrd for PathNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PathNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.cmp(&other.score).reverse()
    }
}

#[allow(dead_code)]
fn find_paths(maze: &Grid<char>, reindeer: Reindeer, target_score: usize) -> HashSet<Point<usize>> {
    let mut queue: BinaryHeap<PathNode> = BinaryHeap::new();
    // let mut predecessors: HashMap<Reindeer, Option<&Reindeer>> = HashMap::new();
    let mut distances: HashMap<Reindeer, usize> = HashMap::new();

    queue.push(PathNode{cursor: reindeer, score: 0});

    for y in 0..maze.height() {
        for x in 0..maze.width() {
            for dir in vec![Direction::Up, Direction::Right, Direction::Left, Direction::Down] {
                distances.insert(Reindeer { pos: Point{x, y}, facing: dir }, usize::max_value());
            }
        }
    }

    distances.insert(reindeer, 0);

    while !queue.is_empty() {
        let node = queue.pop().unwrap();
        dbg!(&node);

        if node.score > target_score {
            break;
        }

        if maze[node.cursor.pos] == 'E' {
            
        }

        let front_pos = node.cursor.facing.offset_point(node.cursor.pos).unwrap_or(Point{x:0, y:0});
        if maze[front_pos] != '#' {
            let alt = node.score + 1;
            if alt < *distances.get(&Reindeer {pos: front_pos, facing:node.cursor.facing}).expect("?!") {
                distances.insert(node.cursor, alt);
                queue.push(PathNode{
                    cursor: Reindeer {pos:front_pos, facing:node.cursor.facing},
                    score: alt
                })
            }
        }

        let turned_dir = node.cursor.facing.left_from();
        let offset_pos = turned_dir.offset_point(node.cursor.pos).unwrap_or(Point{x:0, y:0});
        if maze[offset_pos] != '#' {
            let alt = node.score + 1001;
            let r = Reindeer {pos: offset_pos, facing:turned_dir};
            if alt < *distances.get(&r).expect("?!") {
                distances.insert(r, alt);
                queue.push(PathNode{
                    cursor: r,
                    score: alt
                })
            }
        }

        let turned_dir = node.cursor.facing.right_from();
        let offset_pos = turned_dir.offset_point(node.cursor.pos).unwrap_or(Point{x:0, y:0});
        if maze[offset_pos] != '#' {
            let alt = node.score + 1001;
            let r = Reindeer {pos: offset_pos, facing:turned_dir};
            if alt < *distances.get(&r).expect("?!") {
                distances.insert(r, alt);
                queue.push(PathNode{
                    cursor: r,
                    score: alt
                })
            }
        }
    }

    let mut terminal_path_points = HashSet::new();
    for pair in distances {
        if pair.1 <= target_score {
            terminal_path_points.insert(pair.0.pos);
        }
    }
    terminal_path_points
}

pub fn run_b(_input: &str) -> &'static str {
    // let grid: Grid<char> = Grid::parse(input);
    // let reindeer = Reindeer { 
    //     pos: grid.find(&'S').expect("No start tile!"),
    //     facing: Direction::Right
    // };
    // let minimal_score = search_maze(&grid, reindeer, 0, &mut Box::new(HashMap::new())).expect("No valid path through maze!");
    // let mut visited: HashSet<Point<usize>> = HashSet::new();
    // visited.extend(find_paths(&grid, reindeer, minimal_score));
    // visited.len()
    "(skipped)"
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