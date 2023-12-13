use aoc2023::utils::*;
use std::vec;

const DAY: u8 = 10;

#[derive(Debug, PartialEq, Eq)]
enum PipeType {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    StartPos,
}

#[derive(Debug, PartialEq, Eq)]
struct Pipe {
    r#type: PipeType,
    x: usize,
    y: usize,
}

impl Pipe {
    fn new(x: usize, y: usize, char: char) -> Self {
        let r#type = match char {
            '|' => PipeType::Vertical,
            '-' => PipeType::Horizontal,
            'L' => PipeType::NorthEast,
            'J' => PipeType::NorthWest,
            '7' => PipeType::SouthWest,
            'F' => PipeType::SouthEast,
            '.' => PipeType::Ground,
            'S' => PipeType::StartPos,
            _ => panic!("Invalid char: {}", char),
        };
        Pipe { r#type, x, y }
    }
}

struct Grid {
    grid: Vec<Vec<Pipe>>,
}

impl Grid {
    fn new(data: &str) -> Self {
        let grid: Vec<Vec<Pipe>> = data
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, ch)| Pipe::new(x, y, ch))
                    .collect()
            })
            .collect();
        Grid { grid }
    }

    fn get_start(&self) -> &Pipe {
        self.grid
            .iter()
            .flat_map(|line| line.iter())
            .find(|pipe| pipe.r#type == PipeType::StartPos)
            .unwrap()
    }

    fn get_start_and_its_neighbors(&self) -> (&Pipe, Vec<&Pipe>) {
        let mut result = Vec::new();
        let start = self.get_start();

        if let Some(left) = start
            .x
            .checked_sub(1)
            .and_then(|x| self.grid.get(start.y)?.get(x))
        {
            if matches!(
                left.r#type,
                PipeType::Horizontal | PipeType::NorthEast | PipeType::SouthEast
            ) {
                result.push(left);
            }
        }

        if let Some(right) = start
            .x
            .checked_add(1)
            .and_then(|x| self.grid.get(start.y)?.get(x))
        {
            if matches!(
                right.r#type,
                PipeType::Horizontal | PipeType::NorthWest | PipeType::SouthWest
            ) {
                result.push(right);
            }
        }

        if let Some(up) = start
            .y
            .checked_sub(1)
            .and_then(|y| self.grid.get(y)?.get(start.x))
        {
            if matches!(
                up.r#type,
                PipeType::Vertical | PipeType::SouthEast | PipeType::SouthWest
            ) {
                result.push(up);
            }
        }

        if let Some(down) = start
            .y
            .checked_add(1)
            .and_then(|y| self.grid.get(y)?.get(start.x))
        {
            if matches!(
                down.r#type,
                PipeType::Vertical | PipeType::NorthEast | PipeType::NorthWest
            ) {
                result.push(down);
            }
        }

        (start, result)
    }

    fn get_neighbors(&self, pipe: &Pipe) -> Vec<&Pipe> {
        (match pipe.r#type {
            PipeType::Vertical => vec![(pipe.y - 1, pipe.x), (pipe.y + 1, pipe.x)],
            PipeType::Horizontal => vec![(pipe.y, pipe.x - 1), (pipe.y, pipe.x + 1)],
            PipeType::NorthEast => vec![(pipe.y - 1, pipe.x), (pipe.y, pipe.x + 1)],
            PipeType::NorthWest => vec![(pipe.y - 1, pipe.x), (pipe.y, pipe.x - 1)],
            PipeType::SouthWest => vec![(pipe.y + 1, pipe.x), (pipe.y, pipe.x - 1)],
            PipeType::SouthEast => vec![(pipe.y + 1, pipe.x), (pipe.y, pipe.x + 1)],
            _ => unreachable!(),
        })
        .iter()
        .map(|v| &self.grid[v.0][v.1])
        .collect()
    }

    fn enclosed_tiles(&self, lp: &Vec<&Pipe>) -> usize {
        let (start, neighbors) = self.get_start_and_its_neighbors();
        let consider_start = neighbors.iter().any(|v| v.y == start.y + 1);

        self.grid
            .iter()
            .map(|line| {
                let mut flag = false;
                line.iter()
                    .filter(|pipe| {
                        flag ^= (matches!(
                            pipe.r#type,
                            PipeType::Vertical | PipeType::SouthWest | PipeType::SouthEast
                        ) || (pipe.r#type == PipeType::StartPos && consider_start))
                            && lp.contains(&pipe);
                        flag && !lp.contains(&pipe)
                    })
                    .count()
            })
            .sum()
    }
}

fn calculate(grid: &Grid) -> Vec<&Pipe> {
    let (start, start_neighbors) = grid.get_start_and_its_neighbors();
    let stop_pipe = start_neighbors[1];

    let mut visited = vec![start];
    let mut current = start_neighbors[0];

    while current != stop_pipe {
        visited.push(current);
        let neighbors = grid.get_neighbors(current);
        current = neighbors.iter().find(|&x| !visited.contains(x)).unwrap();
    }
    visited.push(stop_pipe);
    visited
}

#[timed::timed]
fn part1(data: &str) -> usize {
    let grid = Grid::new(data);
    calculate(&grid).len() / 2
}

#[timed::timed]
fn part2(data: &str) -> usize {
    let grid = Grid::new(data);
    let lp = calculate(&grid);
    grid.enclosed_tiles(&lp)
}

fn main() {
    let input = read_input(DAY, InputType::NotTest);
    println!("Part 1: {}", part1(&input.part1));
    println!("Part 2: {}", part2(&input.part2));
}
