use aoc2023::utils::*;
use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

const DAY: u8 = 2;

fn capture(data: &str) -> Vec<(char, i32)> {
    let re = Regex::new(r"(\d+) (r|g|b)").unwrap();
    re.captures_iter(data)
        .map(|c| c.extract())
        .map(|(_, [v, c])| (c.chars().next().unwrap(), v.parse::<i32>().unwrap()))
        .collect()
}

#[timed::timed]
fn part1(data: &str) -> i32 {
    data.lines()
        .enumerate()
        .filter(|(_, line)| {
            let mut map = [('r', 12), ('g', 13), ('b', 14)]
                .iter()
                .cloned()
                .collect::<HashMap<_, _>>();
            line.split(';').all(|set| {
                capture(set)
                    .iter()
                    .all(|(c, v)| *map.entry(*c).or_default() >= *v)
            })
        })
        .map(|(i, _)| (i + 1) as i32)
        .sum()
}

#[timed::timed]
fn part2(data: &str) -> i32 {
    data.lines()
        .flat_map(|line| {
            capture(line)
                .iter()
                .fold(HashMap::new(), |mut acc, (c, v)| {
                    acc.entry(c).and_modify(|e| *e = v.max(*e)).or_insert(v);
                    acc
                })
                .into_values()
                .product1::<i32>()
        })
        .sum()
}

fn main() {
    let input = read_input(DAY, InputType::NotTest);
    println!("Part 1: {}", part1(&input.part1));
    println!("Part 2: {}", part2(&input.part2));
}
