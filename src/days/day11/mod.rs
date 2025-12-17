// https://adventofcode.com/2025/day/11

use std::{borrow::Cow, collections::HashMap};

pub fn solution_part1(input: &str) -> usize {
    let server_rack = ServerRack::from_input(input);
    count_paths(&server_rack, "you")
}

pub fn solution_part2(input: &str) -> usize {
    let server_rack = ServerRack::from_input(input);
    count_paths_including(
        &server_rack,
        "svr",
        &HashMap::from_iter(vec![("fft", 0), ("dac", 1)]),
        Cow::Owned(vec![false, false]),
        &mut HashMap::new(),
    )
}

fn count_paths<'a>(server_rack: &ServerRack<'a>, device: &'a str) -> usize {
    if device == "out" {
        return 1;
    }

    let outputs = server_rack.devices.get(device).unwrap();
    outputs.iter().map(|o| count_paths(server_rack, o)).sum()
}

fn count_paths_including<'a>(
    server_rack: &ServerRack<'a>,
    device: &'a str,
    mandatory_devices: &'a HashMap<&'a str, usize>,
    mut mandatory_devices_found: Cow<'a, Vec<bool>>,
    memo: &mut HashMap<(&'a str, Cow<'a, Vec<bool>>), usize>,
) -> usize {
    if device == "out" {
        return (mandatory_devices.len() == mandatory_devices_found.iter().filter(|d| **d).count())
            as usize;
    }

    if let Some(count) = memo.get(&(device, mandatory_devices_found.clone())) {
        return *count;
    }

    mandatory_devices_found = if let Some(index) = mandatory_devices.get(device) {
        let mut cloned = mandatory_devices_found.as_slice().to_vec();
        cloned[*index] = true;
        Cow::Owned(cloned)
    } else {
        mandatory_devices_found
    };
    let outputs = server_rack.devices.get(device).unwrap();
    let count = outputs
        .iter()
        .map(|o| {
            count_paths_including(
                server_rack,
                o,
                mandatory_devices,
                mandatory_devices_found.clone(),
                memo,
            )
        })
        .sum();
    memo.insert((device, mandatory_devices_found), count);
    count
}

struct ServerRack<'a> {
    devices: HashMap<&'a str, Vec<&'a str>>,
}

impl<'a> ServerRack<'a> {
    fn from_input(input: &'a str) -> Self {
        Self {
            devices: input
                .trim()
                .lines()
                .map(|line| {
                    let (name, outputs) = line.split_once(": ").unwrap();
                    let outputs = outputs.split(" ").collect::<Vec<_>>();

                    (name, outputs)
                })
                .collect::<HashMap<_, _>>(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";

    const EXAMPLE_P2: &str = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";

    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_part1_example() {
        assert_eq!(solution_part1(EXAMPLE), 5);
    }

    #[test]
    fn test_part1_real() {
        assert_eq!(solution_part1(INPUT), 788);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(solution_part2(EXAMPLE_P2), 2);
    }

    #[test]
    fn test_part2_real() {
        assert_eq!(solution_part2(INPUT), 316291887968000);
    }
}
