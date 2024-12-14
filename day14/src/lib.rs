use common::Point;
pub mod trees;

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
    
    fn travel(&mut self, seconds: i32, width: i32, height: i32) {
        let x_dist = self.velocity.x * seconds;
        self.position.x = (self.position.x + x_dist) % width;
        if self.position.x < 0 {
            self.position.x += width;
        }

        let y_dist = self.velocity.y * seconds;
        self.position.y = (self.position.y + y_dist) % height;

        if self.position.y < 0 {
            self.position.y += height;
        }
    }
}

fn tally_robots(robots: &Vec<Robot>, width: i32, height: i32) -> (u32, u32, u32, u32) {
    let left_max_x = width/2 - 1;
    let upper_max_y = height/2 - 1;
    let right_min_x = left_max_x + 2;
    let lower_min_y = upper_max_y + 2;

    let mut ul = 0;
    let mut ur = 0;
    let mut ll = 0;
    let mut lr = 0;
    for robot in robots {
        if robot.position.x <= left_max_x && robot.position.y <= upper_max_y {
            ul += 1;
        } else if robot.position.x >= right_min_x && robot.position.y <= upper_max_y {
            ur += 1;
        } else if robot.position.x <= left_max_x && robot.position.y >= lower_min_y {
            ll += 1;
        } else if robot.position.x >= right_min_x && robot.position.y >= lower_min_y {
            lr += 1;
        }
    }

    (ul, ur, ll, lr)
}

pub fn run_a(input: &str) -> u32 {
    let mut robots: Vec<Robot> = Vec::new();
    let mut lines = input.lines();

    let first_line = lines.next().expect("Empty file!");
    let wh: Vec<i32> = first_line.split_whitespace().map(|n| n.parse::<i32>().unwrap()).collect();
    let width = wh[0];
    let height = wh[1];

    for line in lines {
        robots.push(Robot::from_line(line));
    }

    for robot in &mut robots {
        robot.travel(100, width, height);
    }
    
    let (upper_left, upper_right, lower_left, lower_right) = tally_robots(&robots, width, height);

    upper_left * upper_right * lower_left * lower_right
}

pub fn run_b(input: &str) -> i32 {
    // Originally found that the answer is 6512 for my dataset using manual inspection.
    // Reddit has informed me that safety score is a good enough heuristic to find the answer also,
    // so this is implementing that solution.
    // The original tree building solution is maintained for honesty in trees.rs

    let mut robots: Vec<Robot> = Vec::new();
    let mut lines = input.lines();

    let first_line = lines.next().expect("Empty file!");
    let wh: Vec<i32> = first_line.split_whitespace().map(|n| n.parse::<i32>().unwrap()).collect();
    let width = wh[0];
    let height = wh[1];

    for line in lines {
        robots.push(Robot::from_line(line));
    }

    let mut low_score = i32::max_value();
    let mut low_score_n = 0;
    for n in 1..(width*height) {
        for robot in &mut robots {
            robot.travel(1, width, height);
        }
        let (upper_left, upper_right, lower_left, lower_right) = tally_robots(&robots, width, height);
        let score: i32 = (upper_left * upper_right * lower_left * lower_right).try_into().unwrap();
        if low_score > score {
            low_score = score;
            low_score_n = n;
        }
    }
    low_score_n
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
}