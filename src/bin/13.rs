advent_of_code::solution!(13);

#[derive(Debug)]
struct Equation {
    lhs: [i64; 2],
    rhs: i64,
}

fn solve_eq(eq1: &Equation, eq2: &Equation) -> Option<(i64, i64)> {
    let a1 = eq1.lhs[0];
    let b1 = eq1.lhs[1];
    let c1 = eq1.rhs;

    let a2 = eq2.lhs[0];
    let b2 = eq2.lhs[1];
    let c2 = eq2.rhs;

    let determinant = a1 * b2 - a2 * b1;
    if determinant == 0 {
        return None;
    }

    let x = (c1 * b2 - c2 * b1) / determinant;
    let y = (a1 * c2 - a2 * c1) / determinant;

    if a1 * x + b1 * y != c1 || a2 * x + b2 * y != c2 {
        return None;
    }

    Some((x, y))
}

fn solve(input: &str, offset: i64) -> Option<i64> {
    let sum: i64 = input
        .split("\n\n")
        .map(|machine| {
            let mac: Vec<(i64, i64)> = machine
                .lines()
                .map(|line| {
                    let mut parts = line.split(&[',', '+', '='][..]).rev();
                    let second = parts.next().map(|s| s.parse::<i64>().unwrap());
                    let first = parts.nth(1).map(|s| s.parse::<i64>().unwrap());
                    (first.unwrap(), second.unwrap())
                })
                .collect();

            let equations = [
                Equation {
                    lhs: [mac[0].0, mac[1].0],
                    rhs: mac[2].0 + offset,
                },
                Equation {
                    lhs: [mac[0].1, mac[1].1],
                    rhs: mac[2].1 + offset,
                },
            ];

            solve_eq(&equations[0], &equations[1])
                .map(|(x, y)| 3 * x + y)
                .unwrap_or(0)
        })
        .sum();

    Some(sum)
}

pub fn part_one(input: &str) -> Option<i64> {
    solve(input, 0)
}

pub fn part_two(input: &str) -> Option<i64> {
    solve(input, 10000000000000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(875318608908));
    }
}
