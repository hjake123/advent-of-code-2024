#[derive(Debug, Clone)]
pub struct Machine {
    pub a: i32,
    pub b: i32, 
    pub c: i32,
    pub ip: usize,
    pub program: Vec<i8>,
    pub output: Vec<i8>
}

impl Machine {
    pub fn parse(input: &str) -> Self {
        let mut numbers = common::extract_numbers(input).into_iter();
        let a = numbers.next().expect("Invalid machine schema!");
        let b = numbers.next().expect("Invalid machine schema!");
        let c = numbers.next().expect("Invalid machine schema!");
        let mut program: Vec<i8> = Vec::new();
        let output: Vec<i8> = Vec::new();
        for op in numbers {
            program.push(op.try_into().expect("Invalid program!"));
        }
        Machine {
            a, b, c, program, output, ip: 0
        }
    }

    // Tries to run one instruction. Returns whether the computer should keep running.
    pub fn run(&mut self) -> bool {
        let op = self.program[self.ip];
        let mut jumped = false;
        match op {
            0 => { // adv - A DIVIVE
                self.a >>= self.get_combo();
            }
            1 => { // bxl - B XOR LITERAL
                self.b ^= self.get_literal()
            }
            2 => { // bst - B STORE
                self.b = self.get_combo() % 8
            }
            3 => { // jnz - JUMP NOT ZERO
                if self.a != 0 {
                    jumped = true;
                    self.ip = self.get_literal().try_into().unwrap();
                }
            }
            4 => { // bxc - B XOR C
                self.b ^= self.c;
            }
            5 => { // out - OUTPUT
                self.output.push((self.get_combo() % 8).try_into().unwrap());
            }   
            6 => { // bdv - B DIVIDE
                self.b = self.a >> self.get_combo();
            }
            7 => { // cdv - C DIVIDE
                self.c = self.a >> self.get_combo();
            }
            _ => {
                panic!("Invalid opcode!")
            }
        }
        if !jumped {
            self.ip += 2;
        }
        if self.ip >= self.program.len() {
            return false;
        }
        true
    }

    fn get_literal(&self) -> i32 {
        self.program[self.ip + 1].into()
    }

    fn get_combo(&self) -> i32 {
        let combo = self.program[self.ip + 1];
        match combo {
            0 | 1 | 2 | 3 => {
                return combo.into()
            },
            4 => {
                return self.a
            }
            5 => {
                return self.b
            }
            6 => {
                return self.c
            }
            _ => {
                panic!("Invalid combo operand!")
            }
        }
    }
}
