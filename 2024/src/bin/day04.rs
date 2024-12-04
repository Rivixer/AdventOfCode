use aoc2024::utils::*;
use std::vec;

const DAY: u8 = 4;

fn parse_input(data: &str) -> Vec<Vec<char>> {
    data.lines().map(|l| l.chars().collect()).collect()
}

fn is_in_bounds(x: isize, y: isize, size: usize) -> bool {
    x >= 0 && x < size as isize && y >= 0 && y < size as isize
}

#[timed::timed]
fn part1(data: &str) -> usize {
    let grid = parse_input(data);
    let size = grid.len();
    let pattern = vec!['X', 'M', 'A', 'S'];

    let directions = vec![
        (1, 0),
        (-1, 0),
        (0, 1),
        (0, -1),
        (1, 1),
        (-1, -1),
        (1, -1),
        (-1, 1),
    ];

    (0..size * size)
        .map(|i| (i % size, i / size))
        .flat_map(|(x, y)| directions.iter().map(move |&(dx, dy)| (x, y, dx, dy)))
        .filter(|&(x, y, dx, dy)| {
            (0..4).all(|i| {
                let nx = x as isize + dx * i;
                let ny = y as isize + dy * i;
                is_in_bounds(nx, ny, size) && grid[ny as usize][nx as usize] == pattern[i as usize]
            })
        })
        .count()
}

#[timed::timed]
fn part2(data: &str) -> usize {
    let grid = parse_input(data);
    let size = grid.len();

    (1..size - 1)
        .flat_map(|i| (1..size - 1).map(move |j| (i, j)))
        .filter(|&(i, j)| {
            grid[i][j] == 'A'
                && ((grid[i - 1][j - 1] == 'M' && grid[i + 1][j + 1] == 'S')
                    || (grid[i - 1][j - 1] == 'S' && grid[i + 1][j + 1] == 'M'))
                && ((grid[i - 1][j + 1] == 'M' && grid[i + 1][j - 1] == 'S')
                    || (grid[i - 1][j + 1] == 'S' && grid[i + 1][j - 1] == 'M'))
        })
        .count()
}

fn main() {
    let input = read_input(DAY, InputType::Production);
    println!("Part 1: {}", part1(&input.part1));
    println!("Part 2: {}", part2(&input.part2));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_input() -> Input {
        read_input(DAY, InputType::Test)
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&get_test_input().part1), 18);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&get_test_input().part2), 9);
    }
}
