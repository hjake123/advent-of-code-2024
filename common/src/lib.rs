use core::fmt;
use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct Point<T> {
    pub x: T, 
    pub y: T
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

#[derive(Debug, PartialEq)]
pub struct Grid<T> {
    vec: Vec<Vec<T>>
}

#[allow(dead_code)]
impl Grid<char> {
    fn new(input: &str) -> Self {
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
}

#[allow(dead_code)]
impl Grid<i32> {
    fn new(input: &str) -> Self {
        let mut grid: Vec<Vec<i32>> = Vec::new();        
        for line in input.lines() {
            let mut row = Vec::new();
            for word in line.split_whitespace() {
                row.push(word.parse().expect(&("Invalid integer ".to_owned() + word)));
            }
            grid.push(row);
        }
        Grid{vec: grid}
    }
}

#[cfg(test)]
mod tests {}
