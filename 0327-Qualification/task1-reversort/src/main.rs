use std::{io::{self, BufRead}};

trait PositionMinExt {
    fn position_min(&self) -> usize;
}

impl<T: Ord> PositionMinExt for [T] {
    fn position_min(&self) -> usize {
        self.iter().enumerate().min_by_key(|&(_position, value)| value).unwrap().0
    }
}

fn solve(mut list: Vec<u32>) -> usize {
    let mut result = 0;

    for i in 0..list.len()-1 {
        let j = i + list[i..].position_min();
        list[i..=j].reverse();
        result += list[i..=j].len();
    }

    result
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let test_number: u32 = lines.next().unwrap().unwrap().parse().unwrap();
    for t in 1..=test_number {
        let _length: u32 = lines.next().unwrap().unwrap().parse().unwrap();
        let list: Vec<u32> = lines.next().unwrap().unwrap().split(char::is_whitespace).map(|part| part.parse().unwrap()).collect();
        println!("Case #{}: {}", t, solve(list));
    }
}
