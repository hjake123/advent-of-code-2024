use core::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct Point {
    x: usize, 
    y: usize
}

impl Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug, PartialEq)]
pub struct Grid<T> {
    vec: Vec<Vec<T>>
}

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
