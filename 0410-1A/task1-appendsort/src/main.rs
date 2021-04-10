use std::{cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd}, io::{self, BufRead}, ops::{Add, Index, Shl}};

#[derive(Debug)]
struct BigInt {
    storage: Vec<u8>,
}

impl BigInt {
    fn size(&self) -> usize {
        self.storage.len()
    }
}

impl Index<usize> for BigInt {
    type Output = u8;
    fn index(&self, offset: usize) -> &u8 {
        if offset < self.storage.len() {
            return &self.storage[offset];
        } else {
            return &0;
        }
    }
}

impl Add for &BigInt {
    type Output = BigInt;

    fn add(self, other: Self) -> BigInt {
        let mut result = vec![0; self.size().max(other.size())];
        let mut carry = 0;
        for i in 0..result.len() {
            let mut digit = self[i] + other[i] + carry;
            if digit >= 10 {
                digit -= 10;
                carry = 1;
            } else {
                carry = 0;
            }
            result[i] = digit;
        }

        if carry > 0 {
            result.push(carry);
        }

        BigInt {
            storage: result,
        }
    }
}

impl Shl<usize> for &BigInt {
    type Output = BigInt;

    fn shl(self, rhs: usize) -> BigInt {
        let mut result = vec![0; rhs];
        result.extend_from_slice(&self.storage);

        BigInt {
            storage: result,
        }
    }
}

impl PartialEq for BigInt {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..self.size().max(other.size()) {
            if self[i] != other[i] {
                return false;
            }
        }

        return true;
    }
}

impl Eq for BigInt {
}

impl Ord for BigInt {
    fn cmp(&self, other: &Self) -> Ordering {
        for i in (0..self.size().max(other.size())).rev() {
            //println!("{} {}", self[i], other[i]);
            let comparison = self[i].cmp(&other[i]);
            if comparison != Ordering::Equal {
                //println!("{:?}", comparison);
                return comparison;
            }
        }

        return Ordering::Equal;
    }
}

impl PartialOrd for BigInt {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl From<u32> for BigInt {
    fn from(mut num: u32) -> BigInt {
        let mut result = vec![];

        while num > 0 {
            result.push((num % 10) as u8);
            num = num / 10;
        }

        BigInt {
            storage: result,
        }
    }
}

fn get_next_value(previous_value: &BigInt, current_value: &BigInt) -> (BigInt, usize) {
    let current_value_plus = current_value + (&BigInt::from(1));
    let previous_value_plus = previous_value + (&BigInt::from(1));

    //println!("{:?} {:?} {:?} {:?}", previous_value, current_value, previous_value_plus, current_value_plus);
    for shift in 0..10000usize {
        let current_value_shifted = current_value << shift;
        let current_value_plus_shifted = (&current_value_plus) << shift;
        //println!("{:?} {:?}", current_value_shifted, current_value_plus_shifted);

        if (&current_value_shifted) > previous_value {
            return (current_value_shifted, shift);
        }

        if (&current_value_plus_shifted) > (&previous_value_plus) {
            return (previous_value_plus, shift);
        }
    }

    panic!("too large shift");
}

fn solve(list: Vec<u32>) -> usize {
    let mut result = 0;

    let mut last_value = BigInt::from(0);
    for current_num in list {
        let (next_value, shift) = get_next_value(&last_value, &BigInt::from(current_num));
        last_value = next_value;
        result += shift;
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
