// https://adventofcode.com/2025/day/3

pub fn solution_part1(input: &str) -> usize {
    let batteries = Battery::from_input(input);
    batteries.into_iter().map(|b| b.max_joltage(2)).sum()
}

pub fn solution_part2(input: &str) -> usize {
    let batteries = Battery::from_input(input);
    batteries.into_iter().map(|b| b.max_joltage(12)).sum()
}

struct Battery {
    banks: Vec<u8>,
}

impl Battery {
    fn from_input(input: &str) -> Vec<Self> {
        input
            .trim()
            .lines()
            .map(|b| Self {
                banks: b.chars().map(|c| c.to_digit(10).unwrap() as u8).collect(),
            })
            .collect::<Vec<_>>()
    }

    fn max_joltage(&self, banks_count: usize) -> usize {
        let mut max_joltage: usize = 0;
        let mut max_joltage_len = 0;
        for bank in &self.banks {
            if max_joltage_len != banks_count {
                max_joltage = max_joltage * 10 + *bank as usize;
                max_joltage_len += 1;
                continue;
            }

            for i_to_purge in 0..banks_count {
                let mask = 10usize.pow(banks_count as u32 - i_to_purge as u32);
                let left = max_joltage / mask * mask;
                let right = max_joltage % mask;
                let right = right * 10 % mask;
                let new = left + right + *bank as usize;
                if max_joltage < new {
                    max_joltage = new;
                    break;
                }
            }
        }

        max_joltage
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_part1_example() {
        assert_eq!(solution_part1(EXAMPLE), 357);
    }

    #[test]
    fn test_part1() {
        assert_eq!(solution_part1(INPUT), 17412);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(solution_part2(EXAMPLE), 3121910778619);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solution_part2(INPUT), 172681562473501);
    }
}
