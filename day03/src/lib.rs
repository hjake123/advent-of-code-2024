mod a;
mod b;

pub fn run_a(input: &str) -> i32 {
    let mut sum = 0;
    let mut parser = a::Parser::new();

    let mut index = 0;
    let glyphs = &input.chars().collect::<Vec<char>>();
    while index < input.len() {
        parser.run(glyphs, &mut index, &mut sum);
        index += 1;
    }
    sum
}

pub fn run_b(input: &str) -> i32 {
    let mut sum = 0;
    let mut parser = b::Parser::new();

    let mut index = 0;
    let glyphs = &input.chars().collect::<Vec<char>>();
    while index < input.len() {
        parser.run(glyphs, &mut index, &mut sum);
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
