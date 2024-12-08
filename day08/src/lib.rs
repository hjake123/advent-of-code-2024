use std::{cell::RefCell, collections::HashMap, collections::HashSet};

type PositionMap = HashMap<char, RefCell<Vec<(isize, isize)>>>;

fn parse_antenna_positions (input: &str) -> (PositionMap, isize, isize) {
    let mut posmap: PositionMap = HashMap::new();
    let mut y = 0_isize;
    let mut x = 0_isize;
    for line in input.lines() {
        x = 0_isize;
        for ch in line.chars() {
            if ch != '.' {
                if !posmap.contains_key(&ch) {
                    posmap.insert(ch, RefCell::new(Vec::new()));
                }
                posmap.get(&ch).expect("Didn't insert!").borrow_mut().push((x, y));
            }
            x += 1;
        }
        y += 1;
    }
    (posmap, x, y)
}

fn pairs_of<T>(vec: &Vec<T>) -> Vec<(&T, &T)> {
    let mut pairs = Vec::new();
    for a in 0..vec.len() {
        for b in (a + 1)..vec.len() {
            pairs.push((&vec[a], &vec[b]));
        }
    }
    pairs
}

pub fn run_a(input: &str) -> usize {
    let (antennas, width, height) = parse_antenna_positions(input);
    let mut antinodes: HashSet<(isize, isize)> = HashSet::new();
    for frequency in antennas.keys() {
        let antennas = antennas.get(frequency).unwrap().borrow();
        for pair in pairs_of(&antennas) {
            let dx: isize = pair.1.0 - pair.0.0;
            let dy: isize = pair.1.1 - pair.0.1;
            let ax: isize = pair.0.0 - dx;
            let ay: isize = pair.0.1 - dy;
            let bx: isize = pair.1.0 + dx;
            let by: isize = pair.1.1 + dy;
            if ax >= 0 && ay >= 0 && ax < width && ax < height {
                antinodes.insert((ax, ay));
            }
            if bx >= 0 && by >= 0 && bx < width && by < height {
                antinodes.insert((bx, by));
            }  
        }   
    }
    antinodes.len()
}

pub fn run_b(input: &str) -> usize {
    let (antennas, width, height) = parse_antenna_positions(input);
    let mut antinodes: HashSet<(isize, isize)> = HashSet::new();
    for frequency in antennas.keys() {
        let antennas = antennas.get(frequency).unwrap().borrow();
        for pair in pairs_of(&antennas) {
            let dx: isize = pair.1.0 - pair.0.0;
            let dy: isize = pair.1.1 - pair.0.1;
            let mut ax: isize = pair.0.0;
            let mut ay: isize = pair.0.1;
            while ax >= 0 && ay >= 0 && ax < width && ay < height {
                antinodes.insert((ax, ay));
                ax -= dx;
                ay -= dy;

            }

            let mut bx: isize = pair.1.0;
            let mut by: isize = pair.1.1;
            while bx >= 0 && by >= 0 && bx < width && by < height {
                antinodes.insert((bx, by));
                bx += dx;
                by += dy;

            }
        }   
    }
    antinodes.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn a() {
        let result = run_a(&fs::read_to_string("./test.txt").expect("No test file!"));
        assert_eq!(result, 14);
    }

    #[test]
    fn escape() {
        let result = run_a(&fs::read_to_string("./escapetest.txt").expect("No test file!"));
        assert_eq!(result, 1);
    }

    #[test]
    fn simpleb() {
        let result = run_b(&fs::read_to_string("./simpleb.txt").expect("No test file!"));
        assert_eq!(result, 9);
    }

    #[test]
    fn b() {
        let result = run_b(&fs::read_to_string("./test.txt").expect("No test file!"));
        assert_eq!(result, 34);
    }
}