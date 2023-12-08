use aoc2023::utils::*;
use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

const DAY: u8 = 8;

#[derive(Clone)]
struct Instructions {
    data: Vec<char>,
    current: usize,
}

impl Instructions {
    fn new(data: Vec<char>) -> Self {
        Instructions { data, current: 0 }
    }
}

impl Iterator for Instructions {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        let result = self.data.get(self.current).copied();
        self.current = (self.current + 1) % self.data.len();
        result
    }
}

fn parse_input(data: &str) -> (Instructions, HashMap<&str, (&str, &str)>) {
    let mut lines = data.lines();

    let instructions = Instructions::new(lines.next().unwrap().chars().collect());
    lines.next();

    let re = Regex::new(r"[0-9a-zA-Z]{3}").unwrap();
    let mut network = HashMap::new();
    lines
        .map(|line| {
            let mut d = re.find_iter(line);
            (
                d.next().unwrap().as_str(),
                d.map(|v| v.as_str()).collect_tuple().unwrap(),
            )
        })
        .for_each(|(key, values)| {
            network.insert(key, values);
        });

    (instructions, network)
}

fn calculate(
    mut instructions: Instructions,
    network: &HashMap<&str, (&str, &str)>,
    start: &str,
) -> u64 {
    let mut result = 0;
    let mut current_node = start;
    while !current_node.ends_with('Z') {
        match instructions.next() {
            Some('L') => current_node = network.get(current_node).map(|(left, _)| left).unwrap(),
            Some('R') => current_node = network.get(current_node).map(|(_, right)| right).unwrap(),
            _ => panic!(),
        }

        result += 1;
    }
    result
}

#[timed::timed]
fn part1(data: &str) -> u64 {
    let (instructions, network) = parse_input(data);
    calculate(instructions, &network, "AAA")
}

#[timed::timed]
fn part2(data: &str) -> u64 {
    let (instructions, network) = parse_input(data);
    network
        .keys()
        .filter(|&node| node.ends_with('A'))
        .map(|node| calculate(instructions.clone(), &network, node))
        .fold(1, num_integer::lcm)
}

fn main() {
    let input = read_input(DAY, InputType::NotTest);
    println!("Part 1: {}", part1(&input.part1));
    println!("Part 2: {}", part2(&input.part2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_rl() {
        let data = aoc2023::utils::read_input(
            DAY,
            InputType::Other(
                "RL

                AAA = (BBB, CCC)
                BBB = (DDD, EEE)
                CCC = (ZZZ, GGG)
                DDD = (DDD, DDD)
                EEE = (EEE, EEE)
                GGG = (GGG, GGG)
                ZZZ = (ZZZ, ZZZ)"
                    .to_owned(),
            ),
        );
        assert_eq!(part1(&data.part1), 2);
    }

    #[test]
    fn test_part1_llr() {
        let data = aoc2023::utils::read_input(
            DAY,
            InputType::Other(
                "LLR

                AAA = (BBB, BBB)
                BBB = (AAA, ZZZ)
                ZZZ = (ZZZ, ZZZ)"
                    .to_owned(),
            ),
        );
        assert_eq!(part1(&data.part1), 6);
    }

    #[test]
    fn test_part2() {
        let data = aoc2023::utils::read_input(
            DAY,
            InputType::Other(
                "LR

                11A = (11B, XXX)
                11B = (XXX, 11Z)
                11Z = (11B, XXX)
                22A = (22B, XXX)
                22B = (22C, 22C)
                22C = (22Z, 22Z)
                22Z = (22B, 22B)
                XXX = (XXX, XXX)"
                    .to_owned(),
            ),
        );
        assert_eq!(part2(&data.part2), 6);
    }
}
