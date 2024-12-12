fn parse(input: &str) -> Vec<u64> {
    let mut vec: Vec<u64> = Vec::new();
    for word in input.split_whitespace() {
        vec.push(word.parse().unwrap())
    }
    vec
}

fn print_rocks(rocks: &Vec<u64>){
    for rock in rocks {
        print!("{} ", rock);
    }
    println!();
}

fn digit_count(n: u64) -> usize {
    let n = n.to_string();
    n.len()
}

fn split_by_digits(n: u64) -> (u64, u64){
    let mut n = n.to_string();
    let right = n.split_off(n.len() / 2);
    (n.parse().unwrap(), right.parse().unwrap())
}

fn blink(rocks: &mut Vec<u64>) {
    let mut i = 0;
    while i < rocks.len() {
        if rocks[i] == 0 {
            rocks[i] = 1;
        } else if digit_count(rocks[i]) % 2 == 0 {
            let (a, b) = split_by_digits(rocks[i]);
            rocks.remove(i);
            rocks.insert(i, b);
            rocks.insert(i, a);
            i += 1;
        } else {
            rocks[i] = rocks[i] * 2024;
        }
        i += 1;
    }
}

pub fn run_a(input: &str) -> usize {
    let mut rocks = parse(input);
    for _ in 0..25{
        blink(&mut rocks);
    }
    rocks.len()
}

pub fn run_b(input: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn one_blink() {
        let mut rocks = parse("0 1 10 99 999");
        blink(&mut rocks);
        assert_eq!(rocks, parse("1 2024 1 0 9 9 2021976"));
    }

    #[test]
    fn a() {
        let result = run_a(&fs::read_to_string("./test.txt").expect("No test file!"));
        assert_eq!(result, 55312);
    }

    #[test]
    fn b() {
        let result = run_b(&fs::read_to_string("./test.txt").expect("No test file!"));
        assert_eq!(result, 1);
    }
}