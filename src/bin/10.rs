use std::collections::HashSet;

advent_of_code::solution!(10);

#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
struct Node {
    val: i64,
    cur: (i64, i64),
    start: (i64, i64),
}

#[allow(dead_code)]
impl Node {
    fn new(val: i64, start_node: (i64, i64)) -> Self {
        Self {
            val,
            cur: start_node,
            start: start_node,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Default)]
struct Grid {
    width: i64,
    height: i64,
    nodes: Vec<Vec<i64>>,
    zeros: Vec<Node>,
}

#[allow(dead_code)]
impl Grid {
    fn new(input: &str) -> Self {
        let nodes: Vec<Vec<i64>> = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as i64)
                    .collect()
            })
            .collect();

        let height = nodes.len() as i64;
        let width = nodes[0].len() as i64;

        let mut zeros = Vec::new();
        for (y, row) in nodes.iter().enumerate() {
            for (x, &val) in row.iter().enumerate() {
                if val == 0 {
                    zeros.push(Node::new(val, (x as i64, y as i64)));
                }
            }
        }

        Self {
            width,
            height,
            nodes,
            zeros,
        }
    }

    fn solve(&mut self) -> i64 {
        let mut scored_nodes: HashSet<(i64, i64, i64, i64)> = HashSet::new();

        while let Some(node) = self.zeros.pop() {
            if node.val == 9 {
                scored_nodes.insert((node.start.0, node.start.1, node.cur.0, node.cur.1));
                continue;
            }

            for dir in Self::DIRECTIONS {
                let mut next = node.clone();
                next.cur.0 += dir.0;
                next.cur.1 += dir.1;

                if next.cur.0 < 0
                    || next.cur.1 < 0
                    || next.cur.0 >= self.width
                    || next.cur.1 >= self.height
                {
                    continue;
                }

                next.val = self.nodes[next.cur.1 as usize][next.cur.0 as usize];

                if next.val - node.val == 1 {
                    self.zeros.push(next)
                }
            }
        }

        scored_nodes.len() as i64
    }

    const DIRECTIONS: [(i64, i64); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
}

pub fn part_one(_input: &str) -> Option<i64> {
    let mut grid = Grid::new(_input);
    Some(grid.solve())
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
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
