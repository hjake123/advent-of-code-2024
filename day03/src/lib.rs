mod a;
mod b;

#[derive(Debug, PartialEq, Eq)]
pub enum ParseState {
    START,
    A,
    B
}

pub fn run_a(input: &str) -> i32 {
    let mut sum = 0;
    let mut parser = a::Parser {
        state: ParseState::START,
        start_score: 0,
        a_buff: String::new(),
        b_buff: String::new(),
    };

    let mut index = 0;
    while index < input.len() {
        parser.run(&input, &mut index, &mut sum);
        index += 1;
    }
    sum
}

pub fn run_b(input: &str) -> i32 {
    let mut sum = 0;
    let mut parser = b::Parser {
        state: ParseState::START,
        start_score: 0,
        a_buff: String::new(),
        b_buff: String::new(),
    };

    let mut index = 0;
    while index < input.len() {
        parser.run(&input, &mut index, &mut sum);
        index += 1;
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
        assert_eq!(result, 161);
    }

    #[test]
    fn b() {
        let result = run_b(&fs::read_to_string("./testb.txt").unwrap());
        assert_eq!(result, 48);
    }
}
