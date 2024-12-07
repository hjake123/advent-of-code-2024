/*
Pattern number must be within [0, 2^(numbers.len() - 1))
*/
fn test_expression(numbers: &Vec<i64>, pattern_number: i32, width: usize, target: i64) -> bool {
    let mut numbers = numbers.iter();
    let mut res: i64 = *numbers.next().expect("No numbers!");
    let pattern = gen_pattern(pattern_number.try_into().unwrap(), width.try_into().unwrap());
    // 0 in the pattern means +, 1 means *, 2 means ||
    for digit in pattern.chars() {
        match digit {
            '0' => {
                res += *numbers.next().expect("Ran out of numbers!");
            }
            '1' => {
                res *= *numbers.next().expect("Ran out of numbers!");

            }
            '2' => {
                let mut strres = res.to_string();
                strres += &*numbers.next().expect("Ran out of numbers!").to_string();
                res = strres.parse().expect(&("Invalid concatination of ".to_owned() + &strres));
            }
            _ => {}
        }
    }
    res == target
}

fn gen_pattern(pattern_number: u32, width: u32) -> String {
    let mut pattern = String::new();
    for d in 0..width {
        let n: u32 = (pattern_number / 3_u32.pow(d)) % 3;
        pattern.insert(0, char::from_digit(n, 3).expect("Made an invalid digit!"));
    }
    pattern
}

pub fn test_variants(target: i64, numbers: &Vec<i64>) -> bool {
    for pn in 0..(3_i32.pow((numbers.len() - 1).try_into().unwrap())) {
        if test_expression(&numbers, pn, numbers.len() - 1, target) {
            return true
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_pattern_gen() {
        let result = gen_pattern(7, 3);
        assert_eq!(result, "021");
    }
}