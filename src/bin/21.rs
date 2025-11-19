use std::{
    collections::{HashMap, HashSet},
    iter::repeat_n,
    sync::LazyLock,
};

advent_of_code::solution!(21);

#[allow(dead_code)]
type Pad = Vec<Vec<char>>;

#[allow(dead_code)]
const OFFSETS: &[(&str, (i32, i32))] =
    &[("<", (0, -1)), (">", (0, 1)), ("^", (-1, 0)), ("v", (1, 0))];

static DIRPAD: LazyLock<Pad> = LazyLock::new(|| vec![vec!['.', '^', 'A'], vec!['<', 'v', '>']]);
static NUMPAD: LazyLock<Pad> = LazyLock::new(|| {
    vec![
        vec!['7', '8', '9'],
        vec!['4', '5', '6'],
        vec!['1', '2', '3'],
        vec!['.', '0', 'A'],
    ]
});

/// Solution struct to hold input data and caches.
/// - data: Vec<String> - input lines
/// - bpc - cache for button paths
/// - mslc - cache for min sequence lengths
/// - pc - cache for permutations
/// - fpc - cache for find position
#[derive(Debug, Default)]
struct Solution {
    data: Vec<Vec<char>>,
    bpc: HashMap<(char, char), Vec<Vec<char>>>,
    mslc: HashMap<(char, Vec<char>, usize), usize>,
    pc: HashMap<Vec<char>, HashSet<Vec<char>>>,
    fpc: HashMap<(char, char), (usize, usize)>,
}

impl Solution {
    fn new(input: &str) -> Self {
        Self {
            data: input.lines().map(|s| s.chars().collect()).collect(),
            ..Default::default()
        }
    }

    /// Find the (row, col) position of a given character in the pad.
    #[allow(dead_code)]
    fn find_pos(&mut self, pad: &Pad, target: char) -> Option<(usize, usize)> {
        if let Some(cached) = self.fpc.get(&(pad[0][0], target)) {
            return Some(*cached);
        }

        for (r, row) in pad.iter().enumerate() {
            for (c, &ch) in row.iter().enumerate() {
                if ch == target {
                    self.fpc.insert((pad[0][0], target), (r, c));
                    return Some((r, c));
                }
            }
        }
        None
    }

    /// Generate all permutations of a sequence of chars.
    /// Example: ['a','b','b'] → {"abb", "bab", "bba"}
    #[allow(dead_code)]
    fn permutations(&mut self, chars: &[char]) -> HashSet<Vec<char>> {
        if let Some(cached) = self.pc.get(chars) {
            return cached.clone();
        }
        fn helper(chars: &mut Vec<char>, i: usize, acc: &mut HashSet<Vec<char>>) {
            if i == chars.len() {
                acc.insert(chars.clone());
                return;
            }
            for j in i..chars.len() {
                chars.swap(i, j);
                helper(chars, i + 1, acc);
                chars.swap(i, j);
            }
        }

        let mut acc = HashSet::new();
        let mut chars = chars.to_vec();
        helper(&mut chars, 0, &mut acc);
        self.pc.insert(chars, acc.clone());
        acc
    }

    /// Get all shortest movement paths between two buttons.
    /// Example result: ["v>>", ">v>", ">>v"]
    #[allow(dead_code)]
    fn get_button_paths(&mut self, pad: &Pad, b1: char, b2: char) -> Vec<Vec<char>> {
        if let Some(cached) = self.bpc.get(&(b1, b2)) {
            return cached.clone();
        }

        let (r1, c1) = self.find_pos(pad, b1).expect("button1 not found");
        let (r2, c2) = self.find_pos(pad, b2).expect("button2 not found");

        // Movement differences
        let dr = r2 as i8 - r1 as i8;
        let dc = c2 as i8 - c1 as i8;

        // Build list of moves: '<', '>', '^', 'v'
        let mut moves = Vec::new();
        if dc < 0 {
            moves.extend(repeat_n('<', (-dc) as usize));
        } else {
            moves.extend(repeat_n('>', dc as usize));
        }
        if dr < 0 {
            moves.extend(repeat_n('^', (-dr) as usize));
        } else {
            moves.extend(repeat_n('v', dr as usize));
        }

        // All unique permutations of movement sequence
        let perms = self.permutations(&moves);

        let mut paths = Vec::new();

        // Offsets for each movement
        let offsets = |m: char| -> (i32, i32) {
            match m {
                '<' => (0, -1),
                '>' => (0, 1),
                '^' => (-1, 0),
                'v' => (1, 0),
                _ => unreachable!(),
            }
        };

        // Validate each sequence by simulating movement
        'outer: for mut seq in perms {
            let mut r = r1 as i32;
            let mut c = c1 as i32;

            for &step in &seq {
                let (dr, dc) = offsets(step);
                r += dr;
                c += dc;

                // If we step into a gap '.' → reject this path
                let row = pad.get(r as usize);
                if row.is_none() {
                    continue 'outer; // outside keypad
                }
                let row = row.unwrap();

                let ch = row.get(c as usize);
                if ch.is_none() || *ch.unwrap() == '.' {
                    continue 'outer; // invalid
                }
            }

            // Valid path → append "A"
            seq.push('A');
            paths.push(seq);
        }

        self.bpc.insert((b1, b2), paths.clone());

        paths
    }

    fn get_min_sequence_length(&mut self, pad: &Pad, code: &[char], depth: usize) -> usize {
        if let Some(cached) = self.mslc.get(&(pad[0][0], code.to_vec(), depth)) {
            return *cached;
        }
        let mut result = 0;

        // Build "A" + code, then iterate over pairs
        let mut prev = 'A';

        for &ch in code.iter() {
            let paths = self.get_button_paths(pad, prev, ch);

            if depth == 0 {
                // We are controlling NUMPAD directly → just take shortest path length
                let shortest = paths.iter().map(|p| p.len()).min().unwrap();
                result += shortest;
            } else {
                // We are controlling a directional keypad → recursively compute
                // the cost to execute each path on the next robot in the chain.
                let mut best = usize::MAX;

                for path in &paths {
                    let cost = self.get_min_sequence_length(&DIRPAD, path, depth - 1);
                    best = std::cmp::min(best, cost);
                }

                result += best;
            }

            prev = ch;
        }

        self.mslc.insert((pad[0][0], code.to_vec(), depth), result);

        result
    }

    fn solve_all(&mut self, depth: usize) -> usize {
        let mut total = 0;

        for code in self.data.clone() {
            // let num = code[..3].parse::<usize>().unwrap();
            let num = code[0..3]
                .iter()
                .collect::<String>()
                .parse::<usize>()
                .unwrap();
            let len = self.get_min_sequence_length(&NUMPAD, &code, depth);
            total += num * len;
        }

        total
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut solution = Solution::new(input);
    Some(solution.solve_all(2))
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut solution = Solution::new(input);
    Some(solution.solve_all(25))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(126384));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(154115708116294));
    }

    #[test]
    fn test_find_pos() {
        let mut solution = Solution::default();
        assert_eq!(solution.find_pos(&NUMPAD, '5'), Some((1, 1)));
        assert_eq!(solution.find_pos(&NUMPAD, '8'), Some((0, 1)));
        assert_eq!(solution.find_pos(&NUMPAD, 'A'), Some((3, 2)));
    }

    #[test]
    fn test_permutations() {
        let mut solution = Solution::default();
        let perms = solution.permutations(&['a', 'b', 'b']);
        let expected: HashSet<Vec<char>> = ["abb", "bab", "bba"]
            .iter()
            .map(|s| s.chars().collect())
            .collect();
        assert_eq!(perms, expected);
    }
}
