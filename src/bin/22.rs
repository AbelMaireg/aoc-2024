use std::collections::{HashMap, HashSet};

advent_of_code::solution!(22);

struct SecretIter {
    state: i64,
    first_yielded: bool,
}

impl SecretIter {
    fn new(initial: i64) -> Self {
        Self {
            state: initial,
            first_yielded: false,
        }
    }
}

impl Iterator for SecretIter {
    type Item = i64;
    fn next(&mut self) -> Option<i64> {
        if !self.first_yielded {
            self.first_yielded = true;
            return Some(self.state);
        }
        self.state = {
            let mut secret = self.state;
            const MASK: i64 = 0xFFFFFF;

            secret = (secret ^ (secret << 6)) & MASK;
            secret = (secret ^ (secret >> 5)) & MASK;
            secret = (secret ^ (secret << 11)) & MASK;

            secret
        };
        Some(self.state)
    }
}

pub fn part_one(input: &str) -> Option<i64> {
    let s = input
        .lines()
        .map(|s| {
            let s = s.parse().unwrap();
            let mut it = SecretIter::new(s);
            it.nth(2000).unwrap()
        })
        .sum();

    Some(s)
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut global: HashMap<[i8; 4], i64> = HashMap::new();

    for initial in input.lines().map(|s| s.parse().unwrap()) {
        // Generate 2001 prices (initial + 2000 generated)
        let prices: Vec<i8> = SecretIter::new(initial)
            .take(2001)
            .map(|s| (s % 10) as i8)
            .collect();

        // Track which 4-diff keys we've already used for this buyer (distinctBy)
        let mut seen: HashSet<[i8; 4]> = HashSet::new();

        // Slide window of 5 prices -> each window produces 4 diffs and a last price
        for i in 0..=(prices.len() - 5) {
            let w = &prices[i..i + 5]; // 5 prices
            let key = [w[1] - w[0], w[2] - w[1], w[3] - w[2], w[4] - w[3]];
            // If this is the first time this buyer produced this key, count it
            if seen.insert(key) {
                let last_price = w[4] as i64;
                *global.entry(key).or_insert(0) += last_price;
            }
        }
    }

    // Return maximum aggregated profit (or 0 if none)
    Some(*global.values().max().unwrap_or(&0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(37990510));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(23));
    }
}
