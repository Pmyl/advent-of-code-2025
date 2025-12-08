// https://adventofcode.com/2025/day/8

pub fn solution_part1(input: &str, connections: usize) -> usize {
    let boxes = Pos::from_input(input);

    let mut distances = Vec::<((usize, usize), usize)>::new();
    for i in 0..boxes.len() - 1 {
        for j in i + 1..boxes.len() {
            distances.push(((i, j), boxes[i].distance(&boxes[j])));
        }
    }

    distances.select_nth_unstable_by(connections - 1, |a, b| a.1.cmp(&b.1));
    let closest_boxes = distances[..connections].into_iter().map(|d| d.0);

    let mut circuit_connections = vec![0usize; boxes.len()];
    let mut circuits: Vec<Vec<usize>> = vec![];
    for (i, j) in closest_boxes {
        let j_n = circuit_connections[j];
        let i_n = circuit_connections[i];

        if i_n == 0 && j_n == 0 {
            circuits.push(vec![i, j]);
            circuit_connections[i] = circuits.len();
            circuit_connections[j] = circuits.len();
            continue;
        }

        if i_n == 0 {
            circuit_connections[i] = j_n;
            circuits[j_n - 1].push(i);
            continue;
        }

        if j_n == 0 {
            circuit_connections[j] = i_n;
            circuits[i_n - 1].push(j);
            continue;
        }

        if i_n != j_n {
            // move all circuits from j's to i's
            let [circuit_j, circuit_i] = circuits.get_disjoint_mut([j_n - 1, i_n - 1]).unwrap();
            let to_move = circuit_j.drain(..);
            for j_neighbour in to_move {
                circuit_connections[j_neighbour] = i_n;
                circuit_i.push(j_neighbour)
            }
        }
    }

    circuits.sort_by_key(|v| v.len());
    let out = circuits
        .into_iter()
        .rev()
        .map(|v| v.len())
        .take(3)
        .product();

    out
}

pub fn solution_part2(input: &str) -> usize {
    let boxes = Pos::from_input(input);

    let mut distances = Vec::<((usize, usize), usize)>::new();
    for i in 0..boxes.len() - 1 {
        for j in i + 1..boxes.len() {
            distances.push(((i, j), boxes[i].distance(&boxes[j])));
        }
    }

    distances.sort_by_key(|d| d.1);
    let closest_boxes = distances.into_iter().map(|d| d.0);

    let mut circuit_connections = vec![0usize; boxes.len()];
    let mut circuits: Vec<Vec<usize>> = vec![];
    for (i, j) in closest_boxes {
        let j_n = circuit_connections[j];
        let i_n = circuit_connections[i];

        if i_n == 0 && j_n == 0 {
            circuits.push(vec![i, j]);
            circuit_connections[i] = circuits.len();
            circuit_connections[j] = circuits.len();
        } else if i_n == 0 {
            circuit_connections[i] = j_n;
            circuits[j_n - 1].push(i);
        } else if j_n == 0 {
            circuit_connections[j] = i_n;
            circuits[i_n - 1].push(j);
        } else if i_n != j_n {
            // move all circuits from j's to i's
            let [circuit_j, circuit_i] = circuits.get_disjoint_mut([j_n - 1, i_n - 1]).unwrap();
            let to_move = circuit_j.drain(..);
            for j_neighbour in to_move {
                circuit_connections[j_neighbour] = i_n;
                circuit_i.push(j_neighbour)
            }
        } else {
            continue;
        }

        if i_n != 0 && circuits[i_n - 1].len() == boxes.len()
            || j_n != 0 && circuits[j_n - 1].len() == boxes.len()
        {
            return boxes[i].x * boxes[j].x;
        }
    }

    unreachable!()
}

struct Pos {
    x: usize,
    y: usize,
    z: usize,
}

impl Pos {
    fn from_input(input: &str) -> Vec<Self> {
        input
            .trim()
            .lines()
            .map(|line| {
                let mut coordinates = line.splitn(3, ",");
                Self {
                    x: coordinates.next().unwrap().parse::<usize>().unwrap(),
                    y: coordinates.next().unwrap().parse::<usize>().unwrap(),
                    z: coordinates.next().unwrap().parse::<usize>().unwrap(),
                }
            })
            .collect::<Vec<Self>>()
    }

    // Not square rooted because we only care about relative distance
    fn distance(&self, pos: &Pos) -> usize {
        (self.x as isize - pos.x as isize).pow(2) as usize
            + (self.y as isize - pos.y as isize).pow(2) as usize
            + (self.z as isize - pos.z as isize).pow(2) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_part1_example() {
        assert_eq!(solution_part1(EXAMPLE, 10), 40);
    }

    #[test]
    fn test_part1() {
        assert_eq!(solution_part1(INPUT, 1000), 352584);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(solution_part2(EXAMPLE), 25272);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solution_part2(INPUT), 9617397716);
    }
}
