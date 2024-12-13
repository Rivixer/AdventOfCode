use aoc2024::utils::*;
use std::{collections::HashSet, vec};

const DAY: u8 = 12;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Side {
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(PartialEq, Eq, Hash)]
struct Fence {
    x: isize,
    y: isize,
    side: Side,
}

fn parse_input(data: &str) -> Vec<Vec<char>> {
    data.lines().map(|l| l.chars().collect()).collect()
}

fn floodfill(
    grid: &[Vec<char>],
    x: isize,
    y: isize,
    c: char,
    visited: &mut Vec<(isize, isize)>,
    fences: &mut Vec<Fence>,
) {
    let height = grid.len() as isize;
    let width = grid[0].len() as isize;

    if x < 0 || y < 0 || x >= width || y >= height {
        return;
    }

    if c != grid[y as usize][x as usize] {
        return;
    }

    if visited.contains(&(x, y)) {
        return;
    }

    visited.push((x, y));

    let neighbors = [
        ((x - 1, y), Side::Left),
        ((x + 1, y), Side::Right),
        ((x, y - 1), Side::Top),
        ((x, y + 1), Side::Bottom),
    ];

    for &(neighbor, ref fence_type) in &neighbors {
        if !visited.contains(&neighbor) {
            if neighbor.0 < 0 || neighbor.1 < 0 || neighbor.0 >= width || neighbor.1 >= height {
                fences.push(Fence {
                    x: neighbor.0,
                    y: neighbor.1,
                    side: *fence_type,
                });
            } else if grid[neighbor.1 as usize][neighbor.0 as usize] != c {
                fences.push(Fence {
                    x: neighbor.0,
                    y: neighbor.1,
                    side: *fence_type,
                });
            } else {
                floodfill(grid, neighbor.0, neighbor.1, c, visited, fences);
            }
        }
    }
}

fn connect_fences(fences: &[Fence]) -> Vec<Vec<&Fence>> {
    let mut connected = vec![];
    let mut visited: HashSet<&Fence> = HashSet::new();

    for f in fences.iter() {
        if visited.contains(f) {
            continue;
        }

        let mut connected_fences = vec![f];
        visited.insert(f);

        let directions = match f.side {
            Side::Top | Side::Bottom => vec![(-1, 0), (1, 0)],
            Side::Left | Side::Right => vec![(0, -1), (0, 1)],
        };

        for &dir in &directions {
            let mut current = f;
            while let Some(neighbor) = fences.iter().find(|&neighbor| {
                neighbor.x == current.x + dir.0
                    && neighbor.y == current.y + dir.1
                    && neighbor.side == current.side
            }) {
                if !visited.contains(neighbor) {
                    connected_fences.push(neighbor);
                    visited.insert(neighbor);
                    current = neighbor;
                } else {
                    break;
                }
            }
        }

        connected.push(connected_fences);
    }

    connected
}

#[timed::timed]
fn part1(data: &str) -> u64 {
    let grid = parse_input(data);
    let mut visited: HashSet<(isize, isize)> = HashSet::new();
    let mut result = 0;

    for (i, row) in grid.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            let pos = (j as isize, i as isize);

            if visited.contains(&pos) {
                continue;
            }

            let mut field = vec![];
            let mut fences = vec![];

            floodfill(&grid, pos.0, pos.1, c, &mut field, &mut fences);
            visited.extend(field.iter().copied());
            result += field.len() as u64 * fences.len() as u64;
        }
    }

    result
}

#[timed::timed]
fn part2(data: &str) -> u64 {
    let grid = parse_input(data);
    let mut visited: HashSet<(isize, isize)> = HashSet::new();
    let mut result = 0;

    for (i, row) in grid.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            let pos = (j as isize, i as isize);

            if visited.contains(&pos) {
                continue;
            }

            let mut field = vec![];
            let mut fences = vec![];

            floodfill(&grid, pos.0, pos.1, c, &mut field, &mut fences);
            visited.extend(field.iter().copied());

            let connected_fences = connect_fences(&fences);
            result += field.len() as u64 * connected_fences.len() as u64;
        }
    }

    result
}

fn main() {
    let input = read_input(DAY, InputType::Production);
    println!("Part 1: {}", part1(&input.part1));
    println!("Part 2: {}", part2(&input.part2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&get_test_input().part1), 1930);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&get_test_input().part2), 1206);
    }

    fn get_test_input() -> Input {
        read_input(DAY, InputType::Test)
    }
}
