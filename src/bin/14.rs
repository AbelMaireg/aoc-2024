use std::{cmp::Ordering, ops::Div};

advent_of_code::solution!(14);

#[derive(Debug)]
struct Robot {
    x: i64,
    y: i64,
    dir: (i64, i64),
}

fn read_input(input: &str) -> Vec<Robot> {
    input
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
        .collect::<Vec<Robot>>()
}

fn solve(robots: &[Robot], width: i64, height: i64, timer: i64) -> i64 {
    robots
        .iter()
        .fold([0; 4], |mut acc, robot| {
            let x = (robot.x + robot.dir.0 * timer).rem_euclid(width);
            let y = (robot.y + robot.dir.1 * timer).rem_euclid(height);

            match (x.cmp(&width.div(2)), y.cmp(&height.div(2))) {
                (Ordering::Less, Ordering::Less) => acc[0] += 1,
                (Ordering::Greater, Ordering::Less) => acc[1] += 1,
                (Ordering::Less, Ordering::Greater) => acc[2] += 1,
                (Ordering::Greater, Ordering::Greater) => acc[3] += 1,
                _ => {}
            };

            acc
        })
        .iter()
        .product()
}

fn shift_robots(robots: &mut [Robot], width: i64, height: i64) {
    robots.iter_mut().for_each(|robot| {
        robot.x = (robot.x + robot.dir.0).rem_euclid(width);
        robot.y = (robot.y + robot.dir.1).rem_euclid(height);
    });
}

fn variance(robots: &[Robot]) -> i64 {
    let (sum_x, sum_y) = robots
        .iter()
        .fold((0, 0), |(acc_x, acc_y), r| (acc_x + r.x, acc_y + r.y));

    let (mean_x, mean_y) = (sum_x / robots.len() as i64, sum_y / robots.len() as i64);

    let (variance_x, variance_y) = robots.iter().fold((0, 0), |(var_x, var_y), r| {
        (var_x + (r.x - mean_x).pow(2), var_y + (r.y - mean_y).pow(2))
    });

    variance_x + variance_y
}

#[allow(dead_code)]
fn shift_robots_by(robots: &mut [Robot], width: i64, height: i64, times: i64) {
    robots.iter_mut().for_each(|robot| {
        robot.x = (robot.x + robot.dir.0 * times).rem_euclid(width);
        robot.y = (robot.y + robot.dir.1 * times).rem_euclid(height);
    });
}

#[allow(dead_code)]
fn print_robots(robots: &[Robot], width: i64, height: i64) {
    let mut grid = vec![vec!["\x1b[32m█"; width as usize]; height as usize];
    robots.iter().for_each(|robot| {
        grid[robot.y as usize][robot.x as usize] = " ";
    });
    for row in grid {
        for cell in row {
            print!("{}", cell);
        }
        println!();
    }
}

pub fn part_one(input: &str) -> Option<i64> {
    let robots = read_input(input);

    if robots.len() < 20 {
        Some(solve(&robots, 11, 7, 100))
    } else {
        Some(solve(&robots, 101, 103, 100))
    }
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut robots = read_input(input);
    let mut it: i64 = 0;
    let mut sta_dev: i64 = variance(&robots);

    for i in 1..10000 {
        shift_robots(&mut robots, 101, 103);
        let cur_sta_dev = variance(&robots);
        if cur_sta_dev < sta_dev {
            sta_dev = cur_sta_dev;
            it = i;
        }
    }

    let mut origins = read_input(input);
    shift_robots_by(&mut origins, 101, 103, it);
    print_robots(&origins, 101, 103);

    Some(it)
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
        assert_eq!(result, Some(5253));
    }
}
