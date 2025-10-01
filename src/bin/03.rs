use regex::Regex;

advent_of_code::solution!(3);

#[allow(dead_code)]
#[derive(Default)]
struct Program {
    input: String,
    instructions: Vec<(usize, i32, i32)>,
    dos: Vec<(usize, bool)>,
}

#[allow(dead_code)]
impl Program {
    fn new(input: String) -> Self {
        Program {
            input,
            ..Default::default()
        }
    }

    fn parse(&mut self) {
        let regex = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
        for caps in regex.find_iter(&self.input) {
            let nums = caps
                .as_str()
                .split(&['(', ')', ','][..])
                .collect::<Vec<_>>();

            self.instructions.push((
                caps.start(),
                nums[1].parse::<i32>().unwrap(),
                nums[2].parse::<i32>().unwrap(),
            ));
        }
    }

    fn parse_dos(&mut self) {
        let regex = Regex::new(r"(do\(\)|don't\(\))").unwrap();

        for caps in regex.find_iter(&self.input) {
            if caps.as_str() == "do()" {
                self.dos.push((caps.start(), true))
            } else {
                self.dos.push((caps.start(), false))
            }
        }
    }

    fn run(&mut self) -> i32 {
        self.parse();
        self.instructions
            .iter()
            .fold(0, |acc, (_, a, b)| acc + a * b)
    }

    fn run2(&mut self) -> i32 {
        self.parse();
        self.parse_dos();

        let mut acc = 0;
        acc += self.instructions[0].1 * self.instructions[0].2;

        for &(idx, a, b) in self.instructions.iter().skip(1) {
            let flag = self.dos.iter().rev().find(|(dos_idx, _dos)| *dos_idx < idx);

            if flag.is_none() || (flag.is_some() && flag.unwrap().1) {
                acc += a * b;
            }
        }

        acc
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut program = Program::new(input.to_string());
    Some(program.run() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut program = Program::new(input.to_string());
    Some(program.run2() as u64)
}

#[cfg(test)]
mod day_03 {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}
