// https://adventofcode.com/2025/day/6

pub fn solution_part1(input: &str) -> usize {
    MathWorksheet::from_input(input).solve()
}

pub fn solution_part2(input: &str) -> usize {
    MathWorksheet::from_input_right_to_left(input).solve()
}

struct MathWorksheet {
    problems: Vec<MathProblem>,
}

impl MathWorksheet {
    fn from_input(input: &str) -> Self {
        let lines: Vec<&str> = input.trim().lines().collect();
        let operations_line = lines.last().unwrap();
        let numbers_lines = &lines[..lines.len() - 1];
        let operations = operations_line
            .chars()
            .filter(|c| *c != ' ')
            .collect::<Vec<_>>();

        let mut problems = vec![MathProblem::default(); operations.len()];
        for (i, operation) in operations.into_iter().enumerate() {
            problems[i].operation = operation;
        }

        for line in numbers_lines {
            for (i, number) in line.split_whitespace().enumerate() {
                problems[i].numbers.push(number.parse::<usize>().unwrap());
            }
        }

        Self { problems }
    }

    fn from_input_right_to_left(input: &str) -> Self {
        let lines: Vec<&str> = input.lines().collect();
        let operations_line = lines.last().unwrap();
        let numbers_lines = &lines[..lines.len() - 1];
        let operations = operations_line
            .chars()
            .filter(|c| *c != ' ')
            .collect::<Vec<_>>();

        let mut problems = vec![MathProblem::default(); operations.len()];
        for (i, operation) in operations.into_iter().enumerate() {
            problems[i].operation = operation;
        }

        let numbers_lines = numbers_lines
            .into_iter()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let max_digits = numbers_lines.len();
        let width = numbers_lines[0].len();
        let mut problems_index = 0;

        for j in 0..width {
            let mut n: usize = 0;
            let mut empty_column = true;
            for i in 0..max_digits {
                if numbers_lines[i][j] != ' ' {
                    n = n * 10 + numbers_lines[i][j].to_digit(10).unwrap() as usize;
                    empty_column = false;
                }
            }
            if empty_column {
                problems_index += 1;
                continue;
            }
            problems[problems_index].numbers.push(n);
        }

        Self { problems }
    }

    fn solve(self) -> usize {
        self.problems.into_iter().map(|p| p.solve()).sum()
    }
}

#[derive(Default, Clone)]
struct MathProblem {
    numbers: Vec<usize>,
    operation: char,
}

impl MathProblem {
    fn solve(self) -> usize {
        match self.operation {
            '+' => self.numbers.into_iter().sum(),
            '*' => self.numbers.into_iter().product(),
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ";

    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_part1_example() {
        assert_eq!(solution_part1(EXAMPLE), 4277556);
    }

    #[test]
    fn test_part1() {
        assert_eq!(solution_part1(INPUT), 4364617236318);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(solution_part2(EXAMPLE), 3263827);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solution_part2(INPUT), 9077004354241);
    }
}
