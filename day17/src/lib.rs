use std::collections::VecDeque;

use machine::Machine;
mod machine;

pub fn run_a(input: &str) -> String {
    let mut machine = Machine::parse(input);
    while machine.run() {}
    machine.output.iter().map(|num| num.to_string()).collect::<Vec<String>>().join(",")
}

fn get_output_digit(a: u64) -> i8 {
    let b = a % 8;
    let b = b ^ 3;
    let c = a >> b;
    // let a = a >> 3;
    let b = b ^ 5;
    let b = b ^ c;
    (b % 8).try_into().unwrap()
}

fn consume_output(mut outputs: VecDeque<i8>, prior_a: u64) -> Option<u64> {
    if outputs.is_empty(){
        return Some(prior_a);
    }
    let output_digit = outputs.pop_front().unwrap();
    for a_octet in 0..8 {
        let partial_a = prior_a << 3 | a_octet;
        if get_output_digit(partial_a.try_into().unwrap()) == output_digit {
            let val = consume_output(outputs.clone(), partial_a);
            if val.is_some() {
                return val;
            }
        }
    }
    None
}

pub fn run_b(input: &str) -> u64 {
    // Not a general solution!
    let machine = Machine::parse(input);
    let outputs: VecDeque<i8> = VecDeque::from(machine.program.into_iter().rev().collect::<Vec<i8>>());
    let res = consume_output(outputs.clone(), 0);
    if res.is_some(){
        return res.unwrap();
    }
    0
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
    fn b_get_digit(){
        let mut a = 63687530;
        assert_eq!(get_output_digit(a), 1);
        a >>= 3;
        assert_eq!(get_output_digit(a), 6);
        a >>= 3;
        assert_eq!(get_output_digit(a), 7);
        a >>= 3;
        assert_eq!(get_output_digit(a), 4);
        a >>= 3;
        assert_eq!(get_output_digit(a), 3);
        a >>= 3;
        assert_eq!(get_output_digit(a), 0);
        a >>= 3;
        assert_eq!(get_output_digit(a), 5);
        a >>= 3;
        assert_eq!(get_output_digit(a), 0);
        a >>= 3;
        assert_eq!(get_output_digit(a), 6);
    }

    #[test]
    fn puzzle_input_sc() {
        let mut machine = Machine::parse(&fs::read_to_string("./pisc.txt").expect("No test file!"));
        machine.a = run_b(&fs::read_to_string("./pisc.txt").expect("No test file!")).try_into().unwrap();
        while machine.run() {}
        let result = machine.output.iter().map(|num| num.to_string()).collect::<Vec<String>>().join(",");
        assert_eq!(result, "2,4,1,3,7,5,0,3,1,5,4,1,5,5,3,0");
    }
}