use std::collections::HashSet;

advent_of_code::solution!(15);

fn move_(pos: (i64, i64), motion: (i64, i64)) -> (i64, i64) {
    (pos.0 + motion.0, pos.1 + motion.1)
}

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
        for (y, row) in self.grid.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                if (x as i64, y as i64).eq(&self.robot) {
                    write!(f, "@")?;
                } else {
                    write!(f, "{}", cell)?;
                }
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

        let motions = Self::parse_motion(sections.next());

        Self {
            width: grid[0].len() as i64,
            height: grid.len() as i64,
            grid,
            robot,
            motions,
        }
    }

    fn new2(input: &str) -> Self {
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
                    .flat_map(|(x, c)| match c {
                        '.' => ['.', '.'],
                        '#' => ['#', '#'],
                        'O' => ['[', ']'],
                        ch => {
                            if ch == '@' {
                                robot = (x as i64 * 2, y as i64)
                            }
                            ['.', '.']
                        }
                    })
                    .collect()
            })
            .collect();

        let motions = Self::parse_motion(sections.next());

        Self {
            width: grid[0].len() as i64,
            height: grid.len() as i64,
            grid,
            robot,
            motions,
        }
    }

    fn parse_motion(motions: Option<&str>) -> Vec<(i64, i64)> {
        motions
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
            .collect()
    }

    fn get(&self, pos: (i64, i64)) -> char {
        self.grid[pos.1 as usize][pos.0 as usize]
    }

    fn get_mut(&mut self, pos: (i64, i64)) -> &mut char {
        &mut self.grid[pos.1 as usize][pos.0 as usize]
    }

    fn swap(&mut self, pos1: (i64, i64), pos2: (i64, i64)) {
        let temp = self.get(pos1);
        *self.get_mut(pos1) = self.get(pos2);
        *self.get_mut(pos2) = temp;
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
        self.robot = move_(self.robot, motion);
        *self.get_mut(self.robot) = '.';
    }

    fn attempt2(&mut self, motion: (i64, i64)) {
        let next_pos = move_(self.robot, motion);

        match self.get(next_pos) {
            '.' => {
                self.robot = next_pos;
            }
            '#' => {}
            '[' | ']' => {
                if motion.1 == 0 {
                    let mut shifts = vec![next_pos];

                    while let '[' | ']' = self.get(move_(*shifts.last().unwrap(), motion)) {
                        shifts.push(move_(*shifts.last().unwrap(), motion));
                    }
                    shifts.push(move_(*shifts.last().unwrap(), motion));

                    match self.get(*shifts.last().unwrap()) {
                        '#' => {}
                        '.' => {
                            for i in (1..shifts.len()).rev() {
                                self.swap(shifts[i - 1], shifts[i]);
                            }
                            self.swap(self.robot, shifts[0]);
                            self.robot = move_(self.robot, motion);
                        }
                        _ => {}
                    }

                    return;
                }

                let mut shifts = vec![HashSet::from([self.robot])];

                'stack: loop {
                    let mut next_layer = HashSet::new();

                    for &pos in shifts.last().unwrap() {
                        match self.get((pos.0 + motion.0, pos.1 + motion.1)) {
                            '.' => {}
                            '[' | ']' => {
                                next_layer.insert(move_(pos, motion));
                            }
                            '#' => break 'stack,
                            _ => unreachable!(),
                        }
                    }

                    if next_layer.is_empty() {
                        for layer in shifts.iter().rev() {
                            for &pos in layer {
                                self.swap(pos, move_(pos, motion));
                            }
                        }

                        self.robot = next_pos;
                        break;
                    }

                    next_layer.clone().iter().for_each(|&i| {
                        let p = match self.get(i) {
                            '[' => (i.0 + 1, i.1),
                            ']' => (i.0 - 1, i.1),
                            _ => unreachable!(),
                        };
                        next_layer.insert(p);
                    });

                    shifts.push(next_layer);
                }
            }
            _ => unreachable!(),
        }
    }

    fn exec(&mut self, by: char, f: fn(&mut Self, (i64, i64))) -> Option<u64> {
        for motion in self.motions.clone() {
            f(self, motion);
        }

        let mut count = 0;
        for (y, row) in self.grid.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                if cell == by {
                    count += y * 100 + x
                }
            }
        }

        Some(count as u64)
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut warehouse = Warehouse::new(input);
    warehouse.exec('O', Warehouse::attempt)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut warehouse = Warehouse::new2(input);
    warehouse.exec('[', Warehouse::attempt2)
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
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9021));
    }
}
