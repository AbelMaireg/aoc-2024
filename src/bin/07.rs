advent_of_code::solution!(7);

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Equation {
    lhs: u64,
    rhs: Vec<u64>,
}

impl Equation {
    fn new(lhs: u64, rhs: Vec<u64>) -> Self {
        Self { lhs, rhs }
    }

    fn eval(&self, concat: bool) -> bool {
        let last_element = self.rhs.last().unwrap().to_owned();

        if self.rhs.len() == 1 {
            return last_element == self.lhs;
        }

        if concat {
            let lhs = self.lhs.to_string();
            let last_element = last_element.to_string();
            if lhs.ends_with(&last_element) {
                let rhs = &self.rhs[..self.rhs.len() - 1];

                let equ = Equation::new(
                    lhs.strip_suffix(&last_element)
                        .unwrap()
                        .parse::<u64>()
                        .unwrap_or(0),
                    rhs.to_vec(),
                );

                if equ.eval(concat) {
                    return true;
                }
            }
        }

        if self.lhs.is_multiple_of(last_element) {
            let rhs = &self.rhs[..self.rhs.len() - 1];

            let equ = Equation::new(self.lhs / last_element, rhs.to_vec());

            if equ.eval(concat) {
                return true;
            }
        }

        if self.lhs.saturating_sub(last_element) != 0 {
            let rhs = &self.rhs[..self.rhs.len() - 1];

            let equ = Equation::new(self.lhs.saturating_sub(last_element), rhs.to_vec());

            if equ.eval(concat) {
                return true;
            }
        }

        false
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct All {
    equations: Vec<Equation>,
}

#[allow(dead_code)]
impl All {
    fn new(input: &str) -> Self {
        let equations = input
            .lines()
            .map(|line| {
                let (left, right) = line.split_once(":").unwrap();
                let left = left.trim().parse::<u64>().unwrap();

                let right: Vec<u64> = right
                    .split_whitespace()
                    .map(|s| s.parse::<u64>().unwrap())
                    .collect();

                Equation {
                    lhs: left,
                    rhs: right,
                }
            })
            .collect();

        Self { equations }
    }

    fn calibration(&self, concat: bool) -> u64 {
        self.equations
            .iter()
            .filter(|eq| eq.eval(concat))
            .map(|eq| eq.lhs)
            .sum()
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let all = All::new(input);
    Some(all.calibration(false))
}

pub fn part_two(input: &str) -> Option<u64> {
    let all = All::new(input);
    Some(all.calibration(true))
}

#[cfg(test)]
mod day_07 {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
