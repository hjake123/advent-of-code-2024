use std::thread::{self, JoinHandle};

use std::collections::HashMap;

fn parse(input: &str) -> Vec<u64> {
    let mut vec: Vec<u64> = Vec::new();
    for word in input.split_whitespace() {
        vec.push(word.parse().unwrap())
    }
    vec
}

fn digit_count(n: u64) -> usize {
    let n = n.to_string();
    n.len()
}

fn split_by_digits(n: u64) -> (u64, u64){
    let n = n.to_string();
    let (left, right) = n.split_at(n.len() / 2);
    (left.parse().unwrap(), right.parse().unwrap())
}

type RockCache = HashMap<(u64, usize), usize>;

fn blink(rock: u64, depth: usize, max_depth: usize, cache: &mut RockCache) -> usize {
    if cache.contains_key(&(rock, depth)) {
        return *cache.get(&(rock, depth)).unwrap()
    }
    if depth == max_depth {
        cache.insert((rock, depth), 1);
        return 1
    }
    if rock == 0 {
        let result = blink(1, depth + 1, max_depth, cache);
        cache.insert((rock, depth), result);
        return result
    }
    if digit_count(rock) % 2 == 0 {
        let (left, right) = split_by_digits(rock);
        let l_result = blink(left, depth + 1, max_depth, cache);
        let result = l_result + blink(right, depth + 1, max_depth, cache);
        cache.insert((rock, depth), result);
        return result
    }
    let result = blink(rock * 2024, depth + 1, max_depth, cache);
    cache.insert((rock, depth), result);
    result
}

fn run(rocks: Vec<u64>, max_depth: usize) -> usize {
    let mut handles: Vec<JoinHandle<usize>> = Vec::new();
    for rock in rocks {
        handles.push(thread::spawn(move || {
            let mut cache: RockCache = HashMap::new();
            blink(rock, 0, max_depth, &mut cache)
        }));
    }
    let mut sum = 0;
    for handle in handles {
        sum += handle.join().unwrap()
    }
    sum
}

pub fn run_a(input: &str) -> usize {
    let rocks = parse(input);
    run(rocks, 25)
}

pub fn run_b(input: &str) -> usize {
    let rocks = parse(input);
    run(rocks, 75)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn a() {
        let result = run_a(&fs::read_to_string("./test.txt").expect("No test file!"));
        assert_eq!(result, 55312);
    }
}