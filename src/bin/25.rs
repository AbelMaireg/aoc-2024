advent_of_code::solution!(25);

struct Cronicle {
    keys: Vec<[i8; 5]>,
    locks: Vec<[i8; 5]>,
}

impl Cronicle {
    fn new(input: &str) -> Self {
        let mut keys = vec![];
        let mut locks = vec![];
        let it = input.split("\n\n");

        it.for_each(|sec| {
            let mut cols = [-1i8; 5];
            let mut sec_it = sec.lines();
            sec.lines().for_each(|line| {
                line.chars().enumerate().for_each(|(i, c)| {
                    if c == '#' {
                        cols[i] += 1;
                    }
                });
            });

            if sec_it.next().unwrap().chars().all(|c| c == '#') {
                locks.push(cols);
            } else {
                keys.push(cols);
            };
        });

        Self { keys, locks }
    }

    fn fits(&self) -> u64 {
        let mut count = 0;

        for key in self.keys.iter() {
            for locks in self.locks.iter() {
                if key.iter().zip(locks.iter()).all(|(k, l)| k + l < 6) {
                    count += 1;
                }
            }
        }

        count
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let cronicle = Cronicle::new(input);
    Some(cronicle.fits())
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
