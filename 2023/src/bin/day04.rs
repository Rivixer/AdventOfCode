use aoc2023::utils::*;
use regex::Regex;
use std::collections::HashSet;

const DAY: u8 = 4;

#[derive(Clone)]
struct LineData {
    win_numbers: HashSet<u32>,
    my_numbers: HashSet<u32>,
}

impl LineData {
    fn get_pairs(self) -> Vec<u32> {
        self.win_numbers
            .intersection(&self.my_numbers)
            .cloned()
            .collect()
    }
}

fn parse_input(data: &str) -> Vec<LineData> {
    let re = Regex::new(r"\d+").unwrap();
    data.lines()
        .map(|line| {
            let mut parts = line.split("|");
            let win_numbers = re
                .find_iter(parts.next().unwrap())
                .flat_map(|v| v.as_str().parse::<u32>())
                .skip(1)
                .collect();
            let my_numbers = re
                .find_iter(parts.next().unwrap())
                .flat_map(|v| v.as_str().parse::<u32>())
                .collect();
            LineData {
                win_numbers,
                my_numbers,
            }
        })
        .collect()
}

#[timed::timed]
fn part1(data: &str) -> u32 {
    let inputs = parse_input(data);
    inputs
        .iter()
        .map(|input| input.clone().get_pairs().len())
        .filter(|&v| v > 0)
        .map(|v| 2_u32.pow(v as u32 - 1))
        .sum()
}

#[timed::timed]
fn part2(data: &str) -> i32 {
    let inputs = parse_input(data);
    let mut mem = vec![1; inputs.len()];
    inputs.iter().enumerate().for_each(|(i, input)| {
        let n = input.clone().get_pairs().len();
        ((i + 1)..(i + n + 1)).for_each(|j| {
            mem[j] += mem[i];
        });
    });
    mem.iter().sum::<i32>()
}

fn main() {
    let input = read_input(DAY, InputType::NotTest);
    println!("Part 1: {}", part1(&input.part1));
    println!("Part 2: {}", part2(&input.part2));
}
