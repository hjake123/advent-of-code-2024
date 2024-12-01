use std::fs;

fn main() {
    println!("{}", day00::run(&fs::read_to_string("./test.txt").unwrap()));
}
