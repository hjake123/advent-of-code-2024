use std::fs;
use std::env;

// Update this each time you start a day.
const MAX_DAY: i32 = 13;

fn main() {
    let mut daynum: Option<i32> = None;
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let result = args[1].parse();
        if result.is_err() {
            println!("Invalid day number {}", args[1]);
            return;
        }
        daynum = Some(result.unwrap_or(0));
    }
    run_day(daynum);
}

// This method controls actually running the days.
// Add a new case for each new day.
fn run_day(daynum: Option<i32>) {
    match daynum {
        Some(1) => {
            println!("Day 1a: {}", day01::run_a(&fs::read_to_string("./input/day01.txt").unwrap()));
            println!("Day 1b: {}", day01::run_b(&fs::read_to_string("./input/day01.txt").unwrap()));
        },
        Some(2) => {
            println!("Day 2a: {}", day02::run_a(&fs::read_to_string("./input/day02.txt").unwrap()));
            println!("Day 2b: {}", day02::run_b(&fs::read_to_string("./input/day02.txt").unwrap()));
        },
        Some(3) => {
            println!("Day 3a: {}", day03::run_a(&fs::read_to_string("./input/day03.txt").unwrap()));
            println!("Day 3b: {}", day03::run_b(&fs::read_to_string("./input/day03.txt").unwrap()));
        },
        Some(4) => {
            println!("Day 4a: {}", day04::run_a(&fs::read_to_string("./input/day04.txt").unwrap()));
            println!("Day 4b: {}", day04::run_b(&fs::read_to_string("./input/day04.txt").unwrap()));
        },
        Some(5) => {
            println!("Day 5a: {}", day05::run_a(&fs::read_to_string("./input/day05.txt").unwrap()));
            println!("Day 5b: {}", day05::run_b(&fs::read_to_string("./input/day05.txt").unwrap()));
        },
        Some(6) => {
            println!("Day 6a: {}", day06::run_a(&fs::read_to_string("./input/day06.txt").unwrap()));
            println!("Day 6b: {}", day06::run_b(&fs::read_to_string("./input/day06.txt").unwrap()));
        },
        Some(7) => {
            println!("Day 7a: {}", day07::run_a(&fs::read_to_string("./input/day07.txt").unwrap()));
            println!("Day 7b: {}", day07::run_b(&fs::read_to_string("./input/day07.txt").unwrap()));
        },
        Some(8) => {
            println!("Day 8a: {}", day08::run_a(&fs::read_to_string("./input/day08.txt").unwrap()));
            println!("Day 8b: {}", day08::run_b(&fs::read_to_string("./input/day08.txt").unwrap()));
        },
        Some(9) => {
            println!("Day 9a: {}", day09::run_a(&fs::read_to_string("./input/day09.txt").unwrap()));
            println!("Day 9b: {}", day09::run_b(&fs::read_to_string("./input/day09.txt").unwrap()));
        },
        Some(10) => {
            println!("Day 10a: {}", day10::run_a(&fs::read_to_string("./input/day10.txt").unwrap()));
            println!("Day 10b: {}", day10::run_b(&fs::read_to_string("./input/day10.txt").unwrap()));
        },
        Some(11) => {
            println!("Day 11a: {}", day11::run_a(&fs::read_to_string("./input/day11.txt").unwrap()));
            println!("Day 11b: {}", day11::run_b(&fs::read_to_string("./input/day11.txt").unwrap()));
        },
        Some(12) => {
            println!("Day 12a: {}", day12::run_a(&fs::read_to_string("./input/day12.txt").unwrap()));
            println!("Day 12b: {}", day12::run_b(&fs::read_to_string("./input/day12.txt").unwrap()));
        },
        Some(13) => {
            println!("Day 13a: {}", day13::run_a(&fs::read_to_string("./input/day13.txt").unwrap()));
            println!("Day 13b: {}", day13::run_b(&fs::read_to_string("./input/day13.txt").unwrap()));
        },
        Some(14) => {
            println!("Day 14a: {}", day14::run_a(&fs::read_to_string("./input/day14.txt").unwrap()));
            println!("Day 14b: {}", day14::run_b(&fs::read_to_string("./input/day14.txt").unwrap()));
        },
        Some(n) => {
            println!("Can't run day number {}", n)
        }
        None => {
            for number in 1..(MAX_DAY + 1) {
                run_day(Some(number))
            }
        }
    }
}
