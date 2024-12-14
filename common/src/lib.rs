use core::fmt;
use std::{fmt::Display, ops::{Index, IndexMut}};

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
}

#[allow(dead_code)]
impl Grid<i32> {
    pub fn parse(input: &str) -> Self {
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

impl<T: std::fmt::Display> Display for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.vec {
            for item in row {
                let _ = write!(f, "{} ", item);
            }
            let _ = writeln!(f, "");
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {}
