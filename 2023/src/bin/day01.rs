use aoc2023::utils::*;
use regex::Regex;
use std::collections::HashMap;

const DAY: u8 = 1;

#[timed::timed]
fn part1(data: &String) -> i32 {
    let re = Regex::new(r"\d").unwrap();
    data.lines()
        .map(|line| {
            let digits: Vec<i32> = re
                .find_iter(line)
                .flat_map(|v| v.as_str().parse::<i32>())
                .collect();
            digits.first().unwrap() * 10 + digits.last().unwrap()
        })
        .sum()
}

#[timed::timed]
fn part2(data: &String) -> i32 {
    let map = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let re = Regex::new(r"\d").unwrap();

    data.lines()
        .map(|line| {
            let modified_line = map.iter().fold(line.to_string(), |acc, (k, v)| {
                acc.replace(
                    k,
                    &format!(
                        "{}{}{}",
                        k.chars().next().unwrap(),
                        v,
                        k.chars().last().unwrap()
                    ),
                )
            });
            let digits: Vec<i32> = re
                .find_iter(&modified_line)
                .map(|v| v.as_str().parse::<i32>().unwrap())
                .collect();
            digits.first().unwrap() * 10 + digits.last().unwrap()
        })
        .sum()
}

fn main() {
    let input = read_input(DAY, InputType::NotTest);
    println!("Part 1: {}", part1(&input.part1));
    println!("Part 2: {}", part2(&input.part2));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_input() -> Input {
        aoc2023::utils::read_input(DAY, InputType::Test)
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&get_test_input().part1), 142);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&get_test_input().part2), 281);
    }

    #[test]
    fn test_fancy_input() {
        let input = aoc2023::utils::read_input(DAY, InputType::Other("sevenine".to_owned()));
        assert_eq!(part2(&input.part2), 79);
    }
}
