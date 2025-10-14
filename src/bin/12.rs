advent_of_code::solution!(12);

#[allow(dead_code)]
const DIRECTION: [(i64, i64); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct Plot {
    ch: char,
    id: Option<i64>,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Garden {
    x: i64,
    y: i64,
    plots: Vec<Vec<Plot>>,
}

#[allow(dead_code)]
impl Garden {
    fn new(input: &str) -> Self {
        let plots: Vec<Vec<Plot>> = input
            .lines()
            .map(|line| line.chars().map(|ch| Plot { ch, id: None }).collect())
            .collect();

        Self {
            x: plots[0].len() as i64,
            y: plots.len() as i64,
            plots,
        }
    }

    fn at(&self, x: i64, y: i64) -> Option<&Plot> {
        if x < 0 || x >= self.x || y < 0 || y >= self.y {
            return None;
        }
        Some(&self.plots[y as usize][x as usize])
    }

    fn solve(&mut self) -> i64 {
        let mut sum = 0;
        let mut region_id = 0;

        for y in 0..self.y {
            for x in 0..self.x {
                let plot = self.at(x, y);

                if plot.is_none() || plot.unwrap().id.is_some() {
                    continue;
                }

                let mut queue: Vec<(i64, i64)> = vec![(x, y)];
                let mut area = 0;
                let mut perimeter = 0;
                region_id += 1;

                while let Some((a, b)) = queue.pop() {
                    let plot = self.at(a, b);

                    if plot.is_none() || plot.unwrap().id.is_some() {
                        continue;
                    }

                    let ch = plot.unwrap().ch;
                    self.plots[b as usize][a as usize].id = Some(region_id);
                    area += 1;

                    for (dx, dy) in DIRECTION.iter() {
                        let nx = a + dx;
                        let ny = b + dy;

                        if let Some(neighbor) = self.at(nx, ny) {
                            if neighbor.ch == ch {
                                queue.push((nx, ny));
                            } else {
                                perimeter += 1;
                            }
                        } else {
                            perimeter += 1;
                        }
                    }
                }

                sum += area * perimeter;
            }
        }

        sum
    }

    fn solve2(&mut self) -> i64 {
        let mut sum = 0;
        let mut region_id = 0;

        for y in 0..self.y {
            for x in 0..self.x {
                let plot = self.at(x, y);

                if plot.is_none() || plot.unwrap().id.is_some() {
                    continue;
                }

                let mut queue: Vec<(i64, i64)> = vec![(x, y)];
                let mut sides: Vec<Vec<(i64, i64)>> = vec![vec![]; 4];
                let mut area = 0;
                region_id += 1;

                while let Some((a, b)) = queue.pop() {
                    let plot = self.at(a, b);

                    if plot.is_none() || plot.unwrap().id.is_some() {
                        continue;
                    }

                    let ch = plot.unwrap().ch;
                    self.plots[b as usize][a as usize].id = Some(region_id);
                    area += 1;

                    for (dir, (dx, dy)) in DIRECTION.iter().enumerate() {
                        let nx = a + dx;
                        let ny = b + dy;

                        if let Some(neighbor) = self.at(nx, ny) {
                            if neighbor.ch == ch {
                                queue.push((nx, ny));
                                continue;
                            }
                        }
                        if dir < 2 {
                            sides[dir].push((a, b));
                        } else {
                            sides[dir].push((b, a));
                        }
                    }
                }

                for side in sides.iter_mut() {
                    side.sort_by(|a, b| {
                        if a.1 == b.1 {
                            a.0.cmp(&b.0)
                        } else {
                            a.1.cmp(&b.1)
                        }
                    });
                }

                let mut sides_c = 0;

                for side in sides.iter() {
                    for i in 0..side.len() {
                        if i == 0 || side[i].0 != side[i - 1].0 + 1 || side[i].1 != side[i - 1].1 {
                            sides_c += 1;
                        }
                    }
                }

                sum += area * sides_c;
            }
        }

        sum
    }
}

pub fn part_one(_input: &str) -> Option<i64> {
    let mut garden = Garden::new(_input);
    Some(garden.solve())
}

pub fn part_two(_input: &str) -> Option<i64> {
    let mut garden = Garden::new(_input);
    Some(garden.solve2())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_one_case_1() {
        let result = part_one("AAAA\nBBCD\nBBCC\nEEEC");
        assert_eq!(result, Some(140));
    }

    #[test]
    fn test_part_one_case_2() {
        let result = part_one("OOOOO\nOXOXO\nOOOOO\nOXOXO\nOOOOO");
        assert_eq!(result, Some(772));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }

    #[test]
    fn test_part_two_case_1() {
        let result = part_two("AAAA\nBBCD\nBBCC\nEEEC");
        assert_eq!(result, Some(80));
    }

    #[test]
    fn test_part_two_case_2() {
        let result = part_two("EEEEE\nEXXXX\nEEEEE\nEXXXX\nEEEEE");
        assert_eq!(result, Some(236));
    }
}
