use aoc2024::utils::*;
use std::collections::HashMap;

const DAY: u8 = 11;

fn parse_input(data: &str) -> Vec<u64> {
    data.split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect()
}

fn process_stone(value: u64, remaining_blinks: u32, cache: &mut HashMap<(u64, u32), u64>) -> u64 {
    if remaining_blinks == 0 {
        return 1;
    }

    if let Some(&cached_result) = cache.get(&(value, remaining_blinks)) {
        return cached_result;
    }

    let result = if value == 0 {
        process_stone(1, remaining_blinks - 1, cache)
    } else {
        let num_digits = value.ilog(10) + 1;
        if num_digits % 2 == 1 {
            process_stone(value * 2024, remaining_blinks - 1, cache)
        } else {
            let divisor = 10u64.pow(num_digits / 2);
            let left = value / divisor;
            let right = value % divisor;
            let left_result = process_stone(left, remaining_blinks - 1, cache);
            let right_result = process_stone(right, remaining_blinks - 1, cache);
            left_result + right_result
        }
    };

    cache.insert((value, remaining_blinks), result);
    result
}

#[timed::timed]
fn part1(data: &str) -> u64 {
    let stones = parse_input(data);
    stones
        .iter()
        .map(|&x| process_stone(x, 25, &mut HashMap::new()))
        .sum()
}

#[timed::timed]
fn part2(data: &str) -> u64 {
    let stones = parse_input(data);
    stones
        .iter()
        .map(|&x| process_stone(x, 75, &mut HashMap::new()))
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

    #[test]
    fn test_part1() {
        assert_eq!(part1(&get_test_input().part1), 55312);
    }

    fn get_test_input() -> Input {
        read_input(DAY, InputType::Test)
    }
}
