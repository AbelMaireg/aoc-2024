use std::collections::HashMap;

advent_of_code::solution!(11);

fn solve(input: &str, loops: u64) -> u64 {
    let mut memo: HashMap<(u64, u64), u64> = HashMap::new();

    input
        .split_whitespace()
        .map(|s| blink(s.parse().unwrap(), loops, &mut memo))
        .sum()
}

fn blink(stone: u64, iterations: u64, memo: &mut HashMap<(u64, u64), u64>) -> u64 {
    if iterations == 0 {
        return 1;
    }

    let parameter = (stone, iterations);

    if let Some(&result) = memo.get(&parameter) {
        return result;
    }

    let count = match stone {
        0 => blink(1, iterations - 1, memo),
        s if s.to_string().len().is_multiple_of(2) => {
            let stone_string = stone.to_string();

            let left_half: u64 = stone_string[..stone_string.len() / 2].parse().unwrap();
            let right_half: u64 = stone_string[stone_string.len() / 2..].parse().unwrap();

            blink(left_half, iterations - 1, memo) + blink(right_half, iterations - 1, memo)
        }
        _ => blink(stone * 2024, iterations - 1, memo),
    };

    memo.insert(parameter, count);

    count
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(solve(input, 25))
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(solve(input, 75))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
