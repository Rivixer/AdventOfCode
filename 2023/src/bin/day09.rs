use aoc2023::utils::*;

const DAY: u8 = 9;

fn parse_input(data: &str) -> Vec<Vec<i64>> {
    data.lines()
        .map(|line| {
            line.split_whitespace()
                .flat_map(|v| v.parse::<i64>())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn calculate(seq: &Vec<i64>) -> i64 {
    let mut result = *seq.last().unwrap();
    let mut differences = seq.clone();
    while !differences.iter().all(|&v| v == 0) {
        differences = differences
            .iter()
            .zip(differences.iter().skip(1))
            .map(|(i, j)| j - i)
            .collect();
        result += *differences.last().unwrap();
    }
    result
}

#[timed::timed]
fn part1(data: &str) -> i64 {
    let histories = parse_input(data);
    histories.iter().map(|history| calculate(history)).sum()
}

#[timed::timed]
fn part2(data: &str) -> i64 {
    let mut histories = parse_input(data);
    histories.iter_mut().for_each(|v| v.reverse());
    histories.iter().map(|history| calculate(history)).sum()
}

fn main() {
    let input = read_input(DAY, InputType::NotTest);
    println!("Part 1: {}", part1(&input.part1));
    println!("Part 2: {}", part2(&input.part2));
}
