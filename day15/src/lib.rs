use common::{Grid, Direction};

fn parse(input: &str) -> (Grid<char>, Vec<Direction>) {
    let mut lines = input.lines();
    let warehouse = Grid::parse_until(&mut lines, |l| {
        l.is_empty()
    });
    let mut commands: Vec<Direction> = Vec::new();
    for remaining_line in lines {
        for ch in remaining_line.chars() {
            let command = Direction::from_char(ch);
            if command.is_ok() {
                commands.push(command.unwrap());
            } else {
                panic!("Invalid command!")
            }
        }
    }
    println!("{}", warehouse);
    dbg!(&commands);
    (warehouse, commands)
}

pub fn run_a(input: &str) -> i32 {
    let (mut warehouse, commands) = parse(input);
    0
}

pub fn run_b(input: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn a_small() {
        let result = run_a(&fs::read_to_string("./small.txt").expect("No test file!"));
        assert_eq!(result, 2028);
    }

    #[test]
    fn a_large() {
        let result = run_a(&fs::read_to_string("./large.txt").expect("No test file!"));
        assert_eq!(result, 10092);
    }

    #[test]
    fn b() {
        let result = run_b(&fs::read_to_string("./large.txt").expect("No test file!"));
        assert_eq!(result, 1);
    }
}