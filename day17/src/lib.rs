use std::collections::VecDeque;

use machine::Machine;
mod machine;

pub fn run_a(input: &str) -> String {
    let mut machine = Machine::parse(input);
    while machine.run() {}
    machine.output.iter().map(|num| num.to_string()).collect::<Vec<String>>().join(",")
}

/*
Either the machine is valid, or we panic.
Valid machnes return the combo number for the register that outputs the answer.
*/
fn validate_and_find_output(machine: &Machine) -> i8 {
    let mut i = 0;
    let mut o = 0;
    while i < machine.program.len() {
        if machine.program[i] == 3 {
            if i != machine.program.len() - 2 || machine.program[i + 1] != 0 {
                panic!("Invalid jump condition")
            }
        }
        if machine.program[i] == 5 {
            if machine.program[i + 1] < 4 {
                panic!("Literal output instruction")
            }
            o = machine.program[i + 1];
        }
        if machine.program[i] == 0 {
            if machine.program[i + 1] != 3 {
                panic!("Not always shifting A left by 3")
            }
        }

        i += 2;
    }
    o
}

/*
Finds the 3 bit value that produces a specific digit of output.
*/
fn trial_of_iteration(machine: &mut Machine, needed_output: i8) -> i32 {
    for n in 0..8{
        machine.a = n;
        println!("{}", n);
        while machine.run() {
            if machine.output.len() > 0 {
                break;
            }
            dbg!(&machine);
        }
        machine.ip = 0;
        machine.output.clear();
        if machine.output.len() > 0 && machine.output[0] == needed_output {
            return n
        }
    }
    panic!("No valid input produces {} for machine {:?}", needed_output, machine);
}

pub fn run_b(input: &str) -> i32 {
    let machine = Machine::parse(input);
    let output_register = validate_and_find_output(&machine);
    let mut accumulator = 0_i32;
    let mut outputs: VecDeque<&i8> = VecDeque::from(machine.program.iter().rev().collect::<Vec<_>>());
    let mut machine = machine.clone();
    while !outputs.is_empty() {
        if output_register == 4 {
            // We're outputting A.
            // Since only the adv instruction affects A,
            // and we know it always just shifts A by 3,
            // this is the easiest case.
            let n: i32 = (outputs.pop_front().unwrap() % 8).into();
            accumulator |= n;
            accumulator <<= 3;
        } else if output_register == 5 {
            let needed = *outputs.pop_front().unwrap();
            let n = trial_of_iteration(&mut machine, needed);
            accumulator |= n;
            accumulator <<= 3;
        } else {
            panic!("Outputting C register, which I haven't prepared for!")
        }
    }
    accumulator
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
    fn b_sanity_check() {
        let result = run_a(&fs::read_to_string("./test_b_corrected.txt").expect("No test file!"));
        assert_eq!(result, "0,3,5,4,3,0");
    }

    #[test]
    fn b() {
        let result = run_b(&fs::read_to_string("./test_b.txt").expect("No test file!"));
        assert_eq!(result, 117440);
    }

    #[test]
    fn b_2() {
        let result = run_b(&fs::read_to_string("./test_b_2.txt").expect("No test file!"));
        assert_eq!(result, 871256);
    }
}