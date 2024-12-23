use core::fmt;
use std::{error::Error, fmt::Display, ops::{Index, IndexMut}, str::Lines};

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct Point<T> {
    pub x: T, 
    pub y: T
}

impl Point<usize> {
    pub fn in_bounds(&self, width: usize, height: usize) -> bool {
        self.x < width && self.y < height
    }

    pub fn new(x: usize, y: usize) -> Self {
        Point{x, y}
    }

    pub fn offset_by(&self, x: isize, y: isize) -> Option<Self> {
        let new_x: Result<usize, _> = (self.x as isize - x).try_into();
        let new_y: Result<usize, _> = (self.y as isize - y).try_into();
        if new_x.is_err() || new_y.is_err() {
            return None
        }
        Some(Point{x: new_x.unwrap(), y: new_y.unwrap()})
    }
}

impl<T: std::fmt::Display> Display for Point<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl<T: std::ops::Add<Output = T> + Copy> Point<T> {
    pub fn offset(&self, x: T, y: T) -> Point<T> {
        Point{x: self.x + x, y: self.y + y}
    }   
}

#[derive(Debug, PartialEq, Clone)]
pub struct Grid<T> {
    vec: Vec<Vec<T>>
}

impl<T> Grid<T> {
    pub fn new(vec: Vec<Vec<T>>) -> Self {
        Grid { vec }
    }

    pub fn width(&self) -> usize {
        self.vec[0].len()
    }

    pub fn height(&self) -> usize {
        self.vec.len()
    }

    pub fn vec(&self) -> &Vec<Vec<T>> {
        &self.vec
    }
}

impl<T: PartialEq> Grid<T> {
    pub fn find(&self, item: &T) -> Option<Point<usize>> {
        for y in 0..self.vec.len() {
            for x in 0..self.vec[y].len() {
                if self[(x, y)] == *item {
                    return Some(Point{x, y})
                }
            }
        }
        None
    }
}

impl Grid<char> {
    pub fn of_size(width: usize, height: usize, filler: char) -> Self {
        let mut vec = Vec::new();
        for _ in 0..height {
            let mut row = Vec::new();
            for _ in 0..width {
                row.push(filler);
            }
            vec.push(row);
        }
        Grid{ vec }
    }

    pub fn parse(input: &str) -> Self {
        let mut grid: Vec<Vec<char>> = Vec::new();        
        for line in input.lines() {
            let mut row = Vec::new();
    
            for letter in line.chars() {
                row.push(letter);
            }
            grid.push(row);
        }
        Grid{vec: grid}
    }

    pub fn parse_until(lines: &mut Lines<'_>, end_condition: impl Fn(&str)->bool) -> Self {
        let mut grid: Vec<Vec<char>> = Vec::new();        
        for line in lines {
            if end_condition(line) {
                break;
            }
            let mut row = Vec::new();
    
            for letter in line.chars() {
                row.push(letter);
            }
            grid.push(row);
        }
        Grid{vec: grid}
    }
}

// impl Grid<i32> {
    // pub fn parse(input: &str) -> Self {
    //     let mut grid: Vec<Vec<i32>> = Vec::new();        
    //     for line in input.lines() {
    //         let mut row = Vec::new();
    //         for word in line.split_whitespace() {
    //             row.push(word.parse().expect(&("Invalid integer ".to_owned() + word)));
    //         }
    //         grid.push(row);
    //     }
    //     Grid{vec: grid}
    // }
// }

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.vec[index.1][index.0]
    }
}

impl<T> Index<(i32, i32)> for Grid<T> {
    type Output = T;

    fn index(&self, index: (i32, i32)) -> &Self::Output {
        let x: usize = index.0.try_into().unwrap();
        let y: usize = index.1.try_into().unwrap();
        &self[(x, y)]
    }
}

impl<T> Index<Point<usize>> for Grid<T> {
    type Output = T;

    fn index(&self, index: Point<usize>) -> &Self::Output {
        &self[(index.x, index.y)]
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.vec[index.1][index.0]
    }
}

impl<T> IndexMut<(i32, i32)> for Grid<T> {
    fn index_mut(&mut self, index: (i32, i32)) -> &mut Self::Output {
        let x: usize = index.0.try_into().unwrap();
        let y: usize = index.1.try_into().unwrap();
        &mut self[(x, y)]
    }
}

impl<T> IndexMut<Point<usize>> for Grid<T> {
    fn index_mut(&mut self, index: Point<usize>) -> &mut Self::Output {
        &mut self.vec[index.y][index.x]
    }
}

impl<T: std::fmt::Display> Display for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.vec {
            for item in row {
                let _ = write!(f, "{}", item);
            }
            let _ = writeln!(f, "");
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Direction {
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

    pub fn left_from(&self) -> Self {
        match *self {
            Self::Up => {
                Self::Left
            }
            Self::Right => {
                Self::Up
            }
            Self::Down => {
                Self::Right
            }
            Self::Left => {
                Self::Down
            }
        }
    }

    pub fn offset_point(&self, point: Point<usize>) -> Option<Point<usize>> {
        let coords = self.offset(point.x, point.y)?;
        Some(Point{x: coords.0, y: coords.1})
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

    pub fn from_char(ch: char) -> Result<Self, Box<dyn Error>> {
        match ch {
            '^' => Ok(Self::Up),
            '<' => Ok(Self::Left),
            '>' => Ok(Self::Right),
            'v' => Ok(Self::Down),
            _ => Err(Box::new(CommonError::DirectionParseError("Invalid direction character".to_owned())))
        }
    }

    pub fn is_dir_char_other_then_own(&self, ch: char) -> bool {
        ch != self.to_char() && (ch == '^' || ch == '<' || ch == '>' || ch == 'v')
    }
}

#[derive(Debug)]
enum CommonError {
    DirectionParseError(String)
}

impl Display for CommonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DirectionParseError(message) => {
                write!(f, "{}", message)
            }
        }
    }
}

impl Error for CommonError{
    
}

pub fn extract_numbers(line: &str) -> Vec<i32> {
    line.replace(|ch: char| !ch.is_ascii_digit() && ch != '-', " ").split_whitespace().map(|chunk| chunk.parse::<i32>().unwrap()).collect()
}

pub fn extract_numbers_i64(line: &str) -> Vec<i64> {
    line.replace(|ch: char| !ch.is_ascii_digit() && ch != '-', " ").split_whitespace().map(|chunk| chunk.parse::<i64>().unwrap()).collect()
}

#[cfg(test)]
mod tests {}
