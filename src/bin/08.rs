use std::collections::{HashMap, HashSet};

advent_of_code::solution!(8);

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Antenna {
    x: i64,
    y: i64,
}

#[allow(dead_code)]
impl Antenna {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    fn slope(&self, other: &Self) -> (i64, i64) {
        (self.x - other.x, self.y - other.y)
    }

    fn go_in_line(&mut self, slope: (i64, i64)) {
        self.x += slope.0;
        self.y += slope.1;
    }

    fn is_in_limit(&self, x_limit: i64, y_limit: i64) -> bool {
        self.x < x_limit && self.x > -1 && self.y < y_limit && self.y > -1
    }

    fn antinode(&self, other: &Self, x_limit: i64, y_limit: i64) -> Option<Antenna> {
        if self == other {
            return None;
        }

        let mut antinode = *self;
        antinode.go_in_line(self.slope(other));

        if antinode.is_in_limit(x_limit, y_limit) {
            return Some(antinode);
        }

        None
    }

    fn resonating_antinode(&self, other: &Self, x_limit: i64, y_limit: i64) -> Vec<Antenna> {
        let mut antinodes = Vec::new();
        let slope = self.slope(other);
        let negetive_slope = (-slope.0, -slope.1);
        let mut antinode = *self;

        while antinode.is_in_limit(x_limit, y_limit) {
            antinodes.push(antinode);
            antinode.go_in_line(slope);
        }

        let mut antinode = *other;

        while antinode.is_in_limit(x_limit, y_limit) {
            antinodes.push(antinode);
            antinode.go_in_line(negetive_slope);
        }

        antinodes
    }
}

#[allow(dead_code)]
#[derive(Debug, Default)]
struct City {
    x_limit: i64,
    y_limit: i64,
    antennas: HashMap<char, HashSet<Antenna>>,
}

#[allow(dead_code)]
impl City {
    fn new(input: &str) -> Self {
        let mut antennas: HashMap<char, HashSet<Antenna>> = HashMap::new();

        input.lines().enumerate().for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, key)| {
                if key != '.' {
                    antennas
                        .entry(key)
                        .or_default()
                        .insert(Antenna::new(x as i64, y as i64));
                }
            });
        });

        Self {
            x_limit: input.lines().nth(0).unwrap().chars().count() as i64,
            y_limit: input.lines().count() as i64,
            antennas,
        }
    }

    fn find_antinodes(&self) -> i64 {
        let mut antinode_set: HashSet<Antenna> = HashSet::new();

        for antennas_per_char in self.antennas.values() {
            for antenna in antennas_per_char.iter() {
                for other in antennas_per_char.iter() {
                    if let Some(antinode) = antenna.antinode(other, self.x_limit, self.y_limit) {
                        antinode_set.insert(antinode);
                    }
                }
            }
        }

        antinode_set.len() as i64
    }

    fn find_resonating_antinodes(&self) -> i64 {
        let mut antinode_set: HashSet<Antenna> = HashSet::new();

        for antennas_per_char in self.antennas.values() {
            for antenna in antennas_per_char.iter() {
                for other in antennas_per_char.iter() {
                    if antenna == other {
                        continue;
                    }
                    antinode_set.extend(antenna.resonating_antinode(
                        other,
                        self.x_limit,
                        self.y_limit,
                    ));
                }
            }
        }

        antinode_set.len() as i64
    }
}

pub fn part_one(input: &str) -> Option<i64> {
    let city = City::new(input);
    Some(city.find_antinodes())
}

pub fn part_two(input: &str) -> Option<i64> {
    let city = City::new(input);
    Some(city.find_resonating_antinodes())
}

#[cfg(test)]
mod day_08 {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
