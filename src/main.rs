use std::fs;

fn main() {
    println!("Day 0: {}", day00::run(&fs::read_to_string("./input/day00.txt").unwrap()));
    println!("Day 1a: {}", day01::run_a(&fs::read_to_string("./input/day01.txt").unwrap()));
    println!("Day 1b: {}", day01::run_b(&fs::read_to_string("./input/day01.txt").unwrap()));
}
