use core::num;

pub fn run_a(input: &str) -> i32 {
    let mut sum = 0;
    for line in input.lines() {
        if is_safe_line(line) {
            sum += 1
        }   
    }
    sum
}

fn is_safe_line(line: &str) -> bool {
    let mut prev: Option<_> = None;
    let mut increasing: Option<bool> = None;
    
    for symbol in line.split_whitespace() {
        let number = symbol.parse().unwrap();
        if prev.is_some() {
            if prev.unwrap() == number {
                return false
            }
            if increasing.is_none() {
                increasing = Some(prev.unwrap() < number);
            }
            if not_safe_diff(prev.unwrap(), number, increasing.unwrap()) {
                return false
            }
        }
        prev = Some(number);
    }
    true
}

fn not_safe_diff(a: i32, b: i32, increasing: bool) -> bool {
    let diff = b - a;
    // println!("{:?} {:?} {:?} {:?} {:?}", a, b, diff, increasing, ((diff < 0) == increasing));
    (diff > 3 || diff < -3) || ((diff < 0) == increasing)
}

pub fn run_b(input: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn a() {
        let result = run_a(&fs::read_to_string("./test.txt").unwrap());
        assert_eq!(result, 2);
    }

    #[test]
    fn a_tricky_case() {
        let result = run_a("29 28 27 29 32");
        assert_eq!(result, 0);
    }

    #[test]
    fn b() {
        let result = run_b(&fs::read_to_string("./test.txt").unwrap());
        assert_eq!(result, 0);
    }
}
