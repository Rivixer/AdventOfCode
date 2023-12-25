use aoc2023::utils::*;
use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};

const DAY: u8 = 25;

fn parse_input(data: &str) -> HashMap<&str, HashSet<&str>> {
    let re = Regex::new(r"\w+").unwrap();

    let mut connections: HashMap<&str, HashSet<&str>> = data
        .lines()
        .map(|v| {
            let mut result = re.find_iter(v);
            (
                result.next().unwrap().as_str(),
                result.map(|v| v.as_str()).collect(),
            )
        })
        .collect();

    connections.clone().iter().for_each(|(key, values)| {
        values.iter().for_each(|v| {
            connections
                .entry(v)
                .or_insert_with(HashSet::new)
                .insert(key);
        })
    });

    connections
}

#[timed::timed]
fn part1(data: &str) -> usize {
    let connections = parse_input(data);

    let mut connection_count = HashMap::new();
    connections.keys().for_each(|&from| {
        let mut queue: VecDeque<&str> = [from].into();
        let mut visited: HashSet<&str> = [from].into();

        while let Some(current) = queue.pop_front() {
            connections.get(current).unwrap().iter().for_each(|next| {
                if !visited.insert(next) {
                    return;
                }

                let mut key = [next, current];
                key.sort();

                *connection_count.entry(key).or_insert(0) += 1;

                queue.push_back(next);
            });
        }
    });

    let mut connection_count = connection_count.iter().collect_vec();
    connection_count.sort_by_key(|&(_, count)| count);
    connection_count.reverse();

    let to_cut_count = 3;
    let with_most_connections = connection_count
        .iter()
        .take(to_cut_count)
        .map(|&v| v.0)
        .collect_vec();

    let left_connections: HashMap<&str, HashSet<&str>> = connections
        .iter()
        .map(|(&key, values)| {
            let filtered_values: HashSet<&str> = values
                .iter()
                .filter(|&&v| {
                    let mut k = [key, v];
                    k.sort();
                    !with_most_connections.contains(&&k)
                })
                .cloned()
                .collect();
            (key, filtered_values)
        })
        .collect();

    let start = *left_connections.keys().next().unwrap();
    let mut queue: VecDeque<&str> = [start].into();
    let mut visited: HashSet<&str> = [start].into();

    while let Some(current) = queue.pop_front() {
        left_connections
            .get(current)
            .unwrap()
            .iter()
            .for_each(|next| {
                if visited.insert(next) {
                    queue.push_back(next);
                }
            });
    }

    visited.len() * (connections.len() - visited.len())
}

fn main() {
    let input = read_input(DAY, InputType::NotTest);
    println!("Part 1: {}", part1(&input.part1));
}
