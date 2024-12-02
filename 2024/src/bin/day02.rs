use aoc2024::utils::*;

const DAY: u8 = 2;

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

fn is_safe(line: &[i32], avoid_index: Option<usize>) -> bool {
    let filtered = line
        .iter()
        .enumerate()
        .filter_map(|(i, &d)| {
            if Some(i) == avoid_index {
                None
            } else {
                Some(d)
            }
        })
        .collect::<Vec<_>>();

    let mut diffs = filtered.windows(2).map(|w| w[1] - w[0]);

    diffs.clone().all(|d| d > 0 && d <= 3) || diffs.all(|d| d >= -3 && d < 0)
}

#[timed::timed]
fn part1(data: &str) -> usize {
    let lines = parse_input(data);
    lines.iter().filter(|&line| is_safe(line, None)).count()
}

#[timed::timed]
fn part2(data: &str) -> usize {
    let lines = parse_input(data);
    lines
        .iter()
        .filter(|&line| (0..line.len()).any(|i| is_safe(&line, Some(i))))
        .count()
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
        assert_eq!(part1(&get_test_input().part1), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&get_test_input().part2), 4);
    }
}
