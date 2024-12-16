use common::{Grid, Direction, Point};

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
    (warehouse, commands)
}

fn robot_move(robot: Point<usize>, command: Direction, warehouse: &mut Grid<char>) -> Point<usize> {
    let robot_target = command.offset_point(robot);
    if robot_target.is_none() {
        return robot;
    }
    let robot_target = robot_target.unwrap();
    if warehouse[robot_target] == '#' {
        return robot;
    }
    if warehouse[robot_target] == '.' {
        warehouse[robot] = '.';
        warehouse[robot_target] = '@';
        return robot_target;
    }
    if warehouse[robot_target] == 'O' {
        // Push the boxes!
        let mut push_target = command.offset_point(robot_target);
        while push_target.is_some() && push_target.unwrap().in_bounds(warehouse.width(), warehouse.height()) && warehouse[push_target.unwrap()] == 'O' {
            push_target = command.offset_point(push_target.unwrap());
        }
        if push_target.is_none_or(|pt| {warehouse[pt] == '#'}) {
            return robot;
        }
        if warehouse[push_target.unwrap()] == '.' {
            // Should be the only option but best to check. 
            warehouse[robot] = '.';
            warehouse[robot_target] = '@';
            warehouse[push_target.unwrap()] = 'O';
            return robot_target;
        }
        
    }
    panic!("Invalid character at robot target {}", warehouse[robot_target]);
}

pub fn run_a(input: &str) -> usize {
    let (mut warehouse, commands) = parse(input);
    let mut robot = warehouse.find(&'@').expect("No robot!");
    for command in commands {
        robot = robot_move(robot, command, &mut warehouse);
    }
    let mut sum = 0;
    for y in 0..warehouse.height() {
        for x in 0..warehouse.width() {
            if warehouse[(x, y)] == 'O' {
                sum += y * 100 + x;
            }
        }
    }
    sum
}

fn widen(warehouse: &Grid<char>) -> Grid<char> {
    let mut widened: Vec<Vec<char>> = Vec::new();
    for row in warehouse.vec() {
        let mut new_row = Vec::new();
        for item in row {
            match item {
                '#' => {
                    new_row.push('#');
                    new_row.push('#');
                },
                'O' => {
                    new_row.push('[');
                    new_row.push(']');
                },
                '.' => {
                    new_row.push('.');
                    new_row.push('.');
                },
                '@' => {
                    new_row.push('@');
                    new_row.push('.');
                },
                _ => {panic!("Invalid item in grid!")}
            }
        }
        widened.push(new_row);
    }
    Grid::new(widened)
}

fn scan_box_parts_above(here: Point<usize>, warehouse: &Grid<char>) -> Vec<(Point<usize>, char)> {
    let mut parts = Vec::new();
    if warehouse[here] == '[' {
        parts.push((here, '['));
        let there = here.offset(1, 0);
        parts.push((there, ']'));
        let above = Point{x: there.x, y: there.y - 1};
        parts.append(&mut scan_box_parts_above(above, warehouse));
    } else if warehouse[here] == ']' {
        parts.push((here, ']'));
        let there = Point{x: here.x - 1, y: here.y};
        parts.push((there, '['));
        let above = Point{x: there.x, y: there.y - 1};
        parts.append(&mut scan_box_parts_above(above, warehouse));
    } else {
        return parts;
    }
    let above = Point{x: here.x, y: here.y - 1};
    if warehouse[above] == '[' || warehouse[above] == ']' {
        parts.append(&mut scan_box_parts_above(above, warehouse));
    } 
    parts
}

fn scan_box_parts_below(here: Point<usize>, warehouse: &Grid<char>) -> Vec<(Point<usize>, char)> {
    let mut parts = Vec::new();
    if warehouse[here] == '[' {
        parts.push((here, '['));
        let there = here.offset(1, 0);
        parts.push((there, ']'));
        let below: Point<usize> = there.offset(0, 1);
        parts.append(&mut scan_box_parts_below(below, warehouse));
    } else if warehouse[here] == ']' {
        parts.push((here, ']'));
        let there = Point{x: here.x - 1, y: here.y};
        parts.push((there, '['));
        let below = there.offset(0, 1);
        parts.append(&mut scan_box_parts_below(below, warehouse));
    } else {
        return parts;
    }
    let below = here.offset(0, 1);
    if warehouse[below] == '[' || warehouse[below] == ']' {
        parts.append(&mut scan_box_parts_below(below, warehouse));
    } 
    parts
}

fn robot_move_wide(robot: Point<usize>, command: Direction, warehouse: &mut Grid<char>) -> Point<usize> {
    let robot_target = command.offset_point(robot);
    if robot_target.is_none() {
        return robot;
    }
    let robot_target = robot_target.unwrap();
    if warehouse[robot_target] == '#' {
        return robot;
    }
    if warehouse[robot_target] == '.' {
        warehouse[robot] = '.';
        warehouse[robot_target] = '@';
        return robot_target;
    }

    if warehouse[robot_target] == '[' || warehouse[robot_target] == ']' {
        // New push logic.
        let mut push_target: Option<Point<usize>> = Some(robot_target);
        while push_target.is_some_and(|target| target.in_bounds(warehouse.width(), warehouse.height()) && (warehouse[target] == '[' || warehouse[target] == ']')) {
            push_target = command.offset_point(push_target.unwrap());
        }
        if push_target.is_none_or(|target| warehouse[target] == '#') {
            return robot;
        }
        // At this point, we are definitely pushing the boxes.

        if command == Direction::Left || command == Direction::Right {
            let push_target = push_target.unwrap();
            let mut pressure_wave = robot_target;
            let mut held_item = warehouse[robot];
            warehouse[robot] = '.';
            while push_target != pressure_wave {
                let about_to_hold_item = warehouse[pressure_wave];
                warehouse[pressure_wave] = held_item;
                held_item = about_to_hold_item;
                pressure_wave = command.offset_point(pressure_wave).unwrap();
            }
            warehouse[push_target] = held_item;
        } else if command == Direction::Up {
            let mut affected_box_parts = scan_box_parts_above(robot_target, warehouse);
            for part in &affected_box_parts {
                let above = Point{x: part.0.x, y: part.0.y - 1};
                if warehouse[above] == '#' {
                    return robot;
                }
            }
            for part in &mut affected_box_parts {
                let above = Point{x: part.0.x, y: part.0.y - 1};
                warehouse[part.0] = '.';
                part.0 = above;
            }
            for part in &affected_box_parts {
                warehouse[part.0] = part.1;
                // println!("moving!");
                // println!("{}", warehouse);
            }
            warehouse[robot_target] = '@';
            warehouse[robot] = '.';
        } else {
            // Command is Direction.Down
            let mut affected_box_parts = scan_box_parts_below(robot_target, warehouse);
            for part in &affected_box_parts {
                let above = Point{x: part.0.x, y: part.0.y + 1};
                if warehouse[above] == '#' {
                    return robot;
                }
            }
            for part in &mut affected_box_parts {
                let above = Point{x: part.0.x, y: part.0.y + 1};
                warehouse[part.0] = '.';
                part.0 = above;
            }
            for part in &affected_box_parts {
                warehouse[part.0] = part.1;
                // println!("moving!");
                // println!("{}", warehouse);
            }
            warehouse[robot_target] = '@';
            warehouse[robot] = '.';
        }
        return robot_target;
    }
    
    panic!("Invalid character at robot target {}", warehouse[robot_target]);
}

pub fn run_b(input: &str) -> usize {
    let (warehouse, commands) = parse(input);
    let mut warehouse = widen(&warehouse);
    let mut robot = warehouse.find(&'@').expect("No robot!");

    for command in commands {
        // println!("{}: \n{}", command.to_char(), warehouse);
        robot = robot_move_wide(robot, command, &mut warehouse);
    }
    // println!("{}", warehouse);

    let mut sum = 0;
    for y in 0..warehouse.height() {
        for x in 0..warehouse.width() {
            if warehouse[(x, y)] == '[' {
                sum += y * 100 + x;
            }
        }
    }
    sum
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
    fn b_moving() {
        run_b(&fs::read_to_string("./moving.txt").expect("No test file!"));
    }

    #[test]
    fn b_moving_2() {
        run_b(&fs::read_to_string("./moving2.txt").expect("No test file!"));
    }

    #[test]
    fn b() {
        let result = run_b(&fs::read_to_string("./large.txt").expect("No test file!"));
        assert_eq!(result, 9021);
    }
}