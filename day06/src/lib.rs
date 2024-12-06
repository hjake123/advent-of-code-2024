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

    pub fn part_b_step(&mut self, layout: &mut Layout) -> bool {
        let (new_x, new_y) = self.dir.offset(self.x, self.y);
        if !(layout[self.y][self.x] != '.') {
            layout[self.y][self.x] = self.dir.to_char();
        }
        if new_y >= layout.len() || new_x >= layout[new_y].len() {
            return false
        }   

        if layout[new_y][new_x] == '#' {
            self.dir = self.dir.right_from();
        } else {
            // Check the new part B with this logic:
            // If there is a rightward path to your right,
            // and no path in front of you,
            // then the obstacle would put you onto your own past path, making you loop.
            let right_of_facing = self.dir.right_from().offset(self.x, self.y);
            if layout[right_of_facing.1][right_of_facing.0] == self.dir.right_from().to_char(){
                self.visit_count += 1;
                dbg!(&self);
                let mut tmp = layout.to_vec();
                tmp[new_y][new_x] = 'O';
                print_grid(&tmp);
            }
            self.x = new_x;
            self.y = new_y;
        }
        true
    }
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
