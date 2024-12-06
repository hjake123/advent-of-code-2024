type Layout = Vec<Vec<char>>;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    pub fn right_from(&self) -> Self {
        match *self {
            Self::Up => {
                Self::Right
            }
            Self::Right => {
                Self::Down
            }
            Self::Down => {
                Self::Left
            }
            Self::Left => {
                Self::Up
            }
        }
    }

    pub fn offset(&self, x: usize, y: usize) -> (usize, usize) {
        match *self {
            Self::Up => {
                (x, y-1)
            }
            Self::Right => {
                (x+1, y)
            }
            Self::Down => {
                (x, y+1)
            }
            Self::Left => {
                (x-1, y)
            }
        }
    }
}

#[derive(Debug)]
struct Guard {
    x: usize,
    y: usize,
    dir: Direction,
    visit_count: i32
}

impl Guard {
    pub fn step(&mut self, layout: &mut Layout) -> bool {
        let (new_x, new_y) = self.dir.offset(self.x, self.y);
        if !(layout[self.y][self.x] == 'X') {
            layout[self.y][self.x] = 'X';
            self.visit_count += 1;
        }
        if new_y >= layout.len() || new_x >= layout[new_y].len() {
            return false
        }   
        if layout[new_y][new_x] == '#' {
            self.dir = self.dir.right_from();
        } else {
            self.x = new_x;
            self.y = new_y;
        }
        true
    }
}

fn parse_layout(input: &str) -> (Layout, Guard) {
    let mut layout = Vec::new();
    let mut x: usize = 0;
    let mut y: usize = 0;
    let mut found_guard = false;

    for line in input.lines() {
        let mut layout_line = Vec::new();
        for space in line.chars() {
            if space == '^' {
                found_guard = true;
            }
            if !found_guard {
                x += 1;
            }
            layout_line.push(space);
        }
        if !found_guard {
            x = 0;
            y += 1;
        }
        layout.push(layout_line);
    }
    (layout, Guard {x, y, dir: Direction::Up, visit_count: 0})
}

pub fn run_a(input: &str) -> i32 {
    let (mut layout, mut guard) = parse_layout(input);
    while guard.step(&mut layout) {}
    guard.visit_count
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
        let result = run_a(&fs::read_to_string("./test.txt").unwrap());
        assert_eq!(result, 41);
    }

    #[test]
    fn b() {
        let result = run_b(&fs::read_to_string("./test.txt").unwrap());
        assert_eq!(result, 1);
    }
}
