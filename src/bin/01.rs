#![allow(dead_code)]
use std::collections::HashMap;

advent_of_code::solution!(1);

struct List {
    items: [Vec<i32>; 2],
}

impl List {
    fn new(input: &str) -> Self {
        let mut ls = input
            .lines()
            .fold([Vec::<i32>::new(), Vec::<i32>::new()], |mut acc, line| {
                let mut it = line.split_whitespace();

                let left_num = it.next().unwrap().parse::<i32>().unwrap();
                let right_num = it.next().unwrap().parse::<i32>().unwrap();

                acc[0].push(left_num);
                acc[1].push(right_num);

                acc
            });

        ls[0].sort();
        ls[1].sort();
        List { items: ls }
    }

    /**
     * For each pair of numbers in the two lists, compute the absolute difference
     * and return the sum of all differences.
     */
    fn run(&self) -> i32 {
        self.items[0]
            .iter()
            .zip(self.items[1].iter())
            .fold(0, |acc, (&l, &r)| acc + (l - r).abs())
    }

    /**
     * For each unique number in the first list, count its occurrences and multiply
     * by the count of the same number in the second list, then sum all these products.
     */
    fn run2(&self) -> i32 {
        let l1_mapped = self.items[0]
            .iter()
            .fold(HashMap::<i32, i32>::new(), |mut acc, &x| {
                *acc.entry(x).or_insert(0) += 1;
                acc
            });

        let l2_mapped = self.items[1]
            .iter()
            .fold(HashMap::<i32, i32>::new(), |mut acc, &x| {
                *acc.entry(x).or_insert(0) += 1;
                acc
            });

        l1_mapped.iter().fold(0, |acc, (&k, &v)| {
            acc + k * v * l2_mapped.get(&k).unwrap_or(&0)
        })
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let list = List::new(input);
    Some(list.run() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let list = List::new(input);
    Some(list.run2() as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
