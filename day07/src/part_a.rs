/*
Pattern number must be within [0, 2^(numbers.len() - 1))
*/
fn test_expression(numbers: &Vec<i64>, pattern_number: i32, width: usize, target: i64) -> bool {
    let mut numbers = numbers.iter();
    let mut res = *numbers.next().expect("Tried to test an empty expression!");
    let mut pattern = format!("{pattern_number:.b}");
    while pattern.len() < width {
        pattern.insert(0, '0');
    }
    // 0 in the pattern means +, 1 means *.
    for digit in pattern.chars() {
        match digit {
            '0' => {
                res += numbers.next().expect("Ran out of numbers to use!");
            }
            '1' => {
                res *= numbers.next().expect("Ran out of numbers to use!");
            }
            _ => {}
        }
    }
    res == target
}

pub fn test_variants(target: i64, numbers: &Vec<i64>) -> bool {
    for pn in 0..(2_i32.pow((numbers.len() - 1).try_into().unwrap())) {
        if test_expression(&numbers, pn, numbers.len() - 1, target) {
            return true
        }
    }
    false
}