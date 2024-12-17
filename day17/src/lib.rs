use machine::Machine;
mod machine;

pub fn run_a(input: &str) -> String {
    let mut machine = Machine::parse(input);
    while machine.run() {}
    machine.output.iter().map(|num| num.to_string()).collect::<Vec<String>>().join(",")
}


pub fn run_b(input: &str) -> i32 {
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
    fn b_sanity_check() {
        let result = run_a(&fs::read_to_string("./test_b_corrected.txt").expect("No test file!"));
        assert_eq!(result, "0,3,5,4,3,0");
    }

    #[test]
    fn b() {
        let result = run_b(&fs::read_to_string("./test_b.txt").expect("No test file!"));
        assert_eq!(result, 117440);
    }
}