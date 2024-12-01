
pub fn run(input: &str) -> usize {
    input.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = run("apple");
        assert_eq!(result, 5);
    }
}
