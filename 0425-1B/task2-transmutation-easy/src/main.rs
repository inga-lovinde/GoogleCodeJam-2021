use std::{io::{self, BufRead}};

fn is_subset(target: &[usize], current: &[usize]) -> bool {
    for i in 0..target.len() {
        if (target[i] > current[i]) {
            return false;
        }
    }

    return true;
}

fn is_solution(n: usize, target: &[usize]) -> bool {
    let mut current = vec![0; n];
    current[n-1] = 1;
    for i in (1..n).rev() {
        let required_remainder = if i < target.len() { target[i] } else { 0 };
        if required_remainder > current[i] {
            return false;
        }

        let to_transmute = current[i] - required_remainder;
        current[i] -= to_transmute;
        current[i-1] += to_transmute;
        if i >= 2 {
            current[i-2] += to_transmute;
        }

        if is_subset(&target, &current) {
            return true;
        }
    }

    return false;
}

fn solve(target: &[usize]) -> usize {
    for i in target.len().. {
        if is_solution(i, &target) {
            return i;
        }
    }

    panic!("Impossible");
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let test_number: u32 = lines.next().unwrap().unwrap().parse().unwrap();
    for t in 1..=test_number {
        let _nab = lines.next().unwrap().unwrap();
        let list: Vec<usize> = lines.next().unwrap().unwrap().split(char::is_whitespace).map(|part| part.parse().unwrap()).collect();
        println!("Case #{}: {}", t, solve(&list));
    }
}
