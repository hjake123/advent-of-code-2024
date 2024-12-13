use std::str::Lines;

use common::Point;

#[derive(Debug)]
struct Machine {
    a_move: Point<f64>,
    b_move: Point<f64>, 
    prize: Point<f64>
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

    let mut a_line = a_line.split_whitespace().map(|str| str.parse::<f64>().unwrap());
    let mut b_line = b_line.split_whitespace().map(|str| str.parse::<f64>().unwrap());
    let mut p_line = p_line.split_whitespace().map(|str| str.parse::<f64>().unwrap());

    Some(Machine {
        a_move: Point{x: a_line.next().unwrap(), y: a_line.next().unwrap()},
        b_move: Point{x: b_line.next().unwrap(), y: b_line.next().unwrap()},
        prize: Point{x: p_line.next().unwrap(), y: p_line.next().unwrap()}
    })
}

/*
Use some kind of math to just find the solution.
*/
fn calc_solution(m: &Machine) -> Option<u64> {
    let b = (m.a_move.x * m.prize.y - m.a_move.y * m.prize.x) / (m.b_move.y * m.a_move.x - m.a_move.y * m.b_move.x);
    let a = (m.prize.x - m.b_move.x * b) / m.a_move.x;
    if a.fract() == 0.0 && b.fract() == 0.0 {
        return Some(3*a as u64 + b as u64)
    }
    None
}

pub fn run_a(input: &str) -> u64 {
    let mut sum = 0;
    let mut lines = input.lines();
    loop {
        let machine = read_one_machine(&mut lines);
        let _ = lines.next();
        if machine.is_none(){
            break;
        }
        let machine = machine.unwrap();
        let solution = calc_solution(&machine);
        if solution.is_some() {
            sum += solution.unwrap();
        }
    }
    sum
}

pub fn run_b(input: &str) -> u64 {
    let mut sum = 0;
    let mut lines = input.lines();
    loop {
        let machine = read_one_machine(&mut lines);
        let _ = lines.next();
        if machine.is_none(){
            break;
        }
        let mut machine = machine.unwrap();
        machine.prize = Point{x: machine.prize.x + 10000000000000.0, y: machine.prize.y + 10000000000000.0};
        let solution = calc_solution(&machine);
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
        let _ = run_b(&fs::read_to_string("./single.txt").expect("No test file!"));
    }
}