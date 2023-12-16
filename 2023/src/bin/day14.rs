use aoc2023::utils::*;
use itertools::Itertools;
use std::collections::HashMap;

const DAY: u8 = 14;

#[derive(Clone, Hash, PartialEq, Eq)]
struct Grid {
    grid: Vec<Vec<char>>,
}

impl Grid {
    fn new(data: &str) -> Self {
        Grid {
            grid: data.lines().map(|line| line.chars().collect()).collect(),
        }
    }

    fn shift_up(&mut self) {
        (0..self.grid[0].len()).for_each(|x| {
            let mut destination: Option<usize> = None;
            (0..self.grid.len()).for_each(|y| match self.grid[y][x] {
                'O' if destination.is_some() => {
                    self.grid[y][x] = '.';
                    self.grid[destination.unwrap()][x] = 'O';
                    destination = Some(destination.unwrap() + 1);
                }
                '.' if destination.is_none() => destination = Some(y),
                '#' => destination = None,
                _ => (),
            })
        })
    }

    fn rotate_right(&mut self) {
        let rows = self.grid.len();
        let cols = self.grid[0].len();

        let transposed: Vec<Vec<_>> = (0..cols)
            .map(|j| (0..rows).map(|i| self.grid[i][j]).collect())
            .collect();

        self.grid = transposed
            .into_iter()
            .map(|mut row| {
                row.reverse();
                row
            })
            .collect();
    }

    fn score(&self) -> usize {
        self.grid
            .iter()
            .enumerate()
            .map(|(y, row)| row.iter().filter(|&&v| v == 'O').count() * (self.grid.len() - y))
            .sum()
    }
}

#[timed::timed]
fn part1(data: &str) -> usize {
    let mut grid = Grid::new(data);
    grid.shift_up();
    grid.score()
}

#[timed::timed]
fn part2(data: &str) -> usize {
    let mut grid = Grid::new(data);
    let mut grid_cache: HashMap<Grid, usize> = HashMap::new();
    let mut score_cache: HashMap<usize, usize> = HashMap::new();
    let mut result: Option<usize> = None;
    let n = 1_000_000_000;

    for i in 1..n {
        for _ in 0..4 {
            grid.shift_up();
            grid.rotate_right();
        }
        if grid_cache.keys().contains(&grid) {
            let start = grid_cache[&grid];
            let length = i - start;
            result = Some(score_cache[&(start + (n - start) % length)]);
            break;
        }
        score_cache.insert(i, grid.score());
        grid_cache.insert(grid.clone(), i);
    }
    result.unwrap()
}

fn main() {
    let input = read_input(DAY, InputType::NotTest);
    println!("Part 1: {}", part1(&input.part1));
    println!("Part 2: {}", part2(&input.part2));
}
