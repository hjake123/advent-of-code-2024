pub fn run_a(input: &str) -> i32 {
    0
}

pub fn run_b(input: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn a_small() {
        let result = run_a(&fs::read_to_string("./small.txt").expect("No test file!"));
        assert_eq!(result, 2028);
    }

    #[test]
    fn a_large() {
        let result = run_a(&fs::read_to_string("./large.txt").expect("No test file!"));
        assert_eq!(result, 10092);
    }

    #[test]
    fn b() {
        let result = run_b(&fs::read_to_string("./large.txt").expect("No test file!"));
        assert_eq!(result, 1);
    }
}