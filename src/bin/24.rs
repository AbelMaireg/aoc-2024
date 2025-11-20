use std::collections::HashMap;

advent_of_code::solution!(24);

type Map = HashMap<String, (String, String, String)>;
type KnownMap = HashMap<String, bool>;

fn process(map: &Map, knowns: &mut KnownMap, lhs1: &str, lhs2: &str, op: &str, rhs: &str) -> bool {
    let lhs1_val = knowns.get(lhs1);
    if lhs1_val.is_none() {
        let (lhs_lhs, lhs_rhs, lhs_op) = map.get(lhs1).unwrap();
        process(map, knowns, lhs_lhs, lhs_rhs, lhs_op, lhs1);
    }
    let lhs2_val = knowns.get(lhs2);
    if lhs2_val.is_none() {
        let (rhs_lhs, rhs_rhs, rhs_op) = map.get(lhs2).unwrap();
        process(map, knowns, rhs_lhs, rhs_rhs, rhs_op, lhs2);
    }

    let lhs_val = knowns.get(lhs1).unwrap();
    let rhs_val = knowns.get(lhs2).unwrap();

    let val = match op {
        "AND" => *lhs_val && *rhs_val,
        "OR" => *lhs_val || *rhs_val,
        "XOR" => *lhs_val ^ *rhs_val,
        _ => unreachable!(),
    };

    knowns.insert(rhs.to_string(), val);
    val
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut sec_it = input.split("\n\n");

    let mut knowns: HashMap<String, bool> = sec_it
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut parts = line.split(": ");
            let key = parts.next().unwrap().to_string();
            let value = parts.next().unwrap() == "1";
            (key, value)
        })
        .collect();

    let map: Map = sec_it
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();

            let lhs1 = parts.next().unwrap().to_string();
            let op = parts.next().unwrap().to_string();
            let lhs2 = parts.next().unwrap().to_string();
            parts.next();
            let rhs = parts.next().unwrap().to_string();

            (rhs, (lhs1, lhs2, op))
        })
        .collect::<HashMap<_, _>>();

    let mut processed = map
        .iter()
        .filter(|&(k, _)| k.starts_with("z"))
        .map(|(k, (lhs1, lhs2, op))| (k.to_string(), process(&map, &mut knowns, lhs1, lhs2, op, k)))
        .collect::<Vec<_>>();

    processed.sort_by(|a, b| a.0.cmp(&b.0));

    let joined = processed
        .iter()
        .rev()
        .map(|(_, v)| (if *v { "1" } else { "0" }).to_string())
        .collect::<String>();

    u64::from_str_radix(&joined, 2).ok()
}

pub fn part_two(input: &str) -> Option<String> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2024));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
