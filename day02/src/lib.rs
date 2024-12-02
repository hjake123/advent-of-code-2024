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
    is_safe_vec(line.split_whitespace().map(|symbol| symbol.parse().unwrap()).collect())
}

fn is_safe_vec(numbers: Vec<i32>) -> bool {
    let mut prev: Option<_> = None;
    let mut increasing: Option<bool> = None;
    
    // println!("Testing {:?}", numbers);
    for number in numbers {
        if prev.is_some() {
            let prev = prev.unwrap();
            if prev == number {
                return false
            }
            if increasing.is_none() {
                increasing = Some(prev < number);
            }
            if not_safe_diff(prev, number, increasing.unwrap()) {
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
    is_safe_vec_dampened(&line.split_whitespace().map(|symbol| symbol.parse().unwrap()).collect())
}

fn is_safe_vec_dampened(numbers: &Vec<i32>) -> bool {
    if is_safe_vec(numbers.to_vec()) {
        return true
    }
    let mut i = 0;
    while i < numbers.len() {
        if retry_without_index(numbers, i) {
            return true
        }
        i += 1;
    }
    false
}

fn retry_without_index(numbers: &Vec<i32>, index: usize) -> bool {
    // println!("Problem in {:?} ", numbers);
    // println!("Dampening error by removing index {}, which is {} ", index, numbers[index]);
    let mut pruned_numbers = numbers.to_vec();
    pruned_numbers.remove(index);
    // println!("Resulting vec is {:?}", pruned_numbers);
    return is_safe_vec(pruned_numbers)
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
        let result = run_b("29 28 27 31 32");
        assert_eq!(result, 0);
    }
}
