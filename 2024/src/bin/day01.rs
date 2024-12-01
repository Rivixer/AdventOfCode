use aoc2024::utils::*;
use regex::Regex;
use std::{collections::HashMap, error::Error};

const DAY: u8 = 1;

fn parse_input(input: &str) -> Result<(Vec<i32>, Vec<i32>), Box<dyn Error>> {
    let re = Regex::new(r"(\d+)\s+(\d+)")?;
    let mut vec1 = Vec::new();
    let mut vec2 = Vec::new();

    for cap in re.captures_iter(input) {
        vec1.push(cap[1].parse()?);
        vec2.push(cap[2].parse()?);
    }

    Ok((vec1, vec2))
}


#[timed::timed]
fn part1(data: &str) -> i32 {
    let (mut vec1, mut vec2) = parse_input(data).unwrap();
    vec1.sort();
    vec2.sort();

    vec1.iter()
        .zip(vec2.iter())
        .map(|(a, b)| (a-b).abs())
        .sum()
}

#[timed::timed]
fn part2(data: &str) -> i32 {
    let (vec1, vec2) = parse_input(data).unwrap();
    let mut map = HashMap::new();

    vec2.iter().for_each(|&num| {
        *map.entry(num).or_insert(0) += 1;
    });

    vec1.iter()
        .map(|&num| map.get(&num).unwrap_or(&0) * num)
        .sum()
}

fn main() {
    let input = read_input(DAY, InputType::Production);
    println!("Part 1: {}", part1(&input.part1));
    println!("Part 2: {}", part2(&input.part2));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_input() -> Input {
        read_input(DAY, InputType::Test)
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&get_test_input().part1), 11);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&get_test_input().part2), 31);
    }
}
