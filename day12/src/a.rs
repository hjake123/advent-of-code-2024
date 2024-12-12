pub fn load(input: &str) -> (Vec<Vec<char>>, Vec<Vec<bool>>) {
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
pub struct Region {
    pub letter: char,
    pub area: usize,
    pub perimeter: usize
}

impl Region {
    pub fn new(letter: char) -> Self {
        Region{
            letter,
            area: 0,
            perimeter: 0
        }
    }

    pub fn tally(&mut self, grid: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>, x: usize, y: usize) {
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