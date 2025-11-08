advent_of_code::solution!(17);

pub fn part_one(input: &str) -> Option<String> {
    let mut r = Register::new(input);
    let output = r.run();

    Some(
        output
            .iter()
            .map(std::string::ToString::to_string)
            .collect::<Vec<_>>()
            .join(","),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let r = Register::new(input);
    r.reverse()
}

#[derive(Debug)]
enum Flag {
    Continue,
    ContinueFrom(usize),
    Out(u64),
}

#[derive(Clone)]
struct Register {
    a: u64,
    b: u64,
    c: u64,
    program: Vec<u8>,
    ptr: usize,
}

impl Register {
    fn new(input: &str) -> Self {
        fn after_colon_reg(line: Option<&str>) -> u64 {
            line.unwrap()
                .split_whitespace()
                .last()
                .unwrap()
                .parse()
                .unwrap()
        }
        let mut it = input.lines();

        let a = after_colon_reg(it.next());
        let b = after_colon_reg(it.next());
        let c = after_colon_reg(it.next());
        it.next(); // skip empty line
        let program = it
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();

        Self {
            a,
            b,
            c,
            program,
            ptr: 0,
        }
    }

    fn current_ins(&self) -> usize {
        self.program[self.ptr] as usize
    }

    fn current_literal(&self) -> u8 {
        self.program[self.ptr + 1]
    }

    fn current_combo(&self) -> u64 {
        match self.current_literal() {
            i if i < 4 => i as u64,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => unreachable!(),
        }
    }

    fn run(&mut self) -> Vec<u64> {
        use Flag::*;
        let mut s = Vec::new();

        while self.ptr < self.program.len() {
            match INSTRUCTIONS[self.current_ins()](self) {
                Continue => self.ptr += 2,
                ContinueFrom(idx) => self.ptr = idx,
                Out(i) => {
                    s.push(i);
                    self.ptr += 2
                }
            }
        }

        s
    }

    fn reverse(&self) -> Option<u64> {
        let program: Vec<u64> = self.program.iter().rev().map(|&op| op.into()).collect();
        let mut candidates = vec![0_u64];
        for &instruction in &program {
            let mut new_candidates = vec![];
            for candidate in candidates {
                let shifted = candidate << 3;
                for add in 0..8 {
                    let attempt = shifted + add;
                    let mut test_reg = self.clone();
                    test_reg.a = attempt;
                    let outputs = test_reg.run();
                    if !outputs.is_empty() && outputs[0] == instruction {
                        new_candidates.push(attempt);
                    }
                }
            }
            candidates = new_candidates;
        }
        candidates.into_iter().min()
    }
}

const INSTRUCTIONS: [fn(reg: &mut Register) -> Flag; 8] = [
    |reg| {
        reg.a >>= reg.current_combo();
        Flag::Continue
    },
    |reg| {
        reg.b ^= reg.current_literal() as u64;
        Flag::Continue
    },
    |reg| {
        reg.b = reg.current_combo() % 8;
        Flag::Continue
    },
    |reg| {
        if reg.a != 0 {
            return Flag::ContinueFrom(reg.current_literal() as usize);
        }
        Flag::Continue
    },
    |reg| {
        reg.b ^= reg.c;
        Flag::Continue
    },
    |reg| Flag::Out(reg.current_combo() % 8),
    |reg| {
        reg.b = reg.a >> reg.current_combo();
        Flag::Continue
    },
    |reg| {
        reg.c = reg.a >> reg.current_combo();
        Flag::Continue
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("2,1,0,1,7,2,5,0,3".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(267265166222235));
    }
}
