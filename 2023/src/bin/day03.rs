use aoc2023::utils::*;
use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

const DAY: u8 = 3;

#[derive(Debug, Clone, Copy)]
struct Number {
    number: u32,
    line_index: i32,
    start: i32,
    end: i32,
}

impl Number {
    fn neighbors(self) -> impl Iterator<Item = (i32, i32)> {
        ((self.line_index - 1)..=(self.line_index + 1))
            .cartesian_product((self.start - 1)..=(self.end + 1))
            .filter(|(y, x)| *x >= 0 && *y >= 0)
            .filter(move |(y, x)| *y != self.line_index || !(self.start..=self.end).contains(x))
    }
}

#[derive(Debug)]
struct Char {
    index: (i32, i32),
    char: char,
}

fn convert_input(data: &str) -> (Vec<Number>, Vec<Char>) {
    let re = Regex::new(r"\d+").unwrap();

    let numbers = data
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            re.find_iter(line).map(move |num| Number {
                number: num.as_str().parse::<u32>().unwrap(),
                line_index: i as i32,
                start: num.start() as i32,
                end: num.end() as i32 - 1,
            })
        })
        .collect();

    let chars = data
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars().enumerate().filter_map(move |(j, ch)| {
                if !"0123456789.".contains(ch) {
                    Some(Char {
                        index: (i as i32, j as i32),
                        char: ch,
                    })
                } else {
                    None
                }
            })
        })
        .collect();

    (numbers, chars)
}

#[timed::timed]
fn part1(numbers: &Vec<Number>, chars: &Vec<Char>) -> u32 {
    numbers
        .into_iter()
        .filter(|num| {
            num.neighbors()
                .any(|pos| chars.iter().any(|ch| ch.index == pos))
        })
        .map(|num| num.number)
        .sum()
}

#[timed::timed]
fn part2(numbers: &Vec<Number>, chars: &Vec<Char>) -> u32 {
    let mut asterisks: HashMap<(i32, i32), Vec<u32>> = chars
        .iter()
        .filter(|&v| v.char == '*')
        .map(|v| (v.index, vec![]))
        .collect();

    for number in numbers {
        for n in number.neighbors() {
            if let Some(v) = asterisks.get_mut(&n) {
                v.push(number.number)
            }
        }
    }

    asterisks
        .iter()
        .filter(|(_, v)| v.len() == 2)
        .map(|(_, v)| v[0] * v[1])
        .sum()
}

fn main() {
    let input = read_input(DAY, InputType::NotTest);
    let (numbers, chars) = convert_input(&input.part1);
    println!("Part 1: {}", part1(&numbers, &chars));
    println!("Part 2: {}", part2(&numbers, &chars));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sort_neighbors_key(v: &(i32, i32)) -> i32 {
        v.0 * 10 + v.1
    }

    #[test]
    fn test_neighbors() {
        let number = Number {
            number: 5,
            line_index: 1,
            start: 2,
            end: 3,
        };
        let mut neighbors: Vec<_> = number.neighbors().collect();
        neighbors.sort_by_key(sort_neighbors_key);
        let mut expected = vec![
            (0, 1),
            (0, 2),
            (0, 3),
            (0, 4),
            (2, 1),
            (2, 2),
            (2, 3),
            (2, 4),
            (1, 1),
            (1, 4),
        ];
        expected.sort_by_key(sort_neighbors_key);
        assert_eq!(neighbors, expected);
    }
}
