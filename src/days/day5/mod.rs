// https://adventofcode.com/2025/day/5

pub fn solution_part1(input: &str) -> usize {
    Database::from_input(input).how_many_fresh()
}

pub fn solution_part2(input: &str) -> usize {
    Database::from_input(input).how_many_possible_fresh()
}

struct Database {
    fresh_ids_ranges: Vec<Range>,
    ids: Vec<usize>,
}

#[derive(PartialEq, Eq, Ord, Debug)]
struct Range {
    start: usize,
    end: usize,
}

impl Database {
    fn from_input(input: &str) -> Self {
        let (ranges, ids) = input.split_once("\n\n").unwrap();
        let mut ranges = ranges
            .trim()
            .lines()
            .map(|r| Range::from_input(r))
            .collect::<Vec<_>>();
        let mut ids = ids
            .trim()
            .lines()
            .map(|id| id.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        ranges.sort();
        ids.sort();

        Self {
            fresh_ids_ranges: ranges,
            ids,
        }
    }

    fn how_many_fresh(self) -> usize {
        let mut r_i = 0;
        let mut id_i = 0;
        let mut how_many = 0;

        loop {
            let range = &self.fresh_ids_ranges[r_i];
            let id = &self.ids[id_i];
            if range.start <= *id && *id <= range.end {
                how_many += 1;
                id_i += 1;
                if id_i == self.ids.len() {
                    break;
                }
            } else if range.end < *id {
                r_i += 1;
                if r_i == self.fresh_ids_ranges.len() {
                    break;
                }
            } else {
                id_i += 1;
                if id_i == self.ids.len() {
                    break;
                }
            }
        }

        how_many
    }

    fn how_many_possible_fresh(self) -> usize {
        let mut how_many = 0;
        let mut last_end = 0;

        for range in &self.fresh_ids_ranges {
            let range_size = range.end - range.start + 1;
            let range_already_added = if last_end >= range.start {
                last_end - range.start + 1
            } else {
                0
            };

            if range_already_added < range_size {
                how_many += range_size - range_already_added;
            }
            last_end = last_end.max(range.end);
        }

        how_many
    }
}

impl Range {
    fn from_input(input: &str) -> Self {
        let (start, end) = input.split_once("-").unwrap();
        Self {
            start: start.parse::<usize>().unwrap(),
            end: end.parse::<usize>().unwrap(),
        }
    }
}

// Order by start first and by end only if start are equal
impl PartialOrd for Range {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.start.partial_cmp(&other.start) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.end.partial_cmp(&other.end)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_part1_example() {
        assert_eq!(solution_part1(EXAMPLE), 3);
    }

    #[test]
    fn test_part1() {
        assert_eq!(solution_part1(INPUT), 874);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(solution_part2(EXAMPLE), 14);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solution_part2(INPUT), 348548952146313);
    }
}
