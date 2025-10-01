advent_of_code::solution!(2);

struct Report {
    levels: Vec<Vec<i32>>,
}

#[allow(dead_code)]
impl Report {
    fn new(input: &str) -> Self {
        let levels = input.lines().fold(Vec::<Vec<i32>>::new(), |mut acc, line| {
            let it = line.split_whitespace();
            let mut row = Vec::new();
            for num in it {
                row.push(num.parse::<i32>().unwrap());
            }
            acc.push(row);
            acc
        });

        Report { levels }
    }

    fn run(&self) -> i32 {
        let mut count = 0;

        for level in &self.levels {
            if Self::increasing(level) || Self::decreasing(level) {
                count += 1;
            }
        }

        count
    }

    fn run2(&self) -> i32 {
        let mut count = 0;

        for level in &self.levels {
            if self.every_but_one(level, Self::increasing)
                || self.every_but_one(level, Self::decreasing)
            {
                count += 1;
            }
        }

        count
    }

    fn decreasing(level: &[i32]) -> bool {
        level.windows(2).all(|w| w[0] > w[1] && w[0] - w[1] < 4)
    }

    fn increasing(level: &[i32]) -> bool {
        level.windows(2).all(|w| w[0] < w[1] && w[1] - w[0] < 4)
    }

    fn cut(level: &[i32], idx: usize) -> Vec<i32> {
        let mut res = Vec::from(level);
        res.remove(idx);
        res
    }

    fn every_but_one(&self, level: &[i32], f: fn(&[i32]) -> bool) -> bool {
        if f(level) {
            return true;
        }

        (0..level.len()).any(|i| f(&Self::cut(level, i)))
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let report = Report::new(input);
    Some(report.run() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let report = Report::new(input);
    Some(report.run2() as u64)
}

#[cfg(test)]
mod day_02 {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
