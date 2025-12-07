// https://adventofcode.com/2025/day/7

use std::collections::BTreeSet;

pub fn solution_part1(input: &str) -> usize {
    let input = input.trim().as_bytes();
    let mut splits = 0;
    let mut i = 0;
    let row_size;
    let mut beams: BTreeSet<usize> = BTreeSet::from_iter(vec![{
        let s_column;
        loop {
            if input[i] == b'S' {
                s_column = i;
                while input[i] != b'\n' {
                    i += 1;
                }
                row_size = i;
                i += 1;
                break;
            }
            i += 1;
        }
        s_column
    }]);

    loop {
        while i < input.len() && input[i] != b'\n' {
            if input[i] == b'^' {
                // row_size + 1 because we need to include \n
                let column = i % (row_size + 1);
                if let Some(_) = beams.take(&column) {
                    splits += 1;
                    beams.insert(column - 1);
                    beams.insert(column + 1);
                }
            }
            i += 1;
        }

        if i == input.len() {
            break;
        } else {
            i += 1;
        }
    }

    splits
}

pub fn solution_part2(input: &str) -> usize {
    let input = input.trim().as_bytes();
    let mut i = 0;
    let row_size;
    let start: usize = {
        let s_column;
        loop {
            if input[i] == b'S' {
                s_column = i;
                while input[i] != b'\n' {
                    i += 1;
                }
                row_size = i;
                break;
            }
            i += 1;
        }
        s_column
    };

    let mut cache = vec![-1isize; input.len()];
    through_timelines_recursive(input, start, row_size, &mut cache) as usize
}

fn through_timelines_recursive(
    input: &[u8],
    mut i: usize,
    row_size: usize,
    cache: &mut Vec<isize>,
) -> isize {
    i += row_size + 1;
    if i >= input.len() {
        return 1;
    }

    if cache[i] != -1 {
        return cache[i];
    }

    if input[i] == b'^' {
        cache[i] = through_timelines_recursive(input, i - 1, row_size, cache)
            + through_timelines_recursive(input, i + 1, row_size, cache);
    } else {
        cache[i] = through_timelines_recursive(input, i, row_size, cache);
    }

    cache[i]
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_part1_example() {
        assert_eq!(solution_part1(EXAMPLE), 21);
    }

    #[test]
    fn test_part1() {
        assert_eq!(solution_part1(INPUT), 1566);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(solution_part2(EXAMPLE), 40);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solution_part2(INPUT), 5921061943075);
    }
}
