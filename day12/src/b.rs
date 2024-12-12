pub fn load(input: &str) -> (Vec<Vec<char>>, Vec<Vec<usize>>) {
    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut region_markers: Vec<Vec<usize>> = Vec::new();

    for line in input.lines() {
        let mut row = Vec::new();
        let mut vrow = Vec::new();

        for letter in line.chars() {
            row.push(letter);
            vrow.push(0);
        }
        grid.push(row);
        region_markers.push(vrow);
    }
    (grid, region_markers)
}

pub fn mark_region(grid: &Vec<Vec<char>>, region_grid: &mut Vec<Vec<usize>>, region_id: usize, letter: char, x: usize, y: usize) -> usize {
    if y >= grid.len() || x >= grid[y].len() {
        return 0;
    }
    if grid[y][x] != letter || region_grid[y][x] != 0 {
        return 0;
    }
    region_grid[y][x] = region_id;
    let mut sum = 1;
    sum += mark_region(grid, region_grid, region_id, letter, x + 1, y);
    sum += mark_region(grid, region_grid, region_id, letter, x, y + 1);

    if x > 0 {
        sum += mark_region(grid, region_grid, region_id, letter, x - 1, y);
    }

    if y > 0 {
        sum += mark_region(grid, region_grid, region_id, letter, x , y - 1);
    }
    return sum;
}

/*
An edge of a grid cell.
0 : Top
1 : Right
2 : Bottom
3 : Left
*/
#[derive(Debug, PartialEq, Copy, Clone)]
struct Edge {
    n: i8
}

impl Edge {
    pub fn new() -> Self {
        Edge {
            n: 0
        }
    }

    pub fn counter_clockwise(&self) -> Self {
        Edge {
            n: (self.n - 1).rem_euclid(4)
        }
    }

    pub fn clockwise(&self) -> Self {
        Edge {
            n: (self.n + 1).rem_euclid(4)
        }
    }

    pub fn adjacent_cell(&self, x: usize, y: usize, max_x: usize, max_y: usize) -> Option<(usize, usize)> {
        match self.n {
            0 => {
                if y == 0 {
                    return None
                }
                Some((x, y-1))
            }                
            1 => {
                if x+1 >= max_x {
                    return None
                }
                Some((x+1, y))
            }
            2 => {
                if y+1 >= max_y {
                    return None
                }
                Some((x, y+1))
            }
            3 => {
                if x == 0 {
                    return None
                }
                Some((x-1, y))
            }
            _ => {
                panic!("Edge has invalid internal number {}", self.n)
            }
        }
    }

    pub fn next_along(&self, x: usize, y: usize, max_x: usize, max_y: usize) -> Option<(usize, usize)> {
        match self.n {
            0 => {
                if x+1 >= max_x {
                    return None
                }
                Some((x+1, y))
            }
            1 => {
                if y+1 >= max_y {
                    return None
                }
                Some((x, y+1))
            }
            2 => {
                if x == 0 {
                    return None
                }
                Some((x-1, y))
            }
            3 => {
                if y == 0 {
                    return None
                }
                Some((x, y-1))
            }
            _ => {
                panic!("Edge has invalid internal number {}", self.n)
            }
        }
    }
}

pub fn scan_fence(region_grid: &Vec<Vec<usize>>, region_id: usize, start_x: usize, start_y: usize) -> usize {
    let mut edge = Edge::new();
    let start_edge = Edge::new();
    let mut x = start_x;
    let mut y = start_y;
    let mut side_count = 0;
    let max_y =  region_grid.len();
    let max_x = region_grid[0].len();
    loop {
        let next_cell = edge.next_along(x, y, max_x, max_y);
        let adjacent_to_next_cell = edge.adjacent_cell(x, y, max_x, max_y);
        let front_next_cell = match adjacent_to_next_cell {
            Some((x, y)) => {
                edge.next_along(x, y, max_x, max_y)
            }
            None => None
        };

        if next_cell.is_some_and(|cell| region_grid[cell.1][cell.0] == region_id) {
            if front_next_cell.is_some_and(|cell| region_grid[cell.1][cell.0] == region_id) {
                // Edge is hitting a wall; turn counter-clockwise.
                x = front_next_cell.unwrap().0;
                y = front_next_cell.unwrap().1;
                edge = edge.counter_clockwise();
                side_count += 1;
            } else {
                // Edge is passing onto a new space. Carry on.
                x = next_cell.unwrap().0;
                y = next_cell.unwrap().1;
            }
        } else {
            // Edge is trying to enter the void. Migrate clockwise and add one side?
            edge = edge.clockwise();
            side_count += 1;
        }
        if x == start_x && y == start_y && edge == start_edge {
            break;
        }
    }
    side_count
}