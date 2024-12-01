use std::fs;

fn main() {
    println!("Day 0: {}", day00::run(&fs::read_to_string("./input/day00.txt").unwrap()));
    println!("Day 1: {}", day01::run(&fs::read_to_string("./input/day01.txt").unwrap()));
}
