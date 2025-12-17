// https://adventofcode.com/2025/day/9

use std::io::Write;
use std::{collections::HashSet, fmt::Display, fs::OpenOptions};

pub fn solution_part1(input: &str) -> usize {
    let mut min_distance_tr = usize::MAX;
    let mut min_tr = Pos { x: 0, y: 0 };
    let mut max_distance_tr = 0;
    let mut max_tr = Pos { x: 0, y: 0 };
    let mut min_distance_tl = usize::MAX;
    let mut min_tl = Pos { x: 0, y: 0 };
    let mut max_distance_tl = 0;
    let mut max_tl = Pos { x: 0, y: 0 };

    let map = Map::from_input(input);
    let top_left = Pos { x: 0, y: 0 };
    let top_right = Pos { x: map.width, y: 0 };
    for pos in map.positions {
        let distance_tl = top_left.relative_distance(&pos);
        if distance_tl < min_distance_tl {
            min_distance_tl = distance_tl;
            min_tl = pos.clone();
        }

        if distance_tl > max_distance_tl {
            max_distance_tl = distance_tl;
            max_tl = pos.clone();
        }

        let distance_tr = top_right.relative_distance(&pos);
        if distance_tr < min_distance_tr {
            min_distance_tr = distance_tr;
            min_tr = pos.clone();
        }

        if distance_tr > max_distance_tr {
            max_distance_tr = distance_tr;
            max_tr = pos;
        }
    }

    ((max_tl.x - min_tl.x + 1) * (max_tl.y - min_tl.y + 1))
        .max((min_tr.x - max_tr.x + 1) * (max_tr.y - min_tr.y + 1))
}

pub fn solution_part2(input: &str) -> usize {
    let map = Map::from_input(input);
    let pixel_size = 200;
    let height = map
        .positions
        .iter()
        .map(|p| p.y / pixel_size)
        .max()
        .unwrap();
    let width = map.width / pixel_size;
    let map_map = HashSet::<Pos>::from_iter(map.positions.iter().map(|p| Pos {
        x: p.x / pixel_size,
        y: p.y / pixel_size,
    }));
    println!("Original: {} - New: {}", map.positions.len(), map_map.len());
    let mut f = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open("output.txt")
        .unwrap();
    for j in 0..height + 2 {
        for i in 0..width + 2 {
            if map_map.contains(&Pos { x: i, y: j }) {
                write!(f, "#").unwrap();
            } else {
                write!(f, ".").unwrap();
            }
        }
        writeln!(f).unwrap();
    }
    0
}

struct Map {
    width: usize,
    positions: Vec<Pos>,
}

impl Map {
    fn from_input(input: &str) -> Self {
        let mut width = 0;
        let positions = input
            .trim()
            .lines()
            .map(|l| {
                let (x, y) = l.split_once(",").unwrap();
                let x = x.parse::<usize>().unwrap();
                let y = y.parse::<usize>().unwrap();
                width = width.max(x);
                Pos { x, y }
            })
            .collect::<Vec<_>>();

        Self { width, positions }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn relative_distance(&self, pos: &Pos) -> usize {
        self.x.abs_diff(pos.x) + self.y.abs_diff(pos.y)
    }
}

impl Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_part1_example() {
        assert_eq!(solution_part1(EXAMPLE), 50);
    }

    #[test]
    fn test_part1() {
        assert_eq!(solution_part1(INPUT), 4725826296);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(solution_part2(EXAMPLE), 24);
    }

    #[test]
    fn test_part2_real() {
        assert_eq!(solution_part2(INPUT), 9999999999999);
    }
}
