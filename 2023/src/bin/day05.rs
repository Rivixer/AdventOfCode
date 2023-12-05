use aoc2023::utils::*;
use regex::Regex;

const DAY: u8 = 5;

#[derive(Debug, Clone, Copy)]
struct Map {
    destination: i64,
    source: i64,
    range: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Seed {
    start: i64,
    range: i64,
}

fn parse_input(data: &str) -> (Vec<i64>, Vec<Vec<Map>>) {
    let re = Regex::new(r"\d+").unwrap();

    let seeds: Vec<i64> = re
        .find_iter(data.lines().next().unwrap())
        .flat_map(|v| v.as_str().parse::<i64>())
        .collect();

    let maps: Vec<Vec<Map>> = data
        .split("\r\n\r\n")
        .skip(1)
        .map(|raw_map| {
            raw_map
                .lines()
                .skip(1)
                .map(|line| {
                    re.find_iter(line)
                        .flat_map(|v| v.as_str().parse::<i64>())
                        .collect::<Vec<_>>()
                })
                .map(|numbers| Map {
                    destination: numbers[0],
                    source: numbers[1],
                    range: numbers[2],
                })
                .collect()
        })
        .collect();
    (seeds, maps)
}

fn calculate(seeds: Vec<Seed>, maps: &Vec<Map>) -> Vec<Seed> {
    seeds
        .iter()
        .flat_map(|seed| {
            let mut start = seed.start;
            let mut range = seed.range;
            let mut result = vec![];
            while range > 0 {
                if let Some(map) = maps
                    .iter()
                    .find(|map| map.source <= start && start < map.source + map.range)
                {
                    let len = (map.range - (start - map.source)).min(range);
                    result.push(Seed {
                        start: start - map.source + map.destination,
                        range: len,
                    });
                    start += len;
                    range -= len;
                } else {
                    result.push(Seed { start, range });
                    break;
                }
            }
            result
        })
        .collect()
}

#[timed::timed]
fn part1(data: &str) -> i64 {
    let (seeds, maps) = parse_input(data);

    let mut result: Vec<Seed> = seeds
        .iter()
        .cloned()
        .zip(std::iter::repeat(1))
        .map(|(start, range)| Seed { start, range })
        .collect();

    for map in &maps {
        result = calculate(result, &map);
    }

    result.iter().map(|seed| seed.start).min().unwrap()
}

#[timed::timed]
fn part2(data: &str) -> i64 {
    let (seeds, maps) = parse_input(data);
    let zipped_seeds: Vec<_> = seeds
        .chunks_exact(2)
        .map(|chunk| (chunk[0], chunk[1]))
        .collect();

    let mut result: Vec<Seed> = zipped_seeds
        .iter()
        .cloned()
        .map(|(start, range)| Seed { start, range })
        .collect();

    for map in &maps {
        result = calculate(result, &map);
    }

    result.iter().min().map(|seed| seed.start).unwrap()
}

fn main() {
    let input = read_input(DAY, InputType::NotTest);
    println!("Part 1: {}", part1(&input.part1));
    println!("Part 2: {}", part2(&input.part2));
}
