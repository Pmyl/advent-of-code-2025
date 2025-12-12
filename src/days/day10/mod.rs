// https://adventofcode.com/2025/day/1

use std::fmt::Display;

pub fn solution_part1(input: &str) -> usize {
    let diagrams = Diagram::from_input(input);
    diagrams
        .into_iter()
        .map(|diagram| diagram.min_presses())
        .sum()
}

pub fn solution_part2(input: &str) -> usize {
    let diagrams = Diagram::from_input(input);
    diagrams
        .into_iter()
        .map(|diagram| diagram.min_presses_joltage())
        .sum()
}

struct Diagram {
    indicator_lights: IndicatorLights,
    // list of buttons, each button toggle a list of lights between on/off
    button_wirings: Vec<Vec<usize>>,
    joltage_requirements: Joltage,
}

// each bit represents a light, size is not important, only the 1s (lights on) are important
#[derive(Copy, Clone)]
struct IndicatorLights(u16);

// each block of 12 bits is a counter. 12 bits should be enough for each counter, and there are max 10 counters 10 * 12 = 120 with 8 leftovers
#[derive(Copy, Clone)]
struct Joltage(u128);

impl Diagram {
    fn from_input(input: &str) -> Vec<Self> {
        input
            .trim()
            .lines()
            .into_iter()
            .map(|l| {
                let l: Vec<&str> = l.split(" ").collect();
                let [indicators, buttons @ .., joltage] = &*l else {
                    unreachable!(
                        "Mr. Advent of Code personally promised me this would never happen"
                    );
                };
                let indicator_lights = IndicatorLights::from_input(indicators);

                let button_wirings = buttons
                    .into_iter()
                    .map(|b| {
                        let b = b.trim_matches(&['(', ')']);
                        b.split(',').map(|b| b.parse::<usize>().unwrap()).collect()
                    })
                    .collect();

                let joltage_requirements = Joltage::from_input(joltage);

                Self {
                    indicator_lights,
                    button_wirings,
                    joltage_requirements,
                }
            })
            .collect::<Vec<_>>()
    }

    fn min_presses(&self) -> usize {
        min_presses_recursive(self, 0, IndicatorLights(0))
    }

    fn min_presses_joltage(&self) -> usize {
        min_presses_joltage_recursive(self, 0, self.joltage_requirements)
    }
}

fn min_presses_recursive(diagram: &Diagram, i: usize, mut current: IndicatorLights) -> usize {
    if diagram.indicator_lights.0 == current.0 {
        return 0;
    }

    if diagram.button_wirings.len() == i {
        return usize::MAX - 1;
    }

    let no_press = min_presses_recursive(diagram, i + 1, current);
    let press = min_presses_recursive(
        diagram,
        i + 1,
        current.toggle_many(diagram.button_wirings[i].iter().copied()),
    );
    no_press.min(1 + press)
}

fn min_presses_joltage_recursive(diagram: &Diagram, i: usize, current: Joltage) -> usize {
    if current.is_empty() {
        return 0;
    }

    let press = if diagram.button_wirings[i]
        .iter()
        .any(|b| current.counter(*b) == 0)
    {
        // println!(
        //     "Press is MAX. i: {} Buttons: {:?}, Current: {}",
        //     i, diagram.button_wirings[i], current
        // );
        usize::MAX - 1
    } else {
        // println!(
        //     "Press is GOOD. i: {} Buttons: {:?}, Current: {}",
        //     i, diagram.button_wirings[i], current
        // );
        min_presses_joltage_recursive(
            diagram,
            i,
            current.decrement_many(diagram.button_wirings[i].iter().copied()),
        )
    };

    let no_press = if diagram.button_wirings.len() == i + 1 {
        // println!(
        //     "No Press is MAX. i: {} Buttons: {:?}, Current: {}",
        //     i, diagram.button_wirings[i], current
        // );
        usize::MAX - 1
    } else {
        // println!(
        //     "No Press is GOOD. i: {} Buttons: {:?}, Current: {}",
        //     i, diagram.button_wirings[i], current
        // );
        min_presses_joltage_recursive(diagram, i + 1, current)
    };

    let res = no_press.min(1 + press);
    // println!(
    //     "Req: {} Curr: {} i: {} buttons: {:?} NoPress: {} Press: {} Res: {}",
    //     diagram.joltage_requirements,
    //     current,
    //     i,
    //     diagram.button_wirings[i],
    //     if no_press != usize::MAX - 1 {
    //         no_press.to_string()
    //     } else {
    //         "MAX".to_string()
    //     },
    //     if press != usize::MAX - 1 {
    //         press.to_string()
    //     } else {
    //         "MAX".to_string()
    //     },
    //     res
    // );
    res
}

impl IndicatorLights {
    fn from_input(input: &str) -> Self {
        let indicators = input.trim_matches(&['[', ']']);
        IndicatorLights(0).toggle_many(
            indicators
                .bytes()
                .enumerate()
                .filter_map(|(i, b)| if b == b'#' { Some(i) } else { None }),
        )
    }

    fn toggle_many<T>(&mut self, i: T) -> IndicatorLights
    where
        T: IntoIterator<Item = usize>,
    {
        IndicatorLights(
            i.into_iter()
                .fold(self.0, |acc, a| acc ^ 2u16.pow(a as u32)),
        )
    }
}

const JOLTAGE_BITS: usize = 12;
const JOLTAGE_MAX_N: usize = 10;
impl Joltage {
    fn from_input(input: &str) -> Self {
        let joltage = input.trim_matches(&['{', '}']);
        let mut size = 0;
        let joltage_bits = joltage
            .split(",")
            .map(|n| n.parse::<usize>().unwrap())
            .enumerate()
            .fold(0u128, |acc, (i, n)| {
                size += 1;
                acc + 2u128.pow(JOLTAGE_BITS as u32 * i as u32) * n as u128
            });
        Joltage(joltage_bits | (size << (JOLTAGE_BITS * JOLTAGE_MAX_N)))
    }

    fn decrement_many<T>(&self, i: T) -> Joltage
    where
        T: IntoIterator<Item = usize>,
    {
        Joltage(
            i.into_iter().fold(self.0, |acc, a| {
                acc - 2u128.pow(JOLTAGE_BITS as u32 * a as u32)
            }) | ((self.size() as u128) << (JOLTAGE_BITS * JOLTAGE_MAX_N)),
        )
    }

    fn counter(&self, i: usize) -> u128 {
        (self.0 >> (i * JOLTAGE_BITS)) % 2u128.pow(JOLTAGE_BITS as u32)
    }

    fn size(&self) -> usize {
        (self.0 >> (JOLTAGE_MAX_N * JOLTAGE_BITS)) as usize
    }

    fn is_empty(&self) -> bool {
        (self.0 << (128 - JOLTAGE_MAX_N * JOLTAGE_BITS)) == 0
    }
}

impl Display for Joltage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;
        for i in 0..self.size() {
            write!(f, "{},", self.counter(i))?;
        }
        write!(f, "}}")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn joltage() {
        let joltage = Joltage::from_input("{10,11,11,5,10,5}");
        assert_eq!(joltage.to_string(), "{10,11,11,5,10,5,}");
        assert_eq!(joltage.size(), 6);
        assert_eq!(joltage.counter(0), 10);
        assert_eq!(joltage.counter(1), 11);
        assert_eq!(joltage.counter(2), 11);
        assert_eq!(joltage.counter(3), 5);
        assert_eq!(joltage.counter(4), 10);
        assert_eq!(joltage.counter(5), 5);
        assert!(!joltage.is_empty());

        let decremented = joltage.decrement_many([0, 1, 2, 4, 5]);
        assert_eq!(decremented.to_string(), "{9,10,10,5,9,4,}");
        assert_eq!(decremented.size(), 6);
        assert_eq!(decremented.counter(0), 9);
        assert_eq!(decremented.counter(1), 10);
        assert_eq!(decremented.counter(2), 10);
        assert_eq!(decremented.counter(3), 5);
        assert_eq!(decremented.counter(4), 9);
        assert_eq!(decremented.counter(5), 4);
        assert!(!decremented.is_empty());

        let zero = decremented
            .decrement_many([0, 1, 2, 3, 4, 5])
            .decrement_many([0, 1, 2, 3, 4, 5])
            .decrement_many([0, 1, 2, 3, 4, 5])
            .decrement_many([0, 1, 2, 3, 4, 5])
            .decrement_many([0, 1, 2, 3, 4])
            .decrement_many([0, 1, 2, 4])
            .decrement_many([0, 1, 2, 4])
            .decrement_many([0, 1, 2, 4])
            .decrement_many([0, 1, 2, 4])
            .decrement_many([1, 2]);

        assert_eq!(zero.to_string(), "{0,0,0,0,0,0,}");
        assert_eq!(zero.size(), 6);
        assert!(zero.is_empty());
    }

    #[test]
    fn test_part1_example() {
        assert_eq!(solution_part1(EXAMPLE), 7);
    }

    #[test]
    fn test_part1() {
        assert_eq!(solution_part1(INPUT), 538);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(solution_part2(EXAMPLE), 33);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solution_part2(INPUT), 0);
    }
}
