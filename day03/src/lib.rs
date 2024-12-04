pub fn run_a(input: &str) -> i32 {
    let mut sum = 0;
    let mut parser = Parser {
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

#[derive(Debug)]
struct Parser {
    state: ParseState,
    start_score: usize,
    a_buff: String,
    b_buff: String
}

impl Parser {
    fn reset(&mut self) {
        self.a_buff.clear();
        self.b_buff.clear();
        self.start_score = 0;
        self.state = ParseState::START;
    }

    fn run(&mut self, glyphs: &str, index: &mut usize, sum: &mut i32) -> Option<i32> {
        let glyph = glyphs.chars().collect::<Vec<char>>()[*index];
        match self.state {
            ParseState::START => {
                let start_pattern: &str = "mul(";
                if self.start_score < start_pattern.len() && glyph != start_pattern.chars().collect::<Vec<char>>()[self.start_score]{
                    self.reset();
                    return None;
                }
                self.start_score += 1;
                if self.start_score >= start_pattern.len() {
                    self.state = ParseState::A;
                }
            }
            ParseState::A => {
                if glyph == ',' {
                    self.state = ParseState::B;
                } else if glyph.is_numeric() { 
                    self.a_buff.insert(self.a_buff.len(), glyph); 
                } else {
                    *index -= 1;
                    self.reset();
                }
            }
            ParseState::B => {
                if glyph == ')' {
                    let res = self.calc_mult();
                    if res.is_some() {
                        *sum += res.unwrap();
                    }
                    self.reset();
                } else if glyph.is_numeric() { 
                    self.b_buff.insert(self.b_buff.len(), glyph); 
                } else {
                    *index -= 1;
                    self.reset();
                }
            }
        }
        return None;
    }

    fn calc_mult(&self) -> Option<i32> {
        let a = self.a_buff.parse();
        if a.is_err() {
            return None
        }
        let a: i32 = a.unwrap();
    
        let b = self.b_buff.parse();
        if b.is_err() {
            return None
        }
        let b: i32 = b.unwrap();
        
        Some(a * b)
    }
}



#[derive(Debug, PartialEq, Eq)]
enum ParseState {
    START,
    A,
    B
}

pub fn run_b(input: &str) -> i32 {
    1
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
        let result = run_b(&fs::read_to_string("./test.txt").unwrap());
        assert_eq!(result, 1);
    }
}
