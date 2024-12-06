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

    pub fn offset(&self, x: usize, y: usize) -> Option<(usize, usize)> {
        match *self {
            Self::Up => {
                if y == 0 {
                    return None
                }
                Some((x, y-1))
            }
            Self::Right => {
                Some((x+1, y))
            }
            Self::Down => {
                Some((x, y+1))
            }
            Self::Left => {
                if x == 0 {
                    return None
                }
                Some((x-1, y))
            }
        }
    }

    pub fn to_char(&self) -> char {
        match *self {
            Self::Up => {
                '^'
            }
            Self::Right => {
                '>'
            }
            Self::Down => {
                'v'
            }
            Self::Left => {
                '<'
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
        let (new_x, new_y) = self.dir.offset(self.x, self.y).expect("Tried to move off screen!");
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

    pub fn part_b_step(&mut self, layout: &mut Layout) -> bool {
        let (new_x, new_y) = self.dir.offset(self.x, self.y).expect("Tried to move off screen!");
        if !(layout[self.y][self.x] != '.') {
            layout[self.y][self.x] = self.dir.to_char();
        }
        if !in_bounds(layout, new_x, new_y) {
            return false
        }   

        if layout[new_y][new_x] == '#' || layout[new_y][new_x] == '@' {
            self.dir = self.dir.right_from();
            layout[new_y][new_x] = '@';
        } else {
            // Check the new part B with this logic:
            // If there is an already used obstacle on your right
            // then this is a place where turning right would cause a loop.
            let right_of_facing = self.dir.right_from();
            let mut scanner = (self.x, self.y);
            while in_bounds(layout, scanner.0, scanner.1) {
                if layout[scanner.1][scanner.0] == '@' {
                    self.visit_count += 1;
                    break;
                }
                if layout[scanner.1][scanner.0] == '#' {
                    break;
                }
                let possible_scanner = right_of_facing.offset(scanner.0, scanner.1);
                if possible_scanner.is_none() {
                    break;
                }
                scanner = possible_scanner.unwrap();
            }

            // Normal stop logic resumes.
            self.x = new_x;
            self.y = new_y;
        }
        true
    }
}

fn in_bounds(layout: &Layout, x: usize, y: usize) -> bool {
    y < layout.len() && x < layout[y].len()
}

fn print_grid(vec2d: &Vec<Vec<char>>) {
    for line in vec2d {
        for ch in line {
            print!("{}", ch);
        }
        println!();
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
    let (mut layout, mut guard) = parse_layout(input);
    while guard.part_b_step(&mut layout) {}
    guard.visit_count
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
        assert_eq!(result, 6);
    }
}
