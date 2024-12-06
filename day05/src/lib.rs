use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::str::Lines;

pub fn run_a(input: &str) -> i32 {
    let mut sum = 0;
    let (after_rules, before_rules, lines) = parse_rules(input);
    for update in lines {
        let pages = parse_pages(update, &after_rules, &before_rules);
        let mut sorted = pages.to_vec();
        sorted.sort();
        if sorted == pages {
            sum += pages[pages.len()/2].number;
        }
    }
    sum
}

type Rules = RefCell<Vec<i32>>;
type Ruleset = HashMap<i32, Rules>;

/*
Parses the rules and returns:
- the map of rules about which come AFTER a specific number
- the map of rules about which come BEFORE a specific number
- the iterator of the lines of input after the rules have been parsed
*/
fn parse_rules(input: &str) -> (Ruleset, Ruleset, Lines<'_>) {
    let mut after_rules: Ruleset = HashMap::new();
    let mut before_rules: Ruleset = HashMap::new();


    let mut lines = input.lines();
    loop {
        let line = lines.next().unwrap();
        if line.is_empty() {
            break;
        }
        let mut split_line = line.split('|');
        let left: i32 = split_line.next().unwrap().parse().unwrap();
        let right: i32 = split_line.next().unwrap().parse().unwrap();

        if !after_rules.contains_key(&left) {
            after_rules.insert(left, RefCell::new(Vec::new()));
        }
        after_rules.get(&left).unwrap().borrow_mut().push(right);

        if !before_rules.contains_key(&right) {
            before_rules.insert(right, RefCell::new(Vec::new()));
        }
        before_rules.get(&right).unwrap().borrow_mut().push(left);
    }
    return (after_rules, before_rules, lines);
}

#[derive(Debug)]
struct Page<'a> {
    number: i32,
    after_rules: Option<&'a Rules>,
    before_rules: Option<&'a Rules>
}

impl<'a> Page<'a> {
    fn new(s: &str, after: &'a Ruleset, before: &'a Ruleset) -> Result<Self, String> {
        let n = s.parse();
        if n.is_err() {
            let mut msg = String::new();
            msg += "Could not parse page ";
            msg += s;
            return Err(msg)
        }
        let n = n.unwrap();
        Ok(
            Page {
                number: n,
                after_rules: after.get(&n),
                before_rules: before.get(&n)
            }
        )
    }
}

impl<'a> PartialEq for Page<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.number == other.number
    }
}

impl<'a> Eq for Page<'a> {}

impl<'a> PartialOrd for Page<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other))
    }
}

impl<'a> Ord for Page<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.after_rules.is_some_and(|rules| rules.borrow().contains(&other.number)){
            return Ordering::Less;
        }
        if self.before_rules.is_some_and(|rules| rules.borrow().contains(&other.number)){
            return Ordering::Greater;
        }
        return Ordering::Equal;
    }
}

impl<'a> Clone for Page<'a> {
    fn clone(&self) -> Self {
        Page { 
            number: self.number, 
            after_rules: self.after_rules, 
            before_rules: self.before_rules 
        }
    }
}

fn parse_pages<'a>(line: &str, after: &'a Ruleset, before: &'a Ruleset) -> Vec<Page<'a>> {
    line.split(',').map(|symbol| Page::new(symbol, after, before).unwrap()).collect()
}


pub fn run_b(input: &str) -> i32 {
    let mut sum = 0;
    let (after_rules, before_rules, lines) = parse_rules(input);
    for update in lines {
        let pages = parse_pages(update, &after_rules, &before_rules);
        let mut sorted = pages.to_vec();
        sorted.sort();
        if sorted != pages {
            sum += sorted[sorted.len()/2].number;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn a() {
        let result = run_a(&fs::read_to_string("./test.txt").unwrap());
        assert_eq!(result, 143);
    }

    #[test]
    fn b() {
        let result = run_b(&fs::read_to_string("./test.txt").unwrap());
        assert_eq!(result, 123);
    }
}