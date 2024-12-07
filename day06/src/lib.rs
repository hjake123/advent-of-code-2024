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

    pub fn is_dir_char_other_then_own(&self, ch: char) -> bool {
        ch != self.to_char() && (ch == '^' || ch == '<' || ch == '>' || ch == 'v')
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

    pub fn step_check_loop(&mut self, layout: &mut Layout) -> bool {
        if let Some((new_x, new_y)) = self.dir.offset(self.x, self.y){
            if !(layout[self.y][self.x] != '.') {
                layout[self.y][self.x] = 'o';
            }
            if !in_bounds(layout, new_x, new_y) {
                return false
            }   
    
            if layout[new_y][new_x] == '#' || self.dir.is_dir_char_other_then_own(layout[new_y][new_x]) {
                if  layout[new_y][new_x] == '#' {
                    layout[new_y][new_x] = self.dir.to_char();
                }
                self.dir = self.dir.right_from();
            } else if layout[new_y][new_x] == self.dir.to_char() {
                // Loop detected.
                self.visit_count += 1;
                return false
            } else {
                self.x = new_x;
                self.y = new_y;
            }
            return true
        }
        false 
    }
}

fn in_bounds(layout: &Layout, x: usize, y: usize) -> bool {
    y < layout.len() && x < layout[y].len()
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
    let (layout, mut guard) = parse_layout(input);
    let startx = guard.x;
    let starty = guard.y;
    for y in 0..layout.len() {
        for x in 0..layout[y].len() {
            if layout[y][x] == '#' {
                continue;
            }
            let mut mutated_layout = layout.to_vec();
            mutated_layout[y][x] = '#';
            while guard.step_check_loop(&mut mutated_layout) {}
            guard.x = startx;
            guard.y = starty;
            guard.dir = Direction::Up;
        }
    }
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

    #[test]
    fn easy() {
        let result = run_b(&fs::read_to_string("./easytest.txt").unwrap());
        assert_eq!(result, 1);
    }

    #[test]
    fn evil() {
        let result = run_b(&fs::read_to_string("./eviltest.txt").unwrap());
        assert_eq!(result, 1);
    }
}
