use aoc2023::utils::*;
use itertools::Itertools;

const DAY: u8 = 6;

fn calculate(times: Vec<u64>, distances: Vec<u64>) -> u64 {
    times
        .iter()
        .zip(&distances)
        .map(|(&time, &distance)| (0..time).filter(|&i| distance < (time - i) * i).count() as u64)
        .product()
}

#[timed::timed]
fn part1(data: &str) -> u64 {
    let (times, distances) = data
        .lines()
        .map(|line| {
            line.split_whitespace()
                .flat_map(|v| v.parse::<u64>())
                .collect::<Vec<_>>()
        })
        .collect_tuple()
        .unwrap();

    calculate(times, distances)
}

#[timed::timed]
fn part2(data: &str) -> u64 {
    let (time, distance) = data
        .lines()
        .flat_map(|line| {
            line.chars()
                .filter(|v| v.is_digit(10))
                .collect::<String>()
                .parse::<u64>()
        })
        .collect_tuple()
        .unwrap();

    calculate(vec![time], vec![distance])
}

fn main() {
    let input = read_input(DAY, InputType::NotTest);
    println!("Part 1: {}", part1(&input.part1));
    println!("Part 2: {}", part2(&input.part2));
}
