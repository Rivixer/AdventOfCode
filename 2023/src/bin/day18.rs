use aoc2023::utils::*;
use regex::Regex;

const DAY: u8 = 18;

enum Part {
    One,
    Two,
}

struct Instruction {
    direction: char,
    steps: i64,
}

impl Instruction {
    fn new(direction: char, steps: i64) -> Self {
        Instruction { direction, steps }
    }

    fn direction_delta(&self) -> (i64, i64) {
        match self.direction {
            'R' | '0' => (1, 0),
            'D' | '1' => (0, 1),
            'L' | '2' => (-1, 0),
            'U' | '3' => (0, -1),
            _ => panic!(),
        }
    }
}

fn parse_input(data: &str, part: Part) -> Vec<Instruction> {
    let re = Regex::new(r"([URDL])\s(\d+)\s\(#([0-9a-f]{6})\)").unwrap();
    data.lines()
        .map(|line| {
            let captures = re.captures(line).unwrap();

            match part {
                Part::One => parse_instruction_part_one(&captures),
                Part::Two => parse_instruction_part_two(&captures),
            }
        })
        .collect()
}

fn parse_instruction_part_one(captures: &regex::Captures) -> Instruction {
    let direction = captures.get(1).unwrap().as_str().chars().last().unwrap();
    let steps = captures.get(2).unwrap().as_str().parse::<i64>().unwrap();
    Instruction::new(direction, steps)
}

fn parse_instruction_part_two(captures: &regex::Captures) -> Instruction {
    let hex_color = captures.get(3).unwrap().as_str();
    let steps = i64::from_str_radix(&hex_color[..5], 16).unwrap();
    let direction = hex_color.chars().last().unwrap();
    Instruction::new(direction, steps)
}

fn calculate(instructions: &[Instruction]) -> i64 {
    let points: Vec<(i64, i64)> = instructions
        .iter()
        .scan((0, 0), |(cur_x, cur_y), instruction| {
            let (dx, dy) = instruction.direction_delta();
            *cur_x += dx * instruction.steps;
            *cur_y += dy * instruction.steps;
            Some((*cur_x, *cur_y))
        })
        .collect();

    let area = points
        .windows(2)
        .map(|v| v[0].0 * v[1].1 - v[1].0 * v[0].1)
        .sum::<i64>()
        / 2;

    let parimeter: i64 = instructions.iter().map(|v| v.steps).sum();

    area + parimeter / 2 + 1
}

#[timed::timed]
fn part1(data: &str) -> i64 {
    let instructions = parse_input(data, Part::One);
    calculate(&instructions)
}

#[timed::timed]
fn part2(data: &str) -> i64 {
    let instructions = parse_input(data, Part::Two);
    calculate(&instructions)
}

fn main() {
    let input = read_input(DAY, InputType::NotTest);
    println!("Part 1: {}", part1(&input.part1));
    println!("Part 2: {}", part2(&input.part2));
}
