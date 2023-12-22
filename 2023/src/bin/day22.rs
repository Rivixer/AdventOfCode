use aoc2023::utils::*;
use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet};

const DAY: u8 = 22;

#[derive(Hash, PartialEq, Eq, Clone)]
struct Point {
    x: usize,
    y: usize,
    z: usize,
}

impl Point {
    fn new(x: usize, y: usize, z: usize) -> Self {
        Point { x, y, z }
    }
}

#[derive(Hash, PartialEq, Eq)]
struct Brick {
    start: Point,
    end: Point,
}

impl Brick {
    fn new(start: Point, end: Point) -> Self {
        Brick { start, end }
    }
}

fn parse_input(data: &str) -> Vec<Brick> {
    let re = Regex::new(r"-?\d+").unwrap();
    data.lines()
        .map(|line| {
            let digits: Vec<usize> = re
                .find_iter(line)
                .map(|m| m.as_str().parse().unwrap())
                .collect();
            Brick::new(
                Point::new(digits[0], digits[1], digits[2]),
                Point::new(digits[3], digits[4], digits[5]),
            )
        })
        .collect_vec()
}

fn dependency_depth(brick: &Brick, children_list: &HashMap<&Brick, HashSet<&Brick>>) -> i32 {
    let mut in_degree_counts: HashMap<&Brick, i32> = children_list
        .iter()
        .flat_map(|(_, children)| children)
        .fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c).or_insert(0) += 1;
            acc
        });

    let mut depth = -1;
    let mut queue = vec![brick];
    while let Some(current) = queue.pop() {
        depth += 1;
        if let Some(children) = children_list.get(&current) {
            children.iter().for_each(|&child| {
                if let Some(count) = in_degree_counts.get_mut(&child) {
                    *count -= 1;
                    if *count == 0 {
                        queue.push(child);
                    }
                }
            });
        }
    }

    depth
}

#[timed::timed]
fn calculate(data: &str) -> (usize, i32) {
    let mut bricks = parse_input(data);
    bricks.sort_by_key(|b| b.start.z);

    let mut parent: HashMap<&Brick, HashSet<&Brick>> = HashMap::new();
    let mut children: HashMap<&Brick, HashSet<&Brick>> = HashMap::new();
    let mut max_z: HashMap<(usize, usize), (usize, &Brick)> = HashMap::new();
    let max_point = Point::new(usize::MAX, usize::MAX, usize::MAX);
    let weird_brick = Brick::new(max_point.clone(), max_point);

    bricks.iter().for_each(|brick| {
        let grid2d = (brick.start.x..=brick.end.x).cartesian_product(brick.start.y..=brick.end.y);
        let current_z = grid2d
            .clone()
            .map(|(x, y)| max_z.entry((x, y)).or_insert((0, &weird_brick)).0)
            .max()
            .unwrap();
        grid2d.for_each(|(x, y)| {
            let current_max_z = max_z.get_mut(&(x, y)).unwrap();
            let (last_z, last_id) = current_max_z.to_owned();
            if last_z == current_z {
                parent
                    .entry(brick)
                    .or_insert_with(HashSet::new)
                    .insert(last_id);
                children
                    .entry(last_id)
                    .or_insert_with(HashSet::new)
                    .insert(brick);
            }
            *current_max_z = (current_z + brick.end.z - brick.start.z + 1, brick);
        });
    });

    let single_supported: HashSet<&Brick> = parent
        .iter()
        .filter(|(_, v)| v.len() == 1 && !v.contains(&weird_brick))
        .flat_map(|(_, v)| v.iter().cloned())
        .collect();

    let result1 = bricks.len() - single_supported.len();
    let result2 = single_supported
        .iter()
        .map(|&v| dependency_depth(v, &children))
        .sum();

    (result1, result2)
}

fn main() {
    let input = read_input(DAY, InputType::NotTest);
    let result = calculate(&input.part1);
    println!("Part 1: {}", result.0);
    println!("Part 2: {}", result.1);
}
