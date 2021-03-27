use std::{io::{self, BufRead}};

const CHAR_C: char = 'C';
const CHAR_J: char = 'J';

fn solve(cj_price: u32, jc_price: u32, template: &str) -> u32 {
    let chars: Vec<_> = template.chars().filter(|&ch| ch == CHAR_C || ch == CHAR_J).collect();

    let mut result = 0;
    for i in 1..chars.len() {
        result += match (chars[i-1], chars[i]) {
            (CHAR_C, CHAR_J) => cj_price,
            (CHAR_J, CHAR_C) => jc_price,
            _ => 0,
        };
    }

    result
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let test_number: u32 = lines.next().unwrap().unwrap().parse().unwrap();
    for t in 1..=test_number {
        let line = lines.next().unwrap().unwrap();
        let parts: Vec<_> = line.split(char::is_whitespace).collect();
        println!("Case #{}: {}", t, solve(parts[0].parse().unwrap(), parts[1].parse().unwrap(), parts[2]));
    }
}
