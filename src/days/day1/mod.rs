// https://adventofcode.com/2025/day/1

pub fn solution_part1(input: &str) -> usize {
    let mut pointer: i32 = 50;
    let mut zeroes = 0;
    let steps = Step::from_input(input);

    for step in steps {
        match step.dir {
            Dir::Left => pointer = (pointer - step.amount).rem_euclid(100),
            Dir::Right => pointer = (pointer + step.amount).rem_euclid(100),
        }

        if pointer == 0 {
            zeroes += 1;
        }
    }

    zeroes
}

pub fn solution_part2(input: &str) -> usize {
    let mut zeroes: usize = 0;
    let steps = Step::from_input(input);

    let mut pointer: i32 = 50;
    for step in steps {
        let multiplier = (step.dir == Dir::Left) as i32 * 2 - 1;
        let complete_turns = step.amount as usize / 100;
        zeroes += complete_turns;
        let movements = step.amount - complete_turns as i32 * 100;

        let prev_pointer = pointer;
        pointer += movements * multiplier;

        let pointer_without_modulo = pointer;
        pointer = pointer.rem_euclid(100);

        if pointer == 0 || pointer_without_modulo != pointer && prev_pointer != 0 {
            zeroes += 1
        }
    }

    zeroes
}

struct Step {
    dir: Dir,
    amount: i32,
}

#[derive(PartialEq)]
enum Dir {
    Left,
    Right,
}

impl Step {
    fn from_input(input: &str) -> Vec<Self> {
        input
            .lines()
            .map(|line| {
                let mut chars = line.chars();
                Self {
                    dir: match chars.next().unwrap() {
                        'L' => Dir::Left,
                        'R' => Dir::Right,
                        _ => unreachable!(),
                    },
                    amount: chars.collect::<String>().parse::<i32>().unwrap(),
                }
            })
            .collect::<Vec<_>>()
    }
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
        assert_eq!(solution_part1(INPUT), 1191);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(solution_part2(EXAMPLE), 6);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solution_part2(INPUT), 6858);
    }
}
