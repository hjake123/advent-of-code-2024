use std::{cmp::Ordering, collections::HashMap, str::Lines};

use common::Point;

#[derive(Debug)]
struct Machine {
    a_move: Point,
    b_move: Point, 
    prize: Point
}

fn read_one_machine(line_source: &mut Lines<'_>) -> Option<Machine> {
    let a_line = line_source.next();
    let b_line = line_source.next();
    let p_line = line_source.next();

    if a_line.is_none() || b_line.is_none() || p_line.is_none() {
        return None
    }

    let a_line = a_line.unwrap().replace(|c: char| !c.is_ascii_digit() && !c.is_whitespace(), "");
    let b_line = b_line.unwrap().replace(|c: char| !c.is_ascii_digit() && !c.is_whitespace(), "");
    let p_line = p_line.unwrap().replace(|c: char| !c.is_ascii_digit() && !c.is_whitespace(), "");

    let mut a_line = a_line.split_whitespace().map(|str| str.parse::<usize>().unwrap());
    let mut b_line = b_line.split_whitespace().map(|str| str.parse::<usize>().unwrap());
    let mut p_line = p_line.split_whitespace().map(|str| str.parse::<usize>().unwrap());

    Some(Machine {
        a_move: Point{x: a_line.next().unwrap(), y: a_line.next().unwrap()},
        b_move: Point{x: b_line.next().unwrap(), y: b_line.next().unwrap()},
        prize: Point{x: p_line.next().unwrap(), y: p_line.next().unwrap()}
    })
}

/*
Seek the solution using backtracking and memoization.
If you always miss the solution, returns None.
If you don't, returns the number of tokens used to reach a solution.
*/
fn hunt_solution(machine: &Machine, claw: Point, tokens_spent: usize, a_presses: usize, b_presses: usize, cache: &mut HashMap<(Point, usize, usize), Option<usize>>, has_max_depth: bool) -> Option<usize> {
    if has_max_depth && (a_presses > 100 || b_presses > 100) {
        return None
    }
    if cache.contains_key(&(claw, a_presses, b_presses)) {
        return *cache.get(&(claw, a_presses, b_presses)).unwrap();
    }
    if claw == machine.prize {
        cache.insert((claw, a_presses, b_presses), Some(tokens_spent));
        return Some(tokens_spent)
    }

    if claw.x > machine.prize.x || claw.y > machine.prize.y {
        cache.insert((claw, a_presses, b_presses), None);
        return None
    }

    let after_pressing_a = hunt_solution(machine, claw.offset(machine.a_move.x, machine.a_move.y), tokens_spent + 3, a_presses + 1, b_presses, cache, has_max_depth);
    let after_pressing_b = hunt_solution(machine, claw.offset(machine.b_move.x, machine.b_move.y), tokens_spent + 1, a_presses, b_presses + 1, cache, has_max_depth);

    if after_pressing_a.is_none() {
        if after_pressing_b.is_none() {
            cache.insert((claw, a_presses, b_presses), None);
            return None
        }
        cache.insert((claw, a_presses, b_presses), after_pressing_b);
        return after_pressing_b
    }
    if after_pressing_b.is_none() {
        cache.insert((claw, a_presses, b_presses), after_pressing_a);
        return after_pressing_a
    }

    match after_pressing_a.unwrap().cmp(&after_pressing_b.unwrap()) {
        Ordering::Greater => {
            cache.insert((claw, a_presses, b_presses), after_pressing_a);
            return after_pressing_a
        }
        Ordering::Less => {
            cache.insert((claw, a_presses, b_presses), after_pressing_b);
            return after_pressing_b
        }
        Ordering::Equal => {
            // Doesn't matter which
            cache.insert((claw, a_presses, b_presses), after_pressing_a);
            return after_pressing_a
        }
    }

}

pub fn run_a(input: &str) -> usize {
    let mut sum = 0;
    let mut lines = input.lines();
    loop {
        let machine = read_one_machine(&mut lines);
        let _ = lines.next();
        if machine.is_none(){
            break;
        }
        let machine = machine.unwrap();
        let solution = hunt_solution(&machine, Point{x: 0, y: 0}, 0, 0, 0, &mut HashMap::new(), true);
        if solution.is_some() {
            sum += solution.unwrap();
        }
    }
    sum
}

pub fn run_b(input: &str) -> usize {
    let mut sum = 0;
    let mut lines = input.lines();
    loop {
        let machine = read_one_machine(&mut lines);
        let _ = lines.next();
        if machine.is_none(){
            break;
        }
        let mut machine = machine.unwrap();
        machine.prize.x += 10000000000000;
        machine.prize.y += 10000000000000;
        let solution = hunt_solution(&machine, Point{x: 0, y: 0}, 0, 0, 0, &mut HashMap::new(), false);
        if solution.is_some() {
            sum += solution.unwrap();
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn a_single() {
        let result = run_a(&fs::read_to_string("./single.txt").expect("No test file!"));
        assert_eq!(result, 280);
    }

    #[test]
    fn a() {
        let result = run_a(&fs::read_to_string("./test.txt").expect("No test file!"));
        assert_eq!(result, 480);
    }

    #[test]
    fn b() {
        let result = run_b(&fs::read_to_string("./test.txt").expect("No test file!"));
        assert_eq!(result, 1);
    }
}