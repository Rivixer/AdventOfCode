use aoc2023::utils::*;
use std::collections::HashSet;

const DAY: u8 = 11;

#[derive(Clone)]
struct Grid {
    grid: Vec<Vec<char>>,
    max_x: usize,
    max_y: usize,
}

impl Grid {
    fn new(data: &str) -> Self {
        let grid: Vec<Vec<char>> = data.lines().map(|v| v.chars().collect()).collect();
        Grid {
            grid: grid.clone(),
            max_x: grid[0].len(),
            max_y: grid.len(),
        }
    }

    fn empty_cols(&self) -> HashSet<usize> {
        (0..self.max_x)
            .filter(|&x| self.grid.iter().all(|row| row[x] == '.'))
            .collect()
    }

    fn empty_rows(&self) -> HashSet<usize> {
        (0..self.max_y)
            .filter(|&y| self.grid[y].iter().all(|&cell| cell == '.'))
            .collect()
    }

    fn galaxies(&self) -> Vec<(usize, usize)> {
        (0..self.max_y)
            .flat_map(|y| {
                (0..self.max_x)
                    .filter(move |&x| self.grid[y][x] == '#')
                    .map(move |x| (y, x))
            })
            .collect()
    }

    fn calculate(&self, factor: usize) -> usize {
        let empty_cols = self.empty_cols();
        let empty_rows = self.empty_rows();
        let galaxies = self.galaxies();

        galaxies
            .iter()
            .enumerate()
            .flat_map(|(i, &(y1, x1))| {
                let empty_cols_clone = empty_cols.clone();
                let empty_rows_clone = empty_rows.clone();

                galaxies.iter().skip(i + 1).map(move |&(y2, x2)| {
                    let (x1, x2) = (x1.min(x2), x1.max(x2));
                    let (y1, y2) = (y1.min(y2), y1.max(y2));

                    let col_intersection = empty_cols_clone
                        .intersection(&(x1..=x2).collect::<HashSet<_>>())
                        .count()
                        * factor;

                    let row_intersection = empty_rows_clone
                        .intersection(&(y1..=y2).collect::<HashSet<_>>())
                        .count()
                        * factor;

                    x2 - x1 + col_intersection + y2 - y1 + row_intersection
                })
            })
            .sum()
    }
}

#[timed::timed]
fn part1(data: &str) -> usize {
    let grid = Grid::new(data);
    grid.calculate(1)
}

#[timed::timed]
fn part2(data: &str) -> usize {
    let grid = Grid::new(data);
    grid.calculate(1_000_000 - 1)
}

fn main() {
    let input = read_input(DAY, InputType::Test);
    println!("Part 1: {}", part1(&input.part1));
    println!("Part 2: {}", part2(&input.part2));
}
