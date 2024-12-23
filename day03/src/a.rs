#[derive(Debug, PartialEq, Eq)]
enum ParseState {
    START,
    A,
    B
}

#[derive(Debug)]
pub struct Parser {
    state: ParseState,
    start_score: usize,
    a_buff: String,
    b_buff: String
}

impl Parser {
    pub fn new() -> Parser {
        return Parser {
            state: ParseState::START,
            start_score: 0,
            a_buff: String::new(),
            b_buff: String::new(),
        }
    }

    fn reset(&mut self) {
        self.a_buff.clear();
        self.b_buff.clear();
        self.start_score = 0;
        self.state = ParseState::START;
    }

    pub fn run(&mut self, glyphs: &Vec<char>, index: &mut usize, sum: &mut i32) -> Option<i32> {
        let glyph = glyphs[*index];
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