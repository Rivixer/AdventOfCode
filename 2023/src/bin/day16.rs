use aoc2023::utils::*;
use std::collections::HashSet;

const DAY: u8 = 16;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl Direction {
    fn is_horizontal(&self) -> bool {
        matches!(self, Direction::Left | Direction::Right)
    }

    fn is_vertical(&self) -> bool {
        matches!(self, Direction::Up | Direction::Down)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Beam {
    x: usize,
    y: usize,
    direction: Direction,
    edge_encountered: bool,
}

impl Beam {
    fn new(x: usize, y: usize, direction: Direction) -> Self {
        Beam {
            x,
            y,
            direction,
            edge_encountered: false,
        }
    }

    fn move_forward(&mut self, grid: &Grid) {
        let (dx, dy) = match self.direction {
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Up => (0, -1),
        };

        let new_x = self.x.checked_add_signed(dx).unwrap_or_else(|| {
            self.edge_encountered = true;
            self.x
        });

        let new_y = self.y.checked_add_signed(dy).unwrap_or_else(|| {
            self.edge_encountered = true;
            self.y
        });

        if new_x >= grid.grid[0].len() || new_y >= grid.grid.len() {
            self.edge_encountered = true;
        } else {
            self.x = new_x;
            self.y = new_y;
        }
    }

    fn rotate(&mut self, clockwise: bool) {
        self.direction = match (self.direction, clockwise) {
            (Direction::Right, true) | (Direction::Left, false) => Direction::Down,
            (Direction::Down, true) | (Direction::Up, false) => Direction::Left,
            (Direction::Left, true) | (Direction::Right, false) => Direction::Up,
            (Direction::Up, true) | (Direction::Down, false) => Direction::Right,
        };
    }

    fn move_left(&mut self, grid: &Grid) {
        self.rotate(false);
        self.move_forward(grid);
    }

    fn move_right(&mut self, grid: &Grid) {
        self.rotate(true);
        self.move_forward(grid);
    }
}

struct Grid {
    grid: Vec<Vec<char>>,
}

impl Grid {
    fn new(data: &str) -> Self {
        Grid {
            grid: data.lines().map(|line| line.chars().collect()).collect(),
        }
    }

    fn get_energized_tiles(&self, beam: Beam) -> usize {
        let mut beams = vec![beam];
        let mut visited: HashSet<Beam> = HashSet::new();
        let mut energized: HashSet<(usize, usize)> = HashSet::new();

        while let Some(mut beam) = beams.pop() {
            visited.insert(beam);
            energized.insert((beam.x, beam.y));
            let next_beam = self.go_next(&mut beam);
            if !beam.edge_encountered && !visited.contains(&beam) {
                beams.push(beam)
            }
            if let Some(b) = next_beam {
                if !b.edge_encountered && !visited.contains(&b) {
                    beams.push(b);
                }
            }
        }

        energized.len()
    }

    fn max_energized_row(&self, start: (usize, usize), direction: Direction) -> usize {
        (0..self.grid.len())
            .map(|i| self.get_energized_tiles(Beam::new(start.0, i, direction)))
            .max()
            .unwrap_or(0)
    }

    fn go_next(&self, beam: &mut Beam) -> Option<Beam> {
        let tile = self.grid[beam.y][beam.x];

        match tile {
            '.' => {
                beam.move_forward(self);
                None
            }
            '/' if beam.direction.is_horizontal() => {
                beam.move_left(self);
                None
            }
            '/' => {
                beam.move_right(self);
                None
            }
            '\\' if beam.direction.is_horizontal() => {
                beam.move_right(self);
                None
            }
            '\\' => {
                beam.move_left(self);
                None
            }
            '|' if beam.direction.is_vertical() => {
                beam.move_forward(self);
                None
            }
            '-' if beam.direction.is_horizontal() => {
                beam.move_forward(self);
                None
            }
            '|' | '-' => {
                let mut new_beam = Beam::new(beam.x, beam.y, beam.direction);
                new_beam.move_left(self);
                beam.move_right(self);
                Some(new_beam)
            }
            _ => unreachable!(),
        }
    }
}

#[timed::timed]
fn part1(data: &str) -> usize {
    let grid = Grid::new(data);
    grid.get_energized_tiles(Beam::new(0, 0, Direction::Right))
}

#[timed::timed]
fn part2(data: &str) -> usize {
    let grid = Grid::new(data);
    let top_row = grid.max_energized_row((0, 0), Direction::Down);
    let right_row = grid.max_energized_row((grid.grid[0].len() - 1, 0), Direction::Left);
    let bottom_row = grid.max_energized_row((0, grid.grid.len() - 1), Direction::Up);
    let left_row = grid.max_energized_row((0, 0), Direction::Right);
    [top_row, right_row, bottom_row, left_row]
        .iter()
        .copied()
        .max()
        .unwrap()
}

fn main() {
    let input = read_input(DAY, InputType::NotTest);
    println!("Part 1: {}", part1(&input.part1));
    println!("Part 2: {}", part2(&input.part2));
}
