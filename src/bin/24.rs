use std::collections::{HashMap, HashSet};

advent_of_code::solution!(24);

type Map<'a> = HashMap<&'a str, (&'a str, &'a str, &'a str)>;
type KnownMap<'a> = HashMap<&'a str, bool>;

fn process<'a>(
    map: &Map<'a>,
    knowns: &mut KnownMap<'a>,
    left: &'a str,
    op: &str,
    right: &'a str,
    to: &'a str,
) -> bool {
    let left_val = knowns.get(left);
    if left_val.is_none() {
        let (l, r, o) = map.get(left).unwrap();
        process(map, knowns, l, r, o, left);
    }
    let right_val = knowns.get(right);
    if right_val.is_none() {
        let (l, r, o) = map.get(right).unwrap();
        process(map, knowns, l, r, o, right);
    }

    let left_val = knowns.get(left).unwrap();
    let right_val = knowns.get(right).unwrap();

    let val = match op {
        "AND" => *left_val && *right_val,
        "OR" => *left_val || *right_val,
        "XOR" => *left_val ^ *right_val,
        _ => unreachable!(),
    };

    knowns.insert(to, val);
    val
}

pub fn part_one(input: &str) -> Option<u64> {
    let (one, two) = input.split_once("\n\n").unwrap();

    let mut knowns: KnownMap = one
        .lines()
        .map(|line| {
            let mut parts = line.split(": ");
            let key = parts.next().unwrap();
            let value = parts.next().unwrap() == "1";
            (key, value)
        })
        .collect();

    let map: Map = two
        .lines()
        .map(|line| {
            let parts = line.split_whitespace().collect::<Vec<_>>();
            (parts[4], (parts[0], parts[1], parts[2]))
        })
        .collect();

    let mut processed = map
        .iter()
        .filter(|&(k, _)| k.starts_with("z"))
        .map(|(to, (left, op, right))| {
            (
                to.to_string(),
                process(&map, &mut knowns, left, op, right, to),
            )
        })
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
    let (_, sec) = input.split_once("\n\n").unwrap();
    let gates = sec
        .lines()
        .map(|line| {
            let parts = line.split_whitespace().collect::<Vec<_>>();
            (parts[0], parts[1], parts[2], parts[4])
        })
        .collect::<Vec<_>>();

    let mut output: HashSet<(&str, &str)> = HashSet::new();
    let mut swapped: HashSet<&str> = HashSet::new();

    // Track the operation of gate that each wire label outputs to.
    for &(left, op, right, _) in gates.iter() {
        output.insert((left, op));
        output.insert((right, op));
    }

    for &(left, op, right, to) in gates.iter() {
        match op {
            "AND" => {
                // Check that all AND gates point to an OR, except for first AND.
                if left != "x00" && right != "x00" && !output.contains(&(to, "OR")) {
                    swapped.insert(to);
                }
            }
            "OR" => {
                // Check that only XOR gates point to output, except for last carry which is OR.
                if to.starts_with('z') && to != "z45" {
                    swapped.insert(to);
                }
                // OR can never point to OR.
                if output.contains(&(to, "OR")) {
                    swapped.insert(to);
                }
            }
            "XOR" => {
                if left.starts_with('x') || right.starts_with('x') {
                    // Check that first level XOR points to second level XOR, except for first XOR.
                    if left != "x00" && right != "x00" && !output.contains(&(to, "XOR")) {
                        swapped.insert(to);
                    }
                } else {
                    // Second level XOR must point to output.
                    if !to.starts_with('z') {
                        swapped.insert(to);
                    }
                }
            }
            _ => unreachable!(),
        }
    }

    let mut result: Vec<_> = swapped.into_iter().collect();
    result.sort();
    result.join(",").into()
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
        assert_eq!(result, Some("bfw,bqk,ffh,frj,fst,hwm,kpj,kwq,mjb,nrd,rvg,tgd,tnw,vdt,wpb,z02,z03,z05,z06,z07,z08,z10,z11".to_string()));
    }
}
