// https://adventofcode.com/2025/day/4

pub fn solution_part1(input: &str) -> usize {
    let map = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut rolls = 0;
    for i in 0..map.len() {
        let line = &map[i];
        for y in 0..line.len() {
            if map[i][y] != '@' {
                continue;
            }

            let how_many = (i != 0 && map[i - 1][y] == '@') as usize
                + (i != 0 && y != 0 && map[i - 1][y - 1] == '@') as usize
                + (y != 0 && map[i][y - 1] == '@') as usize
                + (y != 0 && i != map.len() - 1 && map[i + 1][y - 1] == '@') as usize
                + (i != map.len() - 1 && map[i + 1][y] == '@') as usize
                + (i != map.len() - 1 && y != line.len() - 1 && map[i + 1][y + 1] == '@') as usize
                + (y != line.len() - 1 && map[i][y + 1] == '@') as usize
                + (y != line.len() - 1 && i != 0 && map[i - 1][y + 1] == '@') as usize;
            if how_many < 4 {
                rolls += 1;
            }
        }
    }

    rolls
}

pub fn solution_part2(input: &str) -> usize {
    let mut map = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut rolls = 0;
    loop {
        let mut to_add = 0;
        for i in 0..map.len() {
            for y in 0..map[i].len() {
                if map[i][y] != '@' {
                    continue;
                }

                let how_many = (i != 0 && map[i - 1][y] == '@') as usize
                    + (i != 0 && y != 0 && map[i - 1][y - 1] == '@') as usize
                    + (y != 0 && map[i][y - 1] == '@') as usize
                    + (y != 0 && i != map.len() - 1 && map[i + 1][y - 1] == '@') as usize
                    + (i != map.len() - 1 && map[i + 1][y] == '@') as usize
                    + (i != map.len() - 1 && y != map[i].len() - 1 && map[i + 1][y + 1] == '@')
                        as usize
                    + (y != map[i].len() - 1 && map[i][y + 1] == '@') as usize
                    + (y != map[i].len() - 1 && i != 0 && map[i - 1][y + 1] == '@') as usize;
                if how_many < 4 {
                    map[i][y] = 'x';
                    to_add += 1;
                }
            }
        }

        if to_add == 0 {
            break;
        } else {
            rolls += to_add;
        }
    }

    rolls
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_part1_example() {
        assert_eq!(solution_part1(EXAMPLE), 13);
    }

    #[test]
    fn test_part1() {
        assert_eq!(solution_part1(INPUT), 1523);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(solution_part2(EXAMPLE), 43);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solution_part2(INPUT), 0);
    }
}
