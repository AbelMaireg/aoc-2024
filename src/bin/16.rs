use std::collections::{BinaryHeap, HashSet};

advent_of_code::solution!(16);

#[allow(dead_code)]
struct Maze {
    tiles: Vec<Vec<char>>,
    start: Position,
    end: Position,
}

#[allow(dead_code)]
impl Maze {
    fn new(input: &str) -> Self {
        let mut tiles = Vec::new();
        let mut start = Position::new(0, 0);
        let mut end = Position::new(0, 0);

        for (y, line) in input.lines().enumerate() {
            let mut row = Vec::new();
            for (x, ch) in line.chars().enumerate() {
                if ch == 'S' {
                    start = Position::new(x, y);
                } else if ch == 'E' {
                    end = Position::new(x, y);
                }
                row.push(ch);
            }
            tiles.push(row);
        }

        Maze { tiles, start, end }
    }

    fn get(&self, pos: Position) -> char {
        self.tiles[pos.y][pos.x]
    }

    fn solve(&mut self) -> Option<usize> {
        use Direction::*;

        let start_tile = self.start;
        let end_tile = self.end;
        let mut min = BinaryHeap::new();
        let mut visited = HashSet::new();

        [Right, Up, Down, Left].iter().for_each(|&dir| {
            min.push(State {
                cost: 0,
                position: start_tile,
                direction: dir,
            })
        });

        while let Some(State {
            cost: cc,
            position: cp,
            direction: cd,
        }) = min.pop()
        {
            if cp == end_tile {
                return Some(cc);
            }

            visited.insert((cp, cd));

            let np = cp.add(cd);
            if self.get(np) != '#' && !visited.contains(&(np, cd)) {
                min.push(State {
                    cost: cc + 1,
                    position: np,
                    direction: cd,
                })
            }

            for nd in cd.turns() {
                if !visited.contains(&(cp, nd)) && self.get(cp.add(nd)) != '#' {
                    min.push(State {
                        cost: cc + 1001,
                        position: cp.add(nd),
                        direction: nd,
                    });
                }
            }
        }

        None
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut maze = Maze::new(input);
    maze.solve()
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
        assert_eq!(result, Some(6036));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(45));
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[allow(dead_code)]
impl Direction {
    fn turns(self) -> [Self; 2] {
        use Direction::*;
        match self {
            Down | Up => [Right, Left],
            Left | Right => [Up, Down],
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

#[allow(dead_code)]
impl Position {
    fn new(x: usize, y: usize) -> Self {
        Position { x, y }
    }

    fn add(&self, dir: Direction) -> Position {
        use Direction::*;
        match dir {
            Up => Position::new(self.x, self.y - 1),
            Down => Position::new(self.x, self.y + 1),
            Left => Position::new(self.x - 1, self.y),
            Right => Position::new(self.x + 1, self.y),
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
struct State {
    cost: usize,
    position: Position,
    direction: Direction,
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.cost.eq(&other.cost)
    }
}

impl Eq for State {}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[allow(dead_code)]
fn print(maze: &Maze, cc: usize, cp: Position, cd: Direction) {
    println!("Cost: {}", cc);
    println!("Position: ({}, {})", cp.x, cp.y);
    println!("Direction: {:?}", cd);
    for y in 0..maze.tiles.len() {
        for x in 0..maze.tiles[0].len() {
            match maze.get(Position::new(x, y)) {
                '#' => print!("#"),
                '.' => {
                    if cp.x == x && cp.y == y {
                        print!("X");
                    } else {
                        print!(".");
                    }
                }
                'S' => print!("S"),
                'E' => print!("E"),
                _ => print!(" "),
            }
        }
        println!()
    }
    println!()
}
