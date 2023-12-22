use aoc2023::utils::*;
use std::collections::{HashMap, HashSet};

const DAY: u8 = 21;

type Point = (isize, isize);
type Grid = HashMap<Point, char>;

fn initialize_grid(data: &str) -> Grid {
    data.lines()
        .enumerate()
        .flat_map(|(i, row)| {
            row.chars().enumerate().filter_map(move |(j, cell)| {
                if cell == '.' || cell == 'S' {
                    Some(((i as isize, j as isize), cell))
                } else {
                    None
                }
            })
        })
        .collect()
}

fn update_current(current: &mut HashSet<Point>, grid: &Grid, n: usize) {
    let new: HashSet<_> = current
        .iter()
        .flat_map(|&p| {
            vec![
                (p.0 + 1, p.1),
                (p.0 - 1, p.1),
                (p.0, p.1 + 1),
                (p.0, p.1 - 1),
            ]
            .into_iter()
            .map(|p| (p.0, p.1))
        })
        .filter(|&p| grid.contains_key(&cmod(p, n)))
        .clone()
        .collect();
    current.clear();
    current.extend(new);
}

fn cmod(x: Point, n: usize) -> Point {
    (
        ((x.0 as usize % n + n) % n) as isize,
        ((x.1 as usize % n + n) % n) as isize,
    )
}

fn find_start(grid: &Grid) -> Point {
    *grid.iter().find(|v| v.1 == &'S').unwrap().0
}

#[timed::timed]
fn part1(data: &str) -> usize {
    let grid = initialize_grid(&data);
    let mut queue = HashSet::new();
    queue.insert(find_start(&grid));

    (0..64).for_each(|_| {
        update_current(&mut queue, &grid, data.lines().count());
    });

    queue.len()
}

#[timed::timed]
fn part2(data: &str) -> usize {
    let grid = initialize_grid(&data);
    let mut results = vec![];

    let mut queue = HashSet::new();
    queue.insert(find_start(&grid));

    let n = data.lines().count();
    let mut i = n;
    while results.len() < 3 {
        if (i - 65) % n == 0 {
            results.push(queue.len());
        }
        update_current(&mut queue, &grid, n);
        i += 1;
    }

    let x = 26501365 / n;
    results[0]
        + x * (results[1] - results[0] + (x - 1) * (results[2] - results[1] * 2 + results[0]) / 2)
}

fn main() {
    let input = read_input(DAY, InputType::NotTest);
    println!("Part 1: {}", part1(&input.part1));
    println!("Part 2: {}", part2(&input.part2));
}
