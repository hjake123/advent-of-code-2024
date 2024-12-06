pub fn run_a(input: &str) -> i32 {
    let mut sum = 0;
    sum
}

pub fn run_b(input: &str) -> i32 {
    let mut sum: i32 = 0;
    sum
    
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn a() {
        let result = run_a(&fs::read_to_string("./test.txt").unwrap());
        assert_eq!(result, 41);
    }

    #[test]
    fn b() {
        let result = run_b(&fs::read_to_string("./test.txt").unwrap());
        assert_eq!(result, 1);
    }
}
