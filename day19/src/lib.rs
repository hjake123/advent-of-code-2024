use std::collections::HashMap;

fn parse_towels(first_line: &str) -> Vec<String> {
    let mut towels = Vec::new();
    for segment in first_line.replace(",", "").split_whitespace() {
        towels.push(segment.to_owned());
    }
    towels.sort();
    towels
}

fn is_possible(sequence: &str, towels: &Vec<String>) -> bool {
    for towel in towels {
        if sequence == towel {
            return true;
        }
        if sequence.len() < towel.len() {
            continue;
        }
        let splits = sequence.split_at(towel.len());
        if splits.0 == towel {
            // Towel matches the first section of sequence. Recurse.
            if is_possible(splits.1, towels) {
                return true;
            }
        }
    }
    false
}

pub fn run_a(input: &str) -> i32 {
    let mut lines = input.lines();
    let towels = parse_towels(lines.next().expect("Empty input file!"));
    let _ = lines.next();
    let mut possible_count = 0;
    for pattern in lines {
        if is_possible(pattern, &towels) {
            possible_count += 1;
        }
    }
    possible_count
}

fn count_possibilities(sequence: &str, towels: &Vec<String>, cache: &mut HashMap<String, i64>) -> i64 {
    if cache.contains_key(sequence) {
        return *cache.get(sequence).unwrap();
    }
    let mut sum  = 0;
    for towel in towels {
        if sequence == towel {
            sum += 1;
            continue;
        }
        if sequence.len() < towel.len() {
            continue;
        }
        let splits = sequence.split_at(towel.len());
        if splits.0 == towel {
            // Towel matches the first section of sequence. Recurse.
            sum += count_possibilities(splits.1, towels, cache);
        }
    }
    cache.insert(sequence.to_owned(), sum);
    sum
}

pub fn run_b(input: &str) -> i64 {
    let mut lines = input.lines();
    let towels = parse_towels(lines.next().expect("Empty input file!"));
    let _ = lines.next();
    let mut possible_count = 0;
    for pattern in lines {
        let count = count_possibilities(pattern, &towels, &mut HashMap::new());
        possible_count += count;
    }
    possible_count
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn a() {
        let result = run_a(&fs::read_to_string("./test.txt").expect("No test file!"));
        assert_eq!(result, 6);
    }

    #[test]
    fn b() {
        let result = run_b(&fs::read_to_string("./test.txt").expect("No test file!"));
        assert_eq!(result, 16);
    }
}

// 99472399 too low