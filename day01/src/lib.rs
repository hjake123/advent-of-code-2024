use std::fs;

pub fn run(file_path: &str) -> usize {
    let content = fs::read_to_string(file_path);
    content.unwrap().len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
