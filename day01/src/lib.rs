pub fn run(input: &str) -> usize {
    input.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let result = run(&fs::read_to_string("./test.txt").unwrap());
        assert_eq!(result, 11);
    }
}
