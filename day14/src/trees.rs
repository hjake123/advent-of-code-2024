use common::Grid;

use crate::Robot;
use std::fs;
use std::io::Write;

fn robot_grid(robots: &Vec<Robot>, width: i32, height: i32) -> Grid<char> {
    let mut grid: Grid<char> = Grid::of_size(width.try_into().unwrap(), height.try_into().unwrap(), '.');
    for robot in robots {
        grid[(robot.position.x, robot.position.y)] = 'x';
    }
    grid
}

pub fn tree_finder(input: &str) -> Result<(), Box<dyn std::error::Error>>{
    let mut robots: Vec<Robot> = Vec::new();
    let mut lines = input.lines();

    let first_line = lines.next().expect("Empty file!");
    let wh: Vec<i32> = first_line.split_whitespace().map(|n| n.parse::<i32>().unwrap()).collect();
    let width = wh[0];
    let height = wh[1];

    for line in lines {
        robots.push(Robot::from_line(line));
    }

    let res: Result<fs::File, std::io::Error> = fs::File::create_new("trees.txt");
    let mut file = res?;

    let max = width*height;
    for n in 0..max {
        for robot in &mut robots {
            robot.travel(1, width, height);
        }
        writeln!(file, "{}", n)?;
        writeln!(file, "{}", robot_grid(&robots, width, height))?;
        writeln!(file, "")?;
        println!("Tree {} of {} complete", n, max);
    }

    Ok(())
}