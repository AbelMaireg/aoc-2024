advent_of_code::solution!(4);

struct WordSearch {
    grid: Vec<Vec<char>>,
}

#[allow(dead_code)]
impl WordSearch {
    fn new(input: &str) -> Self {
        let grid = input.lines().map(|line| line.chars().collect()).collect();

        Self { grid }
    }

    fn run(&self, word: &str) -> i32 {
        let mut word_count = 0;

        for row in 0..self.grid.len() {
            for col in 0..self.grid[row].len() {
                word_count += self.check_word(word, row as i32, col as i32);
            }
        }

        word_count
    }

    fn check_word(&self, word: &str, row: i32, col: i32) -> i32 {
        let mut word_count = 0;

        for dir in Self::DIRECTION {
            let mut roo = row;
            let mut coo = col;

            'sw: for ch in word.chars() {
                if self.grid[roo as usize][coo as usize] != ch {
                    break 'sw;
                }

                roo += dir[0];
                coo += dir[1];

                if roo == -1
                    || roo == self.grid.len() as i32
                    || coo == -1
                    || coo == self.grid.len() as i32
                {
                    break 'sw;
                }
            }

            if (row - roo).abs() == word.len() as i32 || (col - coo).abs() == word.len() as i32 {
                word_count += 1;
            }
        }

        word_count
    }

    const DIRECTION: [[i32; 2]; 8] = [
        [0, 1],   // right
        [0, -1],  // left
        [-1, 0],  // up
        [1, 0],   // down
        [1, 1],   // down-right
        [-1, -1], // up-left
        [1, -1],  // down-left
        [-1, 1],  // up-right
    ];

    fn run2(&self) -> i32 {
        let mut word_count = 0;

        for row in 1..(self.grid.len() - 1) {
            for col in 1..(self.grid[row].len() - 1) {
                if self.xmas(row, col) {
                    word_count += 1
                };
            }
        }

        word_count
    }

    fn xmas(&self, row: usize, col: usize) -> bool {
        if self.grid[row][col] != 'A' {
            return false;
        }

        let ul = self.grid[row - 1][col - 1];
        let ur = self.grid[row - 1][col + 1];
        let dl = self.grid[row + 1][col - 1];
        let dr = self.grid[row + 1][col + 1];

        ((ul == 'M' && dr == 'S') || (ul == 'S' && dr == 'M'))
            && ((ur == 'M' && dl == 'S') || (ur == 'S' && dl == 'M'))
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let ws = WordSearch::new(input);
    Some(ws.run("XMAS") as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let ws = WordSearch::new(input);
    Some(ws.run2() as u64)
}

#[cfg(test)]
mod day_04 {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(9));
    }
}
