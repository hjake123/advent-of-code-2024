pub fn run_a(input: &str) -> u64 {
    let mut fs = expand_fs_map(input);
    compact(&mut fs);
    let mut checksum: u64 = 0;
    for pos in 0..fs.len() {
        if fs[pos] == -1 {
            break;
        }
        let posu: u64 = pos.try_into().unwrap();
        let numu: u64 = fs[pos].try_into().unwrap();
        let resu: u64 = numu * posu;
        checksum = checksum + resu;
    }  
    checksum
}

pub fn run_b(input: &str) -> u64 {
    let mut fs = expand_fs_map(input);
    whole_file_compact(&mut fs);
    let mut checksum: u64 = 0;
    for pos in 0..fs.len() {
        if fs[pos] == -1 {
            continue;
        }
        let posu: u64 = pos.try_into().unwrap();
        let numu: u64 = fs[pos].try_into().unwrap();
        let resu: u64 = numu * posu;
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
    let mut i = fs.len() - 1;
    'mainloop: while i > 0 {
        if fs[i] == -1 {
            i -= 1;
            continue;
        }
        let current_id = fs[i];
        let mut filesize = 0;
        while fs[i] == current_id {
            if i == 0 {
                break 'mainloop;
            }
            i -= 1;
            filesize += 1;
        }
        let opening = find_empty_space(fs, filesize, i + 1);
        if opening.is_some() {
            let opening = opening.unwrap();
            for offset in 0..filesize {
                fs[opening + offset] = current_id;
                fs[i + offset + 1] = -1;
            }
        }
        
    }
}

fn find_empty_space(fs: &Vec<i32>, amount: usize, before: usize) -> Option<usize> {
    if before < amount {
        return None
    }
    'checkloop: for i in 0..(before - amount + 1) {
        for j in 0..amount {
            if fs[i + j] != -1 {
                continue 'checkloop;
            }
        }
        return Some(i);
    }
    None
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

    fn from_string(input: &str) -> Vec<i32> {
        let mut vec: Vec<i32> = Vec::new();
        for ch in input.chars() {
            if ch == '.' {
                vec.push(-1);
            } else if ch.is_digit(10) {
                vec.push(ch.to_digit(10).unwrap().try_into().unwrap());
            }
        }
        vec
    }

    #[test]
    fn wfc1() {
        let mut vec = from_string("000..1.22");
        whole_file_compact(&mut vec);
        assert_eq!(vec, from_string("000221..."));
    }

    #[test]
    fn wfc2() {
        let mut vec = from_string("000....1111111.22");
        whole_file_compact(&mut vec);
        assert_eq!(vec, from_string("00022..1111111..."));
    }

    #[test]
    fn wfc3() {
        let mut vec = from_string("000.1..222222.33");
        whole_file_compact(&mut vec);
        assert_eq!(vec, from_string("0001.33222222..."));
    }

    #[test]
    fn wfc4() {
        let mut vec = from_string("0.....1111.22..");
        whole_file_compact(&mut vec);
        assert_eq!(vec, from_string("022...1111....."));
    }

    #[test]
    fn wfc5() {
        let mut vec = from_string("0.1.2.3.4.5.6");
        whole_file_compact(&mut vec);
        assert_eq!(vec, from_string("0615243......"));
    }

    #[test]
    fn wfc6() {
        let mut vec = from_string("....0000");
        whole_file_compact(&mut vec);
        assert_eq!(vec, from_string("0000...."));
    }

    #[test]
    fn fes1() {
        let vec = from_string("000..1.22");
        let res = find_empty_space(&vec, 2, 5);
        assert_eq!(res, Some(3));
    }

    #[test]
    fn fes2() {
        let vec = from_string("000..1.22");
        let res = find_empty_space(&vec, 2, 4);
        assert_eq!(res, None);
    }
}