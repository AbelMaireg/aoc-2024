use std::collections::{HashMap, HashSet};

advent_of_code::solution!(20);

type Pos = (i64, i64);
type Grid = Vec<Vec<char>>;
const OFFSETS: [(i64, i64); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn parse_input(lines: &[String]) -> (Grid, Pos, Pos) {
    let mut grid: Grid = vec![];
    let mut start_pos = (0, 0);
    let mut end_pos = (0, 0);
    for (y, line) in lines.iter().enumerate() {
        grid.push(vec![]);
        for (x, c) in line.chars().enumerate() {
            let pos = (x as i64, y as i64);
            match c {
                'S' => start_pos = pos,
                'E' => end_pos = pos,
                _ => {}
            }
            grid[y].push(c);
        }
    }
    let default_grid = grid.clone();
    (default_grid, start_pos, end_pos)
}

fn find_path(grid: &Grid, start: Pos, end: Pos) -> HashMap<Pos, i64> {
    let mut path: HashMap<Pos, i64> = HashMap::new();
    let mut pos = start;
    let mut visited = HashSet::new();
    path.insert(start, 0);
    visited.insert(start);
    while pos != end {
        let mut found = false;
        for &offset in &OFFSETS {
            let next = ((pos.0 + offset.0), pos.1 + offset.1);
            if next.0 < 0
                || next.1 < 0
                || next.1 as usize >= grid.len()
                || next.0 as usize >= grid[0].len()
                || grid[next.1 as usize][next.0 as usize] == '#'
                || visited.contains(&next)
            {
                continue;
            }
            pos = next;
            visited.insert(pos);
            path.insert(pos, path.len() as i64);
            found = true;
            break;
        }
        if !found {
            panic!("No path found");
        }
    }
    path
}

fn count_good_cheats(grid: &Grid, start: Pos, end: Pos, cheat_length: i64) -> i64 {
    let path = find_path(grid, start, end);
    let mut good_cheats = 0;
    for (&pos, &move_num) in &path {
        let r = cheat_length;
        for dx in -r..=r {
            let dy_limit = r - dx.abs();
            for dy in -dy_limit..=dy_limit {
                let cheat_pos = (pos.0 + dx, pos.1 + dy);
                if let Some(&c) = grid
                    .get(cheat_pos.1 as usize)
                    .and_then(|row| row.get(cheat_pos.0 as usize))
                {
                    if c == '#' {
                        continue;
                    }
                } else {
                    continue; // Outside grid
                }
                if let Some(&cheat_move) = path.get(&cheat_pos) {
                    let dist = dx.abs() + dy.abs();
                    if cheat_move > move_num + dist && (cheat_move - (move_num + dist)) >= 100 {
                        good_cheats += 1;
                    }
                }
            }
        }
    }
    good_cheats
}

pub fn part_one(_input: &str) -> Option<i64> {
    let lines: Vec<String> = _input.lines().map(|line| line.to_string()).collect();
    let (grid, start_pos, end_pos) = parse_input(&lines);
    let result = count_good_cheats(&grid, start_pos, end_pos, 2);
    Some(result)
}

pub fn part_two(_input: &str) -> Option<i64> {
    let lines: Vec<String> = _input.lines().map(|line| line.to_string()).collect();
    let (grid, start_pos, end_pos) = parse_input(&lines);
    let result = count_good_cheats(&grid, start_pos, end_pos, 20);
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(0));
    }
}
