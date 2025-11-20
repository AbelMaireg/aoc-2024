use std::collections::{HashMap, HashSet};

advent_of_code::solution!(23);

struct Graph {
    nodes: HashMap<String, HashSet<String>>,
}

impl Graph {
    fn new(input: &str) -> Self {
        let mut graph: HashMap<String, HashSet<String>> = HashMap::new();
        input.lines().for_each(|line| {
            let (left, right) = line.split_once('-').unwrap();
            graph
                .entry(left.to_string())
                .or_default()
                .insert(right.to_string());
            graph
                .entry(right.to_string())
                .or_default()
                .insert(left.to_string());
        });
        Graph { nodes: graph }
    }

    fn max_cliques(
        &self,
        r: HashSet<String>,
        mut p: HashSet<String>,
        mut x: HashSet<String>,
    ) -> HashSet<String> {
        if p.is_empty() && x.is_empty() {
            return r;
        }

        let mut max_clique = HashSet::new();
        let pivot = p.union(&x).next().unwrap().clone();
        let neighbors = self.nodes.get(&pivot).unwrap();

        let candidates: HashSet<String> = p.difference(neighbors).cloned().collect();

        for v in candidates {
            let mut new_r = r.clone();
            new_r.insert(v.clone());

            let new_p: HashSet<String> = p
                .intersection(self.nodes.get(&v).unwrap())
                .cloned()
                .collect();

            let new_x: HashSet<String> = x
                .intersection(self.nodes.get(&v).unwrap())
                .cloned()
                .collect();

            let clique = self.max_cliques(new_r, new_p, new_x);

            if clique.len() > max_clique.len() {
                max_clique = clique;
            }

            p.remove(&v);
            x.insert(v);
        }

        max_clique
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let puzzle = Graph::new(input);
    let mut trios: HashSet<[String; 3]> = HashSet::new();

    for (key, val) in puzzle.nodes.iter() {
        for val_as_key in val.iter() {
            let neighbors = puzzle.nodes.get(val_as_key).unwrap();
            for node in neighbors.intersection(val) {
                let mut trio = [key.clone(), val_as_key.clone(), node.clone()];
                trio.sort();
                trios.insert(trio);
            }
        }
    }

    let count = trios
        .iter()
        .filter(|&[a, b, c]| a.starts_with('t') || b.starts_with('t') || c.starts_with('t'))
        .count();

    Some(count as u64)
}

pub fn part_two(input: &str) -> Option<String> {
    let puzzle = Graph::new(input);
    let keys: HashSet<String> = puzzle.nodes.keys().cloned().collect();

    let mut max = puzzle
        .max_cliques(HashSet::new(), keys, HashSet::new())
        .iter()
        .map(std::string::ToString::to_string)
        .collect::<Vec<String>>();
    max.sort();

    Some(max.join(","))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("co,de,ka,ta".to_string()));
    }
}
