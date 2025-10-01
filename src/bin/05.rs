use std::collections::{HashMap, HashSet};

advent_of_code::solution!(5);

#[allow(dead_code)]
struct Page {
    orders: HashMap<usize, HashSet<usize>>,
    updates: Vec<Vec<usize>>,
}

#[allow(dead_code)]
impl Page {
    fn new(input: &str) -> Self {
        let (orders_data, prods_data) = input.split_once("\n\n").unwrap();

        let orders = orders_data.lines().fold(HashMap::new(), |mut acc, line| {
            let (before, after) = line.split_once("|").unwrap();
            let before = before.parse::<usize>().unwrap();
            let after = after.parse::<usize>().unwrap();

            acc.entry(before).or_insert_with(HashSet::new).insert(after);

            acc
        });

        let prods = prods_data.lines().fold(Vec::new(), |mut acc, line| {
            let update = line
                .split(",")
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            acc.push(update);
            acc
        });

        Self {
            orders,
            updates: prods,
        }
    }

    fn is_valid_page(&self, prod: &[usize]) -> bool {
        for w in prod.windows(2) {
            if let Some(set) = self.orders.get(&w[0]) {
                if !set.contains(&w[1]) {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    }

    fn run(&self) -> usize {
        let mut middles = 0;

        for prod in self.updates.iter() {
            if self.is_valid_page(prod) {
                middles += prod[prod.len() / 2];
            }
        }

        middles
    }

    fn run2(&self) -> usize {
        let mut middles = 0;

        for prod in self.updates.iter() {
            if !self.is_valid_page(prod) {
                let mut p = prod.clone();

                p.sort_by(|a, b| match self.orders.get(a) {
                    Some(set) => {
                        if set.contains(b) {
                            std::cmp::Ordering::Less
                        } else {
                            std::cmp::Ordering::Greater
                        }
                    }
                    None => std::cmp::Ordering::Greater,
                });

                middles += p[p.len() / 2];
            }
        }

        middles
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let page = Page::new(input);
    Some(page.run() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let page = Page::new(input);
    Some(page.run2() as u64)
}

#[cfg(test)]
mod day_05 {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
