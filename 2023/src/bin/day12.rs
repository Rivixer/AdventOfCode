use aoc2023::utils::*;
use itertools::Itertools;
use std::{collections::HashMap, iter::repeat};

const DAY: u8 = 12;

#[derive(PartialEq)]
enum SpringType {
    Operational,
    Damaged,
    Unknown,
}

fn parse_input(data: &str, factor: usize) -> Vec<(Vec<SpringType>, Vec<usize>)> {
    data.lines()
        .map(|line| {
            let (springs, groups) = line.split_once(" ").unwrap();

            let springs = repeat(springs)
                .take(factor)
                .join("?")
                .chars()
                .map(|v| match v {
                    '.' => SpringType::Operational,
                    '#' => SpringType::Damaged,
                    '?' => SpringType::Unknown,
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>();

            let groups = repeat(groups)
                .take(factor)
                .join(",")
                .split(',')
                .flat_map(|v| v.parse::<usize>())
                .collect::<Vec<usize>>();

            (springs, groups)
        })
        .collect()
}

fn calculate(
    springs: &Vec<SpringType>,
    groups: &Vec<usize>,
    current_spring: usize,
    current_group: usize,
    damaged_counter: usize,
    cache: &mut HashMap<(usize, usize, usize), usize>,
) -> usize {
    let cache_key = (current_spring, current_group, damaged_counter);

    if cache.contains_key(&cache_key) {
        return *cache.get(&cache_key).unwrap();
    }

    if current_spring == springs.len() {
        return (current_group == groups.len() && damaged_counter == 0
            || current_group == groups.len() - 1 && groups[current_group] == damaged_counter)
            as usize;
    }

    let mut result = 0;
    if matches!(
        springs[current_spring],
        SpringType::Operational | SpringType::Unknown
    ) {
        if damaged_counter == 0 {
            result += calculate(
                springs,
                groups,
                current_spring + 1,
                current_group,
                damaged_counter,
                cache,
            )
        } else if damaged_counter > 0
            && current_group < groups.len()
            && groups[current_group] == damaged_counter
        {
            result += calculate(
                springs,
                groups,
                current_spring + 1,
                current_group + 1,
                0,
                cache,
            )
        }
    }

    if matches!(
        springs[current_spring],
        SpringType::Damaged | SpringType::Unknown
    ) {
        result += calculate(
            springs,
            groups,
            current_spring + 1,
            current_group,
            damaged_counter + 1,
            cache,
        );
    }

    cache.insert(cache_key, result);
    result
}

#[timed::timed]
fn part1(data: &str) -> usize {
    let input = parse_input(data, 1);
    input
        .iter()
        .map(|(springs, groups)| calculate(springs, groups, 0, 0, 0, &mut HashMap::new()))
        .sum()
}

#[timed::timed]
fn part2(data: &str) -> usize {
    let input = parse_input(data, 5);
    input
        .iter()
        .map(|(springs, groups)| calculate(springs, groups, 0, 0, 0, &mut HashMap::new()))
        .sum()
}

fn main() {
    let input = read_input(DAY, InputType::NotTest);
    println!("Part 1: {}", part1(&input.part1));
    println!("Part 2: {}", part2(&input.part2));
}
