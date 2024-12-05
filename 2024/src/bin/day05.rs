use aoc2024::utils::*;
use std::cmp::Ordering;
use std::collections::HashMap;

const DAY: u8 = 5;

fn parse_input(data: &str) -> (HashMap<i32, Vec<i32>>, Vec<Vec<i32>>) {
    let mut lines = data.lines().peekable();
    let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();

    for line in &mut lines {
        if line.is_empty() {
            break;
        }

        let parts = line
            .split('|')
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        rules.entry(parts[0]).or_default().push(parts[1]);
    }

    let updates = lines
        .map(|line| {
            line.split(',')
                .filter_map(|instr_str| instr_str.parse::<i32>().ok())
                .collect()
        })
        .collect();

    (rules, updates)
}

fn get_compare_key(a: &i32, b: &i32, rules: &HashMap<i32, Vec<i32>>) -> Ordering {
    if rules.get(a).map_or(false, |v| v.contains(b)) {
        Ordering::Less
    } else if rules.get(b).map_or(false, |v| v.contains(a)) {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

#[timed::timed]
fn part1(data: &str) -> i32 {
    let (rules, updates) = parse_input(data);

    updates
        .iter()
        .filter_map(|x| {
            let mut sorted = x.clone();
            sorted.sort_by(|a, b| get_compare_key(a, b, &rules));

            if x == &sorted {
                Some(x.get(x.len() / 2).unwrap())
            } else {
                None
            }
        })
        .sum()
}

#[timed::timed]
fn part2(data: &str) -> i32 {
    let (rules, updates) = parse_input(data);

    updates
        .iter()
        .filter_map(|x| {
            let mut sorted = x.clone();
            sorted.sort_by(|a, b| get_compare_key(a, b, &rules));

            if x != &sorted {
                Some(*sorted.get(sorted.len() / 2).unwrap())
            } else {
                None
            }
        })
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
        assert_eq!(part1(&get_test_input().part1), 143);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&get_test_input().part2), 123);
    }
}
