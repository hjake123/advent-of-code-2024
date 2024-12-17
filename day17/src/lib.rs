#[derive(Debug)]
struct Machine {
    a: i32,
    b: i32, 
    c: i32,
    ip: usize,
    program: Vec<i8>,
    output: Vec<i8>
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
                self.a /= 2_i32.pow(self.get_combo().try_into().unwrap());
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
                self.b = self.a / 2_i32.pow(self.get_combo().try_into().unwrap());
            }
            7 => { // cdv - C DIVIDE
                self.c = self.a / 2_i32.pow(self.get_combo().try_into().unwrap());
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

pub fn run_a(input: &str) -> String {
    let mut machine = Machine::parse(input);
    while machine.run() {}
    machine.output.iter().map(|num| num.to_string()).collect::<Vec<String>>().join(",")
}

pub fn run_b(input: &str) -> String {
    "".to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn a() {
        let result = run_a(&fs::read_to_string("./test.txt").expect("No test file!"));
        assert_eq!(result, "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn a1() {
        let result = run_a(&fs::read_to_string("./test1.txt").expect("No test file!"));
        assert_eq!(result, "0,1,2");
    }

    #[test]
    fn b() {
        let result = run_b(&fs::read_to_string("./test.txt").expect("No test file!"));
        assert_eq!(result, "!");
    }
}