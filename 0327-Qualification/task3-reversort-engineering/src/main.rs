use std::{io::{self, BufRead}};

fn create_list(len: usize, required_cost: usize, min_value: usize) -> Option<Vec<usize>> {
    if len == 0 {
        if required_cost == 0 {
            return Some(Vec::new());
        } else {
            return None;
        }
    }

    let potential_step_cost = len-1;
    if potential_step_cost <= required_cost {
        let mut result = create_list(len-1, required_cost - potential_step_cost, min_value + 1)?;
        result.push(min_value);
        result.reverse();
        return Some(result);
    } else {
        let mut result = create_list(len-1, required_cost, min_value + 1)?;
        result.push(min_value);
        return Some(result);
    }
}

fn solve(n: usize, c: usize) -> Option<Vec<usize>> {
    if c + 1 < n {
        return None;
    }

    let mut result = create_list(n, (c + 1) - n, 1)?;
    result.reverse();
    return Some(result);
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let test_number: u32 = lines.next().unwrap().unwrap().parse().unwrap();
    for t in 1..=test_number {
        let parts: Vec<usize> = lines.next().unwrap().unwrap().split(char::is_whitespace).map(|part| part.parse().unwrap()).collect();
        let n = parts[0];
        let c = parts[1];
        println!("Case #{}: {}", t, match solve(n, c) {
            Some(list) => list.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(" "),
            None => String::from("IMPOSSIBLE"),
        });
    }
}
