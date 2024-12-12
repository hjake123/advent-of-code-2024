type Point = (usize, usize);

#[derive(Debug)]
struct Region {
    letter: char,
    points: Vec<Point>
}

impl Region {
    pub fn new(letter: char, x: usize, y: usize) -> Region{
        let mut points = Vec::new();
        points.push((x, y));
        Region{
            letter,
            points
        }   
    }

    pub fn add(&mut self, x: usize, y: usize) {
        self.points.push((x, y));
    }

    pub fn is_adjacent_to(&self, x: usize, y: usize) -> bool {
        for point in &self.points {
            if x > 0 && point.0 == x - 1 
            || y > 0 && point.1 == y - 1
            || point.0 == x + 1
            || point.1 == y + 1 {
                return true
            }
        }
        false
    }

    pub fn area(&self) -> usize{
        self.points.len()
    }

    pub fn perimeter(&self) -> usize{
        let mut sum = 0;
        for point in &self.points {
            if point.0 == 0 || !self.points.contains(&(point.0 - 1, point.1)) {
                sum += 1;
            }
            if point.1 == 0 || !self.points.contains(&(point.0, point.1 - 1)) {
                sum += 1;
            }
            if !self.points.contains(&(point.0 + 1, point.1)) {
                sum += 1;
            }
            if !self.points.contains(&(point.0, point.1 + 1)) {
                sum += 1;
            }
        }
        sum
    }
}

fn load(input: &str) -> Vec<Vec<char>> {
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        let mut row = Vec::new();
        for letter in line.chars() {
            row.push(letter);
        }
        grid.push(row);
    }
    grid
}

fn extract_regions(grid: Vec<Vec<char>>) -> Vec<Region> {
    let mut regions: Vec<Region> = Vec::new();
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let letter = grid[y][x];
            let mut got_added = false;
            for region in &mut regions {
                if letter == region.letter && region.is_adjacent_to(x, y) {
                    region.add(x, y);
                    got_added = true;
                    break;
                }
            }
            if regions.is_empty() || !got_added{
                regions.push(Region::new(letter, x, y));
            }
        }
    }
    regions
}

pub fn run_a(input: &str) -> usize {
    let regions = extract_regions(load(input));
    let mut sum = 0;
    for region in regions {
        println!("{}: {} {}", region.letter, region.area(), region.perimeter());
        sum += region.area() * region.perimeter();
    }
    sum
}

pub fn run_b(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn a1() {
        let result = run_a(&fs::read_to_string("./easy1.txt").expect("No test file!"));
        assert_eq!(result, 140);
    }

    #[test]
    fn a2() {
        let result = run_a(&fs::read_to_string("./easy2.txt").expect("No test file!"));
        assert_eq!(result, 772);
    }

    #[test]
    fn a3() {
        let result = run_a(&fs::read_to_string("./test.txt").expect("No test file!"));
        assert_eq!(result, 1930);
    }

    #[test]
    fn b() {
        let result = run_b(&fs::read_to_string("./test.txt").expect("No test file!"));
        assert_eq!(result, 1);
    }
}