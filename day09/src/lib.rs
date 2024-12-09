pub fn run_a(input: &str) -> u128 {
    let mut fs = expand_fs_map(input);
    compact(&mut fs);
    let mut checksum: u128 = 0;
    for pos in 0..fs.len() {
        if fs[pos] == -1 {
            break;
        }
        let posu: u32 = pos.try_into().unwrap();
        let numu: u32 = fs[pos].try_into().unwrap();
        let resu: u128 = (numu * posu).try_into().unwrap();
        checksum = checksum + resu;
    }  
    checksum
}

pub fn run_b(input: &str) -> u128 {
    let mut fs = expand_fs_map(input);
    whole_file_compact(&mut fs);
    let mut checksum: u128 = 0;
    for pos in 0..fs.len() {
        if fs[pos] == -1 {
            break;
        }
        let posu: u32 = pos.try_into().unwrap();
        let numu: u32 = fs[pos].try_into().unwrap();
        let resu: u128 = (numu * posu).try_into().unwrap();
        checksum = checksum + resu;
    }  
    checksum
    
}

fn expand_fs_map(input: &str) -> Vec<i32> {
    let mut fs = Vec::new();
    let mut id = 0;
    let mut writing_file = true;
    for runlen in input.chars() {
        let runlen = runlen.to_digit(10).expect("Invalid number!").try_into().unwrap();
        if writing_file {
            for _ in 0..runlen {
                fs.push(id);
            }
            id += 1;
        } else {
            for _ in 0..runlen {
                fs.push(-1);
            }
        }
        writing_file = !writing_file;
    }
    fs
}

fn contiguous_before(vec: &Vec<i32>, before: usize) -> bool {
    for i in 0..before {
        if vec[i] == -1 {
            return false
        }
    }
    true
}

fn compact(fs: &mut Vec<i32>) {
    for i in (0..fs.len()).rev() {
        if contiguous_before(&fs, i) {
            break;
        }
        if fs[i] != -1 {
            for j in 1..fs.len() {
                if fs[j] == -1 {
                    let temp = fs[i];
                    fs[i] = fs[j];
                    fs[j] = temp;
                    break;
                }
            }

        }
    }
}

fn whole_file_compact(fs: &mut Vec<i32>) {
    for i in (0..fs.len()).rev() {
        if contiguous_before(&fs, i) {
            break;
        }
        if fs[i] != -1 {
            for j in 1..fs.len() {
                if fs[j] == -1 {
                    let temp = fs[i];
                    fs[i] = fs[j];
                    fs[j] = temp;
                    break;
                }
            }

        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn a() {
        let result = run_a(&fs::read_to_string("./test.txt").expect("No test file!"));
        assert_eq!(result, 1928);
    }

    #[test]
    fn b() {
        let result = run_b(&fs::read_to_string("./test.txt").expect("No test file!"));
        assert_eq!(result, 2858);
    }
}