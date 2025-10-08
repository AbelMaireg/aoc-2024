use std::collections::BinaryHeap;

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
#[derive(Debug, Clone, Eq, PartialEq)]
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

impl PartialOrd for Block {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Block {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.start.cmp(&self.start)
    }
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut start: usize = 0; // start position
    let mut free_bc: Vec<BinaryHeap<Block>> = Vec::new(); // free block clusters
    let mut alloc_b: Vec<Block> = Vec::new(); // allocated blocks
    let mut sum: usize = 0; // checksum

    for _ in 0..10 {
        free_bc.push(BinaryHeap::new()); // initialize free blocks cluster
    }

    input.chars().enumerate().for_each(|(id, ch)| {
        let size = ch.to_digit(10).unwrap() as usize; // block size (1-9) parsed
        if id.is_multiple_of(2) {
            alloc_b.push(Block::new(start, size, Some(id / 2)));
        } else {
            free_bc[size].push(Block::new(start, size, None));
        }
        start += size;
    });

    for chunk in alloc_b.iter_mut().rev() {
        let mut x_size: Option<usize> = None; // free block size

        for (size, heap) in free_bc.iter().enumerate().skip(chunk.size) {
            if let Some(head) = heap.peek() {
                if (x_size.is_none() || head.start < free_bc[x_size.unwrap()].peek().unwrap().start)
                    && head.start < chunk.start
                {
                    x_size = Some(size);
                }
            }
        }

        if let Some(size) = x_size {
            let mut free_block = free_bc[size].pop().unwrap(); // get the best fit free block

            chunk.start = free_block.start;
            free_block.size -= chunk.size;
            free_block.start += chunk.size;

            if free_block.size > 0 {
                free_bc[free_block.size].push(free_block);
            }
        }

        sum += chunk.eval();
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
