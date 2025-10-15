advent_of_code::solution!(15);

#[allow(dead_code)]
struct Warehouse {
    width: i64,
    height: i64,
    grid: Vec<Vec<char>>,
    motions: Vec<(i64, i64)>,
    robot: (i64, i64),
}

impl std::fmt::Display for Warehouse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.grid {
            for &cell in row {
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[allow(dead_code)]
impl Warehouse {
    fn new(input: &str) -> Self {
        let mut robot: (i64, i64) = (0, 0);
        let mut sections = input.split("\n\n");
        let grid: Vec<Vec<char>> = sections
            .next()
            .unwrap()
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| {
                        if c == '@' {
                            robot = (x as i64, y as i64);
                            return '.';
                        }
                        c
                    })
                    .collect()
            })
            .collect();

        let motions = sections
            .next()
            .unwrap()
            .lines()
            .flat_map(str::chars)
            .filter_map(|c| match c {
                '>' => Some((1, 0)),
                '<' => Some((-1, 0)),
                '^' => Some((0, -1)),
                'v' => Some((0, 1)),
                _ => None,
            })
            .collect();

        Self {
            width: grid[0].len() as i64,
            height: grid.len() as i64,
            grid,
            robot,
            motions,
        }
    }

    fn get(&self, pos: (i64, i64)) -> char {
        self.grid[pos.1 as usize][pos.0 as usize]
    }

    fn get_mut(&mut self, pos: (i64, i64)) -> &mut char {
        &mut self.grid[pos.1 as usize][pos.0 as usize]
    }

    fn attempt(&mut self, motion: (i64, i64)) {
        let robot = self.robot;
        let mut pos_iter = (1..).map(|i| (robot.0 + motion.0 * i, robot.1 + motion.1 * i));

        let mut np = pos_iter.next().unwrap();

        if self.get(np) == '#' {
            return;
        }

        while self.get(np) == 'O' {
            np = pos_iter.next().unwrap();
            if self.get(np) == '#' {
                return;
            }
        }

        *self.get_mut(np) = 'O';
        self.robot = (self.robot.0 + motion.0, self.robot.1 + motion.1);
        *self.get_mut(self.robot) = '.';
    }

    fn exec(&mut self) {
        for motion in self.motions.clone() {
            self.attempt(motion);
        }
    }

    fn count_boxes(&self) -> u64 {
        let mut count = 0;
        for (y, row) in self.grid.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                if cell == 'O' {
                    count += y * 100 + x
                }
            }
        }

        count as u64
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut warehouse = Warehouse::new(input);
    warehouse.exec();
    // println!("{}", warehouse);
    Some(warehouse.count_boxes())
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
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, None);
    }
}
