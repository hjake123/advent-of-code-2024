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
    // dbg!(&grid);

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
    // dbg!(&region_grid);
    let mut counted: HashMap<usize, HashSet<(usize, usize)>> = HashMap::new();
    let mut side_counts: HashMap<usize, usize> = HashMap::new();
    for y in 0..region_grid.len() {
        for x in 0..region_grid[y].len() {
            region_id = region_grid[y][x];
            if !counted.contains_key(&region_id) {
                counted.insert(region_id, HashSet::new());
            }
            // println!("({}, {}) : {}", x, y, region_id);
            // dbg!(&counted);
            // dbg!(&counted.get(&region_id).unwrap().contains(&(x, y)));
            // dbg!(y > 0 && region_grid[y - 1][x] == region_id);
            // dbg!(!(counted.get(&region_id).unwrap().contains(&(x, y))) && !(y > 0 && region_grid[y - 1][x] == region_id));
            if !(counted.get(&region_id).unwrap().contains(&(x, y))) && !(y > 0 && region_grid[y - 1][x] == region_id){
                side_counts.insert(region_id, side_counts.get(&region_id).unwrap_or(&0) +  
                    b::scan_fence(&region_grid, region_id, x, y, counted.get_mut(&region_id).unwrap()));
                // dbg!(&side_counts);
            }
        }
    }

    // dbg!(&areas);
    // dbg!(&side_counts);
    // dbg!(&counted);
    let mut sum: usize = 0;
    for region_id in areas.keys() {
        sum += areas.get(region_id).unwrap() * side_counts.get(region_id).unwrap();
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
    fn b_tiny_box() {
        let result = run_b("111\n101\n111");
        assert_eq!(result, 68);
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