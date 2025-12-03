// https://adventofcode.com/2025/day/2

pub fn solution_part1(input: &str) -> usize {
    let ranges = Range::from_input(input);
    let mut total_invalid = 0;
    for range in ranges {
        let mut i = range.start;
        while i <= range.end {
            let v_str = i.to_string();
            if v_str.len() % 2 == 0 && v_str[0..v_str.len() / 2] == v_str[v_str.len() / 2..] {
                total_invalid += i;
                i += v_str.len() / 2 * 10;
            } else {
                i += 1;
            }
        }
    }

    total_invalid
}

pub fn solution_part2(input: &str) -> usize {
    let ranges = Range::from_input(input);
    let mut total_invalid = 0;
    for range in ranges {
        let mut i = range.start;
        while i <= range.end {
            let digits = digits(i);
            let mut digits_to_compare = digits / 2;
            while digits_to_compare >= 1 {
                if chunk_compare(i, digits, digits_to_compare) {
                    total_invalid += i;
                    i += digits_to_compare * 10 - 1;
                    break;
                }

                digits_to_compare -= 1;
            }
            i += 1;
        }
    }

    total_invalid
}

fn digits(i: usize) -> usize {
    i.checked_ilog10().unwrap_or(0) as usize + 1
}

fn chunk_compare(mut n: usize, digits: usize, digits_to_compare: usize) -> bool {
    if digits % digits_to_compare != 0 {
        return false;
    }

    let mask = 10usize.pow(digits_to_compare as u32);
    let v = n % mask;
    n /= mask;

    while n != 0 {
        let new = n % mask;
        n /= mask;
        if v != new {
            return false;
        }
    }

    true
}

struct Range {
    start: usize,
    end: usize,
}

impl Range {
    fn from_input(input: &str) -> Vec<Self> {
        input
            .split(',')
            .map(|r| {
                let (left, right) = r.trim().split_once('-').unwrap();
                let left2 = left.parse::<usize>().unwrap();
                let right = right.parse::<usize>().unwrap();
                Self {
                    start: left2,
                    end: right,
                }
            })
            .collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_part1_example() {
        assert_eq!(solution_part1(EXAMPLE), 1227775554);
    }

    #[test]
    fn test_part1() {
        assert_eq!(solution_part1(INPUT), 23701357374);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(solution_part2(EXAMPLE), 4174379265);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solution_part2(INPUT), 34284458938);
    }
}
