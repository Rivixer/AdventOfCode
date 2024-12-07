use aoc2024::utils::*;

const DAY: u8 = 7;

fn parse_input(data: &str) -> Vec<(i64, Vec<i64>)> {
    data.lines()
        .map(|line| {
            let mut parts = line.split(": ");
            let value = parts.next().unwrap().parse().unwrap();
            let numbers = parts
                .next()
                .unwrap()
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();
            (value, numbers)
        })
        .collect()
}

fn is_equation_true(
    value: &i64,
    numbers: &[i64],
    acc: i64,
    functions: &[fn(i64, i64) -> i64],
) -> bool {
    if numbers.is_empty() {
        return acc == *value;
    }

    functions
        .iter()
        .any(|f| is_equation_true(value, &numbers[1..], f(acc, numbers[0]), functions))
}

#[timed::timed]
fn part1(data: &str) -> i64 {
    let input = parse_input(data);
    let functions: [fn(i64, i64) -> i64; 2] = [|a, b| a + b, |a, b| a * b];

    input
        .iter()
        .filter(|(value, numbers)| is_equation_true(value, numbers, 0, &functions))
        .map(|(value, _)| *value)
        .sum()
}

#[timed::timed]
fn part2(data: &str) -> i64 {
    let input = parse_input(data);
    let functions: [fn(i64, i64) -> i64; 3] = [
        |a, b| a + b,
        |a, b| a * b,
        |a, b| a * 10i64.pow(b.ilog(10) + 1) + b,
    ];

    input
        .iter()
        .filter(|(value, numbers)| is_equation_true(value, numbers, 0, &functions))
        .map(|(value, _)| *value)
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
        assert_eq!(part1(&get_test_input().part1), 3749);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&get_test_input().part2), 11387);
    }

    fn get_test_input() -> Input {
        read_input(DAY, InputType::Test)
    }
}
