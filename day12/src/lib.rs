fn load(input: &str) -> (Vec<Vec<char>>, Vec<Vec<bool>>) {
    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut visited: Vec<Vec<bool>> = Vec::new();

    for line in input.lines() {
        let mut row = Vec::new();
        let mut vrow = Vec::new();

        for letter in line.chars() {
            row.push(letter);
            vrow.push(false);
        }
        grid.push(row);
        visited.push(vrow);
    }
    (grid, visited)
}

#[derive(Debug)]
struct Region {
    letter: char,
    area: usize,
    perimeter: usize
}

impl Region {
    fn new(letter: char) -> Self {
        Region{
            letter,
            area: 0,
            perimeter: 0
        }
    }

    fn tally(&mut self, grid: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>, x: usize, y: usize) {
        if y >= grid.len() || x >= grid[y].len() || grid[y][x] != self.letter {
            self.perimeter += 1;
            return;
        }
        if visited[y][x] {
            return;
        }
        visited[y][x] = true;
        self.area += 1;
        self.tally(grid, visited, x + 1, y);
        self.tally(grid, visited, x, y + 1);
        if x > 0 {
            self.tally(grid, visited, x - 1, y);
        } else {
            self.perimeter += 1;
        }
        if y > 0 {
            self.tally(grid, visited, x, y - 1);
        } else {
            self.perimeter += 1;
        }
    }
}

pub fn run_a(input: &str) -> usize {
    let (grid, mut visited) = load(input);
    let mut sum = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if !visited[y][x] {
                let mut region = Region::new(grid[y][x]);
                region.tally(&grid, &mut visited, x, y);
                sum += region.area * region.perimeter;
            }
        }
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