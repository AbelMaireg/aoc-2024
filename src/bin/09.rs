use std::collections::LinkedList;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<usize> {
    let mut lp: usize = 0; // left pointer
    let mut rp: usize = input.len() - 1; // right pointer

    let mut lid: usize = 0; // left id value
    let mut rid: usize = rp / 2; // right id value

    let mut rpop: usize = input[rp..(rp + 1)].parse::<usize>().unwrap(); // left overs count from right end pops
    let mut lhole: usize = 0; // holes on the left

    let mut sum: usize = 0; // checksum
    let mut pos: usize = 0; // position after compression

    let mut le = input[lp..(lp + 1)].parse::<usize>().unwrap();

    while lp < rp {
        if lhole > 0 {
            if rp.is_multiple_of(2) {
                for _ in 0..std::cmp::min(lhole, rpop) {
                    sum += pos * rid;
                    pos += 1;
                    lhole -= 1;
                    rpop -= 1;
                }
                if rpop == 0 {
                    rp -= 1;
                    rid -= 1;
                }
            } else {
                rp -= 1;
                rpop = input[rp..(rp + 1)].parse::<usize>().unwrap();
            }
        } else {
            if lp.is_multiple_of(2) {
                for _ in 0..le {
                    sum += pos * lid;
                    pos += 1;
                }
                lid += 1;
            } else {
                lhole = le
            }
            lp += 1;
            le = input[lp..(lp + 1)].parse::<usize>().unwrap();
        }
    }

    if rpop > 0 {
        for _ in 0..rpop {
            sum += pos * rid;
            pos += 1;
        }
    }

    Some(sum)
}

#[allow(dead_code)]
#[derive(Debug)]
struct Block {
    id: Option<usize>,
    start: usize,
    size: usize,
}

#[allow(dead_code)]
impl Block {
    fn new(start: usize, size: usize, id: Option<usize>) -> Self {
        Block { start, size, id }
    }

    fn eval(&self) -> usize {
        self.id.unwrap() * (((self.start + self.start + self.size - 1) * self.size) / 2)
    }
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut start: usize = 0;
    let mut free_blocks: LinkedList<Block> = LinkedList::new();
    let mut allocated_blocks: LinkedList<Block> = LinkedList::new();
    let mut sum: usize = 0;

    input.chars().enumerate().for_each(|(id, ch)| {
        let space = ch.to_digit(10).unwrap() as usize;
        if id.is_multiple_of(2) {
            allocated_blocks.push_back(Block::new(start, space, Some(id / 2)));
        } else {
            free_blocks.push_back(Block::new(start, space, None));
        }
        start += space;
    });

    while let Some(mut block) = allocated_blocks.pop_back() {
        for first_fit_block in free_blocks.iter_mut() {
            if first_fit_block.size >= block.size && first_fit_block.start < block.start {
                block.start = first_fit_block.start;
                first_fit_block.start += block.size;
                first_fit_block.size -= block.size;
            } else if first_fit_block.start >= block.start {
                break;
            }
        }

        sum += block.eval();
    }

    Some(sum)
}

#[cfg(test)]
mod day_09 {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_one_case_1() {
        let result = part_one("12345");
        assert_eq!(result, Some(60));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }

    #[test]
    fn test_part_two_case_1() {
        let result = part_two("1313165");
        assert_eq!(result, Some(169))
    }

    #[test]
    fn test_part_two_case_2() {
        let result = part_two("9953877292941");
        assert_eq!(result, Some(5768))
    }

    #[test]
    fn test_part_two_case_3() {
        let result = part_two("29702");
        assert_eq!(result, Some(59))
    }
}
