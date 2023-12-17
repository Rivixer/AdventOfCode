use aoc2023::utils::*;
use std::collections::{BinaryHeap, HashMap, HashSet};

const DAY: u8 = 17;
const DIRECTIONS: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
struct Block {
    x: i32,
    y: i32,
    direction_index: usize,
}

impl Block {
    fn new(x: i32, y: i32, direction_index: usize) -> Self {
        Block {
            x,
            y,
            direction_index,
        }
    }

    fn r#move(&self, direction: &(i32, i32), steps: usize) -> Self {
        let mut next = self.clone();
        next.x += direction.0 * steps as i32;
        next.y += direction.1 * steps as i32;
        next
    }

    fn update_direction(&mut self, direction_index: usize) {
        self.direction_index = direction_index
    }
}

struct Grid(Vec<Vec<char>>);

impl Grid {
    fn new(data: &str) -> Self {
        Grid(data.lines().map(|v| v.chars().collect()).collect())
    }

    fn in_border(&self, block: &Block) -> bool {
        0 <= block.x
            && 0 <= block.y
            && block.x < self.0[0].len() as i32
            && block.y < self.0.len() as i32
    }

    fn get_cost(&self, block: &Block) -> i32 {
        self.0[block.y as usize][block.x as usize]
            .to_digit(10)
            .unwrap() as i32
    }

    fn at_goal(&self, block: &Block) -> bool {
        block.x == self.0[0].len() as i32 - 1 && block.y == self.0.len() as i32 - 1
    }
}

fn calculate(grid: &Grid, minsteps: usize, maxsteps: usize) -> i32 {
    let mut queue: BinaryHeap<(i32, Block)> =
        [(0, Block::new(0, 0, 0)), (0, Block::new(0, 0, 1))].into();

    let mut visited = HashSet::new();
    let mut costs = HashMap::new();

    while let Some((cost, block)) = queue.pop() {
        let cost = -cost;

        if grid.at_goal(&block) {
            return cost;
        }

        visited.insert(block.clone());

        DIRECTIONS
            .iter()
            .enumerate()
            .filter(|(i, _)| i % 2 != block.direction_index % 2)
            .for_each(|(i, direction)| {
                let mut next_cost = cost;
                for steps in 1..=maxsteps {
                    let mut next = block.r#move(direction, steps);
                    if grid.in_border(&next) {
                        next_cost += grid.get_cost(&next);
                        next.update_direction(i);
                        if steps >= minsteps
                            && next_cost < *costs.get(&next).unwrap_or(&i32::MAX)
                            && !visited.contains(&next)
                        {
                            costs.insert(next.clone(), next_cost);
                            queue.push((-next_cost, next.clone()));
                        }
                    }
                }
            });
    }
    unreachable!()
}

#[timed::timed]
fn part1(data: &str) -> i32 {
    let grid = Grid::new(data);
    calculate(&grid, 1, 3)
}

#[timed::timed]
fn part2(data: &str) -> i32 {
    let grid = Grid::new(data);
    calculate(&grid, 4, 10)
}

fn main() {
    let input = read_input(DAY, InputType::NotTest);
    println!("Part 1: {}", part1(&input.part1));
    println!("Part 2: {}", part2(&input.part2));
}
