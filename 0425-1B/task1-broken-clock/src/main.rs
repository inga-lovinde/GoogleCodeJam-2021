use std::{io::{self, BufRead}};

const MODULO: u128 = 43_200_000_000_000;
const HOUR: u128 = 3_600_000_000_000;
const MINUTE: u128 = 60_000_000_000;
const SECOND: u128 = 1_000_000_000;
const REVERSE_11: u128 = 15709090909091;
const REVERSE_59: u128 = 41735593220339;
const REVERSE_719: u128 = 29621140472879;

fn solve_for_permutation(hc: u128, mc: u128, sc: u128) -> Option<u128> {
    //println!("{} {} {}", hc, mc, sc);
    let c1 = ((12*hc + (MODULO - mc))*REVERSE_11) % MODULO;
    let c2 = ((60*mc + (MODULO - sc))*REVERSE_59) % MODULO;
    let c3 = ((720*hc + (MODULO - sc))*REVERSE_719) % MODULO;
    //println!("{} {} {}", c1, c2, c3);

    if c1 != c2 || c2 != c3 {
        return None;
    }

    return Some((hc + (MODULO - c1)) % MODULO);
}

fn solve(a: u128, b: u128, c: u128) -> Option<(u128, u128, u128, u128)> {
    let permutations = [(a, b, c), (a, c, b), (b, a, c), (b, c, a), (c, a, b), (c, b, a)];
    for &(hc, mc, sc) in permutations.iter() {
        match solve_for_permutation(hc, mc, sc) {
            Some(timestamp) => {
                let hours = timestamp / HOUR;
                let hours_remainder = timestamp % HOUR;
                let minutes = hours_remainder / MINUTE;
                let minutes_remainder = hours_remainder % MINUTE;
                let seconds = minutes_remainder / SECOND;
                let nanoseconds = minutes_remainder % SECOND;
                return Some((hours, minutes, seconds, nanoseconds));
            },
            _ => {},
        }
    }

    return None;
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let test_number: u32 = lines.next().unwrap().unwrap().parse().unwrap();
    for t in 1..=test_number {
        let parts: Vec<u128> = lines.next().unwrap().unwrap().split(char::is_whitespace).map(|part| part.parse().unwrap()).collect();
        let a = parts[0];
        let b = parts[1];
        let c = parts[2];
        println!("Case #{}: {}", t, match solve(a, b, c) {
            Some((h, m, s, ns)) => format!("{} {} {} {}", h, m, s, ns),
            None => String::from("IMPOSSIBLE"),
        });
    }
}
