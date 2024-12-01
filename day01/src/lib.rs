pub fn run_a(input: &str) -> i32 {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for line in input.lines() {
        let mut halves = line.split_whitespace();
        left.push(halves.next().unwrap().parse().unwrap());
        right.push(halves.next().unwrap().parse().unwrap());
    }

    left.sort_unstable();
    right.sort_unstable();

    let mut sum = 0;
    let mut index: usize = 0;
    while index < left.len() {
        sum += (left[index] - right[index]).abs();
        index += 1;
    }
    sum
}

use std::collections::HashMap;

pub fn run_b(input: &str) -> i32 {
    let mut lefts: Vec<i32> = Vec::new();
    let mut map: HashMap<i32, i32> = HashMap::new();

    for line in input.lines() {
        let mut halves = line.split_whitespace();
        let left = halves.next().unwrap().parse().unwrap();
        let right = halves.next().unwrap().parse().unwrap();
        lefts.push(left);

        if map.get(&right).is_none() {
            map.insert(right, 1);
        } else {
            map.insert(right, map.get(&right).unwrap() + 1);
        }
    }

    let mut sum: i32 = 0;
    for left in lefts {
        sum += map.get(&left).unwrap_or(&0) * left;
    }  

    sum
    
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn a() {
        let result = run_a(&fs::read_to_string("./test.txt").unwrap());
        assert_eq!(result, 11);
    }

    #[test]
    fn b() {
        let result = run_b(&fs::read_to_string("./test.txt").unwrap());
        assert_eq!(result, 31);
    }
}
