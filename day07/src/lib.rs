use std::thread::{self, JoinHandle};

mod part_a;
mod part_b;

fn parse_line(line: &str) -> (i64, Vec<i64>) {
    let mut parts = line.split(":");
    let p = parts.next().unwrap();
    let target: i64 = p.parse().expect(&("Can't parse ".to_owned() + p));
    let nums: Vec<i64> = parts.next().unwrap().split_whitespace().map(|token| token.parse().unwrap()).collect();
    (target, nums)
}

pub fn run_a(input: &str) -> i64 {
    let mut sum = 0;
    for line in input.lines() {
        let (target, nums) = parse_line(line);
        if part_a::test_variants(target, &nums) {
            sum += target;
        }
    }
    sum
}

pub fn run_b(input: &str) -> i64 {
    let mut sum = 0;
    let mut handles: Vec<JoinHandle<i64>> = Vec::new();
    for line in input.lines() {
        let line = line.to_owned();
        handles.push(thread::spawn(move || {
            let (target, nums) = parse_line(&line);
            if part_b::test_variants(target, &nums) {
                return target
            }
            0
        }))
    }
    for handle in handles {
        let res = handle.join();
        if res.is_ok() {
            sum += res.unwrap();
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn a() {
        let result = run_a(&fs::read_to_string("./test.txt").expect("No test file!"));
        assert_eq!(result, 3749);
    }

    #[test]
    fn b() {
        let result = run_b(&fs::read_to_string("./test.txt").expect("No test file!"));
        assert_eq!(result, 11387);
    }

    #[test]
    fn concat() {
        let result = run_b("100: 2 0 5");
        assert_eq!(result, 100);
    }
}