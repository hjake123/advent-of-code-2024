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
    (diff > 3 || diff < -3) || ((diff < 0) == increasing)
}

pub fn run_b(input: &str) -> i32 {
    let mut sum = 0;
    for line in input.lines() {
        if is_safe_line_dampened(line) {
            sum += 1;
        }   
    }
    sum
}

fn is_safe_line_dampened(line: &str) -> bool {
    let mut prev: Option<_> = None;
    let mut increasing: Option<bool> = None;
    let mut already_dampened = false;
    let mut turn_count = 0;
    
    for symbol in line.split_whitespace() {
        let number: i32 = symbol.parse().unwrap();

        if prev.is_some() {
            let prev: i32 = prev.unwrap();
            if prev == number {
                if already_dampened {
                    return false
                }
                already_dampened = true;
                continue;
            }
            if increasing.is_none() {
                increasing = Some(prev < number);
            }

            let diff = number - prev;
            let wrong_direction = (diff < 0) == increasing.unwrap();
            let not_safe = (diff > 3 || diff < -3) || wrong_direction;

            if wrong_direction && turn_count < 3 {
                increasing = Some(!increasing.unwrap());
                already_dampened = true;
                continue;
            }

            if not_safe {
                if already_dampened {
                    return false
                }
                already_dampened = true;
                continue;
            }
        }
        prev = Some(number);
        turn_count += 1;
    }
    true
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
        assert_eq!(result, 4);
    }

    #[test]
    fn b_tricky_case() {
        let result = run_b("29 28 30 31 32");
        assert_eq!(result, 1);
    }

    #[test]
    fn b_tricky_case_2() {
        let result = run_b("29 28 30 31 32");
        assert_eq!(result, 1);
    }
}
