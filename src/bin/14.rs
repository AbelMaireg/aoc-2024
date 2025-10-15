use std::{cmp::Ordering, ops::Div};

advent_of_code::solution!(14);

#[derive(Debug)]
struct Robot {
    x: i64,
    y: i64,
    dir: (i64, i64),
}

fn solve(input: &str, width: i64, height: i64, timer: i64, print: bool) -> i64 {
    let robots: Vec<Robot> = input
        .lines()
        .map(|line| {
            let mut parts = line
                .split(&['=', ',', ' '][..])
                .filter_map(|s| s.parse::<i64>().ok());
            Robot {
                x: parts.next().unwrap(),
                y: parts.next().unwrap(),
                dir: (parts.next().unwrap(), parts.next().unwrap()),
            }
        })
        .collect();

    let mut quadrant_counts = [0; 4];
    robots.iter().for_each(|robot| {
        let x = (robot.x + robot.dir.0 * timer).rem_euclid(width);
        let y = (robot.y + robot.dir.1 * timer).rem_euclid(height);

        match (x.cmp(&width.div(2)), y.cmp(&height.div(2))) {
            (Ordering::Less, Ordering::Less) => quadrant_counts[0] += 1,
            (Ordering::Greater, Ordering::Less) => quadrant_counts[1] += 1,
            (Ordering::Less, Ordering::Greater) => quadrant_counts[2] += 1,
            (Ordering::Greater, Ordering::Greater) => quadrant_counts[3] += 1,
            _ => {}
        };
    });

    quadrant_counts.iter().product()
}

fn print_robots(robots: &[Robot], width: i64, height: i64) {
    let mut grid = vec![vec![" "; width as usize]; height as usize];
    robots.iter().for_each(|robot| {
        grid[robot.y as usize][robot.x as usize] = "AA";
    });
    for row in grid {
        println!("{}", row.join(""));
    }
}

pub fn part_one(input: &str) -> Option<i64> {
    Some(solve(input, 11, 7, 100, false))
    // Some(solve(input, 101, 103, 100))
}

pub fn part_two(input: &str) -> Option<u64> {
    solve(input, 101, 103, 100, true);
    Some(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
