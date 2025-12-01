// https://adventofcode.com/2024/day/1

pub fn solution_part1(input: &str) -> usize {
    let mut pointer: i32 = 50;
    let mut zeroes = 0;
    let steps = input
        .lines()
        .map(|line| {
            let mut chars = line.chars();
            (
                chars.next().unwrap(),
                chars.collect::<String>().parse::<i32>().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    for step in steps {
        match step.0 {
            'L' => {
                pointer = (pointer - step.1).rem_euclid(100);
            }
            'R' => {
                pointer = (pointer + step.1).rem_euclid(100);
            }
            _ => unreachable!(),
        }

        if pointer == 0 {
            zeroes += 1;
        }
    }

    zeroes
}

pub fn solution_part2(input: &str) -> usize {
    let mut zeroes: usize = 0;
    let steps = input
        .lines()
        .map(|line| {
            let mut chars = line.chars();
            (
                chars.next().unwrap(),
                chars.collect::<String>().parse::<i32>().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    let mut pointer: i32 = 50;
    for step in steps {
        let complete_turns = step.1 as usize / 100;
        let movements = if complete_turns != 0 {
            zeroes += complete_turns;
            step.1 - complete_turns as i32 * 100
        } else {
            step.1
        };

        let prev_pointer = pointer;
        match step.0 {
            'L' => {
                pointer -= movements;
            }
            'R' => {
                pointer += movements;
            }
            _ => unreachable!(),
        }

        let pointer_without_modulo = pointer;
        pointer = pointer.rem_euclid(100);

        if pointer == 0 {
            zeroes += 1;
        } else if pointer_without_modulo != pointer && prev_pointer != 0 {
            zeroes += 1
        }
    }

    zeroes
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_part1_example() {
        assert_eq!(solution_part1(EXAMPLE), 3);
    }

    #[test]
    fn test_part1() {
        assert_eq!(solution_part1(INPUT), 1036);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(solution_part2(EXAMPLE), 6);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solution_part2(INPUT), 6228);
    }
}
