use common::Point;

#[derive(Debug)]
struct Robot {
    position: Point<i32>,
    velocity: Point<i32>
}

impl Robot {
    fn from_line(line: &str) -> Self {
        let chunks: Vec<i32> = line.replace(|ch: char| !ch.is_ascii_digit() && ch != '-', " ").split_whitespace().map(|chunk| chunk.parse::<i32>().unwrap()).collect();
        Robot { 
            position: Point { x: chunks[0], y: chunks[1] }, 
            velocity: Point { x: chunks[2], y: chunks[3] }
        }
    }
}

pub fn run_a(input: &str) -> i32 {
    let mut robots: Vec<Robot> = Vec::new();
    for line in input.lines() {
        robots.push(Robot::from_line(line));
    }
    dbg!(robots);
    0
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
        let result = run_a(&fs::read_to_string("./test.txt").expect("No test file!"));
        assert_eq!(result, 12);
    }

    #[test]
    fn b() {
        let result = run_b(&fs::read_to_string("./test.txt").expect("No test file!"));
        assert_eq!(result, 1);
    }
}