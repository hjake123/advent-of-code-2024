#[derive(Debug, PartialEq, Eq)]
enum ParseState {
    START,
    DO,
    DONT,
    MUL,
    A,
    B
}

#[derive(Debug)]
pub struct Parser {
    state: ParseState,
    start_score: usize,
    a_buff: String,
    b_buff: String,
    active: bool
}

impl Parser {
    pub fn new() -> Parser {
        return Parser {
            state: ParseState::START,
            start_score: 0,
            a_buff: String::new(),
            b_buff: String::new(),
            active: true
        }
    }

    fn reset(&mut self) {
        self.a_buff.clear();
        self.b_buff.clear();
        self.start_score = 0;
        self.state = ParseState::START;
    }

    pub fn run(&mut self, glyphs: &str, index: &mut usize, sum: &mut i32) -> Option<i32> {
        let glyph = glyphs.chars().collect::<Vec<char>>()[*index];
        match self.state {
            ParseState::START => {
                if glyph == 'd' {
                    self.state = ParseState::DO;
                    self.start_score = 1;
                } else if glyph == 'm' {
                    self.state = ParseState::MUL;
                    self.start_score = 1;
                }
            }
            ParseState::DO => {
                let start_pattern: &str = "do()";
                if self.start_score < start_pattern.len() && glyph != start_pattern.chars().collect::<Vec<char>>()[self.start_score] {
                    if self.start_score == 2 && glyph == 'n' {
                        self.state = ParseState::DONT;
                    } else {
                        self.reset();
                        return None;
                    }
                }
                self.start_score += 1;
                if self.start_score >= start_pattern.len() {
                    self.active = true;
                    self.reset();
                }
            }
            ParseState::DONT => {
                let start_pattern: &str = "don't()";
                if self.start_score < start_pattern.len() && glyph != start_pattern.chars().collect::<Vec<char>>()[self.start_score] {
                    self.reset();
                    return None;
                }
                self.start_score += 1;
                if self.start_score >= start_pattern.len() {
                    self.active = false;
                    self.reset();
                }
            }
            ParseState::MUL => {
                let start_pattern: &str = "mul(";
                if self.start_score < start_pattern.len() && glyph != start_pattern.chars().collect::<Vec<char>>()[self.start_score] {
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
        if !self.active {
            return None
        }

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