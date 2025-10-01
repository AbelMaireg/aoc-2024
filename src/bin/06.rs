use std::collections::HashSet;

advent_of_code::solution!(6);

#[allow(dead_code)]
struct Map {
    width: i32,
    height: i32,
    data: Vec<Vec<char>>,
    start_pos: (i32, i32),
}

#[allow(dead_code)]
impl Map {
    fn new(input: &str) -> Self {
        let mut start_pos = (0, 0);

        let data =
            input
                .lines()
                .enumerate()
                .fold(Vec::<Vec<char>>::new(), |mut acc, (idx, line)| {
                    let mut row = vec![];

                    line.chars().enumerate().for_each(|(jdx, mut c)| {
                        if c == '^' {
                            start_pos = (idx as i32, jdx as i32);
                            c = '.';
                        }
                        row.push(c)
                    });
                    acc.push(row);
                    acc
                });

        Map {
            width: data[0].len() as i32,
            height: data.len() as i32,
            data,
            start_pos,
        }
    }

    fn at(&self, pos: Position) -> Option<char> {
        if pos.y < 0 || pos.y >= self.height || pos.x < 0 || pos.x >= self.width {
            None
        } else {
            Some(self.data[pos.y as usize][pos.x as usize])
        }
    }

    fn run(&self) -> i32 {
        let mut pos = Position::new(self.start_pos.1, self.start_pos.0);
        let mut dir = Direction::new((0, -1));
        let mut map = self.data.clone();

        map[pos.y as usize][pos.x as usize] = 'X';

        loop {
            match self.at(pos.ahead(&dir)) {
                Some('#') => {
                    dir.turn_right();
                    continue;
                }
                None => break,
                _ => {}
            }

            pos.move_to(&dir);
            map[pos.y as usize][pos.x as usize] = 'X';
        }

        map.iter().flatten().fold(0, |acc, cur| {
            if *cur == 'X' {
                return acc + 1;
            }

            acc
        })
    }

    fn path(&self) -> HashSet<Position> {
        let mut pos = Position::new(self.start_pos.1, self.start_pos.0);
        let mut dir = Direction::default();
        let mut visited = HashSet::new();

        while self.at(pos).is_some() {
            visited.insert(pos);

            let next = pos.ahead(&dir);
            match self.at(next) {
                Some('#') => {
                    dir.turn_right();
                }
                None => break,
                _ => {
                    pos = next;
                }
            }
        }

        visited
    }

    fn traverse_with_cycle(&self) -> bool {
        let mut pos = Position::new(self.start_pos.1, self.start_pos.0);
        let mut dir = Direction::default();
        let mut seen = HashSet::new();

        while self.at(pos).is_some() {
            if !seen.insert((pos, dir)) {
                return true;
            }

            let next = pos.ahead(&dir);
            match self.at(next) {
                Some('#') => dir.turn_right(),
                None => return false,
                _ => pos = next,
            }
        }

        false
    }

    fn run2(&mut self) -> i32 {
        let path = self.path();
        let mut count = 0;

        for candidate in path {
            if candidate == Position::new(self.start_pos.1, self.start_pos.0) {
                continue; // don’t block start
            }

            self.data[candidate.y as usize][candidate.x as usize] = '#';

            if self.traverse_with_cycle() {
                count += 1;
            }

            self.data[candidate.y as usize][candidate.x as usize] = '.';
        }

        count
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Direction {
    delta: (i32, i32),
}

impl Default for Direction {
    fn default() -> Self {
        Direction { delta: (0, -1) }
    }
}

#[allow(dead_code)]
impl Direction {
    fn new(delta: (i32, i32)) -> Self {
        Direction { delta }
    }

    fn turn_right(&mut self) {
        let (dx, dy) = self.delta;
        self.delta = (-dy, dx);
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

#[allow(dead_code)]
impl Position {
    fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }

    fn get(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    fn set(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    fn move_to(&mut self, direction: &Direction) {
        self.x += direction.delta.0;
        self.y += direction.delta.1;
    }

    fn ahead(&self, direction: &Direction) -> Position {
        Position {
            x: self.x + direction.delta.0,
            y: self.y + direction.delta.1,
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let map = Map::new(input);
    Some(map.run() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut map = Map::new(input);
    Some(map.run2() as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
