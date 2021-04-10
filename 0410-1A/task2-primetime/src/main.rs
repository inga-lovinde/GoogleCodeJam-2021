use std::collections::HashMap;
use std::{io::{self, BufRead}};

const PRIMES: [u16; 95] = [2,3,5,7,11,13,17,19,23,29,31,37,41,43,47,53,59,61,67,71,73,79,83,89,97,101,103,107,109,113,127,131,137,139,149,151,157,163,167,173,179,181,191,193,197,199,211,223,227,229,233,239,241,251,257,263,269,271,277,281,283,293,307,311,313,317,331,337,347,349,353,359,367,373,379,383,389,397,401,409,419,421,431,433,439,443,449,457,461,463,467,479,487,491,499];

fn factorize(mut number: u16) -> Option<HashMap<u16, u16>> {
    let mut result = HashMap::new();

    for &prime in &PRIMES {
        let mut count = 0;
        while (number % prime) == 0 {
            count += 1;
            number = number / prime;
        }

        if count > 0 {
            result.insert(prime, count);
        }
    }

    if number == 1 {
        return Some(result);
    } else {
        return None;
    }
}

fn factorize_all() -> HashMap<u16, HashMap<u16, u16>> {
    let mut result = HashMap::new();

    for i in 1..u16::MAX {
        match factorize(i) {
            Some(factors) => {
                result.insert(i, factors);
            },
            None => {}
        }
    }

    result
}

fn solve(cards: &Vec<(u16, u16)>, precomputed: &HashMap<u16, HashMap<u16, u16>>) -> Option<u16> {
    let sum: u16 = cards.iter().map(|&(prime, count)| prime * count).sum();
    let cards_hash = cards.iter().map(|&tuple| tuple.clone()).collect::<HashMap<_, _>>();

    for i in (1..=sum).rev() {
        match precomputed.get(&i) {
            Some(factors) => {
                let factors_sum: u16 = factors.iter().map(|(&factor, &count)| factor * count).sum();
                if i + factors_sum == sum {
                    if factors.iter().all(|(&factor, &count)| match cards_hash.get(&factor) {
                        Some(&original_count) => original_count >= count,
                        None => false,
                    }) {
                        return Some(i);
                    }
                }
            }
            None => {
            }
        }
    }

    return None;
}

fn main() {
    let precomputed = factorize_all();

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let test_number: u32 = lines.next().unwrap().unwrap().parse().unwrap();
    for t in 1..=test_number {
        let length: u32 = lines.next().unwrap().unwrap().parse().unwrap();
        let mut list: Vec<(u16, u16)> = vec![];
        for _i in 0..length {
            let parts: Vec<u16> = lines.next().unwrap().unwrap().split(char::is_whitespace).map(|part| part.parse().unwrap()).collect();
            list.push((parts[0], parts[1]))
        }
        println!("Case #{}: {}", t, match solve(&list, &precomputed) {
            Some(score) => score,
            None => 0,
        });
    }
}
