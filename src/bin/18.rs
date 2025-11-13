use std::collections::VecDeque;

advent_of_code::solution!(18);

const DIRECTIONS: [(i64, i64); 4] = [
    (0, 1),  // Down
    (1, 0),  // Right
    (0, -1), // Up
    (-1, 0), // Left
];

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Byte {
    Free,
    Bomb,
    Visited(i64),
}

impl Byte {
    fn value(&self) -> i64 {
        match self {
            Byte::Free => -1,
            Byte::Bomb => -1,
            Byte::Visited(v) => *v,
        }
    }
}

#[allow(dead_code)]
struct Memory {
    bytes: i64,
    size: i64,
    bombs: Vec<(usize, usize)>,
}

#[allow(dead_code)]
impl Memory {
    fn new(input: &str, bytes: i64, size: i64) -> Self {
        let grid = input
            .lines()
            .map(|line| {
                let (x, y) = line.split_once(',').unwrap();
                (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
            })
            .collect::<Vec<(usize, usize)>>();

        Self {
            bombs: grid,
            bytes,
            size,
        }
    }

    fn shortest_path(&self) -> Option<i64> {
        use Byte::*;
        let size = self.size as usize;
        let mut grid: Vec<Vec<Byte>> = vec![vec![Free; size]; size];
        let mut queue: VecDeque<(i64, i64)> = VecDeque::from([(0, 0)]);
        let mut level_count: i64 = 0;

        grid[0][0] = Visited(0);
        for &(x, y) in self.bombs.iter().take(self.bytes as usize) {
            grid[y][x] = Bomb;
        }

        while !queue.is_empty() {
            let mut new_level = VecDeque::new();

            for (x, y) in queue.drain(..) {
                if x == self.size - 1 && y == self.size - 1 {
                    return Some(level_count);
                }

                for &(dx, dy) in &DIRECTIONS {
                    let (nx, ny) = (x + dx, y + dy);
                    if nx >= 0
                        && nx < self.size
                        && ny >= 0
                        && ny < self.size
                        && grid[ny as usize][nx as usize] == Free
                    {
                        grid[ny as usize][nx as usize] = Visited(level_count + 1);
                        new_level.push_back((nx, ny));
                    }
                }
            }

            level_count += 1;
            queue = new_level;
        }

        Some(grid[size - 1][size - 1].value())
    }

    fn first_blocker(&mut self) -> Option<String> {
        loop {
            let path_len = self.shortest_path();
            if path_len.is_none() {
                continue;
            }
            let path_len = path_len.unwrap();

            match path_len {
                -1 => return self.str_from_coord(self.bytes as usize - 1).into(),
                _ => self.bytes += 1,
            }
        }
    }

    #[inline]
    fn str_from_coord(&self, idx: usize) -> String {
        let coord = self.bombs[idx];
        format!("{},{}", coord.0, coord.1)
    }
}

pub fn part_one(input: &str) -> Option<i64> {
    let memory = Memory::new(input, 1024, 71);
    memory.shortest_path()
}

pub fn part_two(input: &str) -> Option<String> {
    let mut memory = Memory::new(input, 1024, 71);
    memory.first_blocker()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("6,1".to_string()));
    }
}
