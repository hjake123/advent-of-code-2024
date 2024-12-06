use std::fs;
use std::env;

// Update this each time you start a day.
const MAX_DAY: i32 = 6;

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
            /*println!("Day 6b: {}", day06::run_b(&fs::read_to_string("./input/day06.txt").unwrap()));*/
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
