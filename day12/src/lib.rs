type Point = (usize, usize);

#[derive(Debug)]
struct Region {
    letter: char,
    points: Vec<Point>
}

impl Region {
    pub fn new(letter: char) -> Region{
        Region{
            letter,
            points: Vec::new()
        }   
    }

    pub fn add(&mut self, x: usize, y: usize) {
        self.points.push((x, y));
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

fn is_touching_same_letter(letter: char, x: usize, y: usize, current_row: &Vec<char>, grid: &Vec<Vec<char>>) -> bool {
    if y > grid.len() || x >= current_row.len() {
        return false
    }
    (x > 0 && current_row[x - 1] == letter) || (y > 0 && grid[y - 1][x] == letter) || (x < (current_row.len() - 1) && current_row[x + 1] == letter)
}

fn add_to_region(letter: char, x: usize, y: usize, regions: &mut Vec<Region>) {
    for region in &mut (*regions) {
        if letter == region.letter && (x > 0 && region.points.contains(&(x-1, y)) || (y > 0 && region.points.contains(&(x, y-1)))){
            region.add(x, y);
            return
        }
    }
    let mut region = Region::new(letter);
    region.add(x, y);
    let regions = regions;
    regions.push(region)
}

fn extract_regions(input: &str) -> Vec<Region> {
    let mut regions: Vec<Region> = Vec::new();
    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut y: usize = 0;
    let mut x: usize = 0;
    for line in input.lines() {
        let mut row = Vec::new();
        for letter in line.chars() {
            row.push(letter);
        }

        for letter in &row {
            let letter = *letter;
            if is_touching_same_letter(letter, x, y, &row, &grid) {
                add_to_region(letter, x, y, &mut regions);
            } else {
                let mut r = Region::new(letter);
                r.add(x, y);
                regions.push(r)
            }
            x += 1;
        }

        grid.push(row);
        x = 0;
        y += 1;
    }
    regions
}

pub fn run_a(input: &str) -> usize {
    let regions = extract_regions(input);
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