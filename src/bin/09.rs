advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<usize> {
    let mut lp: usize = 0; // left pointer
    let mut rp: usize = input.len() - 1; // right pointer

    let mut lid: usize = 0; // left id value
    let mut rid: usize = rp / 2; // right id value

    let mut rpop: usize = input[rp..(rp + 1)].parse::<usize>().unwrap(); // left overs count from right end pops
    let mut lhole: usize = 0; // holes on the left
    let mut _rhole: usize = 0; // left over holes on the right

    let mut sum: usize = 0;
    let mut pos: usize = 0;

    let mut le = input[lp..(lp + 1)].parse::<usize>().unwrap();
    let mut re = input[rp..(rp + 1)].parse::<usize>().unwrap();

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
                    re = input[rp..(rp + 1)].parse::<usize>().unwrap();
                }
            } else {
                _rhole += re;
                rp -= 1;
                re = input[rp..(rp + 1)].parse::<usize>().unwrap();
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

        // println!("lp: {}, rp: {}", lp, rp);
        // println!("lid: {}, rid: {}", lid, rid);
        // println!("rpop: {}, lhole: {}, rhole: {}", rpop, lhole, _rhole);
        // println!("sum: {}, pos: {}\n", sum, pos);
    }

    if rpop > 0 {
        for _ in 0..rpop {
            sum += pos * rid;
            pos += 1;
        }
    }

    Some(sum)
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
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
        assert_eq!(result, Some(70));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
