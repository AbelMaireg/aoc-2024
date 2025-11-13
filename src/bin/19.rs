use std::collections::HashMap;

advent_of_code::solution!(19);

#[derive(Debug)]
struct Suit {
    pattern: Vec<String>,
    design: Vec<String>,
}

impl Suit {
    fn new(input: &str) -> Self {
        let mut line = input.split("\n\n");

        Suit {
            pattern: line.next().unwrap().split(", ").map(String::from).collect(),
            design: line.next().unwrap().lines().map(String::from).collect(),
        }
    }

    fn make_design(&self, design: &str, cache: &mut HashMap<String, u64>) -> u64 {
        if design.is_empty() {
            1
        } else {
            let c = cache.get(design);

            if c.is_none() {
                let sum = self
                    .pattern
                    .iter()
                    .filter(|p| design.starts_with(*p))
                    .map(|p| self.make_design(design.strip_prefix(p).unwrap(), cache))
                    .sum::<u64>();

                cache.insert(design.into(), sum);
            }

            *cache.get(design).unwrap()
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let suit = Suit::new(input);
    let mut cache: HashMap<String, u64> = HashMap::new();

    Some(
        suit.design
            .iter()
            .filter(|d| suit.make_design(d, &mut cache) > 0)
            .count() as u64,
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let suit = Suit::new(input);
    let mut cache: HashMap<String, u64> = HashMap::new();

    Some(
        suit.design
            .iter()
            .map(|d| suit.make_design(d, &mut cache))
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
