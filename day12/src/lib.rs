mod a;

pub fn run_a(input: &str) -> usize {
    let (grid, mut visited) = a::load(input);
    let mut sum = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if !visited[y][x] {
                let mut region = a::Region::new(grid[y][x]);
                region.tally(&grid, &mut visited, x, y);
                sum += region.area * region.perimeter;
            }
        }
    }
    sum
}

mod b;
use std::collections::{HashMap, HashSet};

pub fn run_b(input: &str) -> usize {
    let (grid, mut region_grid) = b::load(input);
    let mut region_id = 1;
    let mut areas: HashMap<usize, usize> = HashMap::new();
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if region_grid[y][x] == 0 {
                areas.insert(region_id, b::mark_region(&grid, &mut region_grid, region_id, grid[y][x], x, y));
                region_id += 1;
            }
        }
    }
    let mut counted: HashSet<usize> = HashSet::new();
    let mut sum: usize = 0;
    for y in 0..region_grid.len() {
        for x in 0..region_grid[y].len() {
            region_id = region_grid[y][x];
            if !counted.contains(&region_id) {
                counted.insert(region_id);
                sum += areas.get(&region_id).unwrap() * b::scan_fence(&region_grid, region_id, x, y);
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
    fn a1() {
        let result = run_a(&fs::read_to_string("./easy1.txt").expect("No test file!"));
        assert_eq!(result, 140);
    }

    #[test]
    fn a2() {
        let result = run_a(&fs::read_to_string("./easy2.txt").expect("No test file!"));
        assert_eq!(result, 772);
    }

    #[test]
    fn a3() {
        let result = run_a(&fs::read_to_string("./test.txt").expect("No test file!"));
        assert_eq!(result, 1930);
    }

    #[test]
    fn b_tiny_1() {
        let result = run_b("00\n11");
        assert_eq!(result, 16);
    }

    #[test]
    fn b_tiny_2() {
        let result = run_b("11\n01");
        assert_eq!(result, 22);
    }

    #[test]
    fn b1_abcde() {
        let result = run_b(&fs::read_to_string("./easy1.txt").expect("No test file!"));
        assert_eq!(result, 80);
    }

    #[test]
    fn b2_island_o() {
        let result = run_b(&fs::read_to_string("./easy2.txt").expect("No test file!"));
        assert_eq!(result, 436);
    }

    #[test]
    fn b3_big_e() {
        let result = run_b(&fs::read_to_string("./b1.txt").expect("No test file!"));
        assert_eq!(result, 236);
    }

    #[test]
    fn b4_island_b() {
        let result = run_b(&fs::read_to_string("./b2.txt").expect("No test file!"));
        assert_eq!(result, 368);
    }

    #[test]
    fn b5() {
        let result = run_b(&fs::read_to_string("./test.txt").expect("No test file!"));
        assert_eq!(result, 1206);
    }
}