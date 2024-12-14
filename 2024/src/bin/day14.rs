use aoc2024::utils::*;
use std::collections::{HashSet, VecDeque};

const DAY: u8 = 14;

#[derive(Hash)]
struct Vector {
    x: i32,
    y: i32,
}

#[derive(Hash)]
struct Robot {
    position: Vector,
    velocity: Vector,
}

fn parse_input(data: &str) -> Vec<Robot> {
    let re = regex::Regex::new(r"p=(-?\d+),(-?\d+)\sv=(-?\d+),(-?\d+)").unwrap();
    data.lines()
        .map(|line| {
            let caps = re.captures(line).unwrap();
            Robot {
                position: Vector {
                    x: caps[1].parse().unwrap(),
                    y: caps[2].parse().unwrap(),
                },
                velocity: Vector {
                    x: caps[3].parse().unwrap(),
                    y: caps[4].parse().unwrap(),
                },
            }
        })
        .collect()
}

fn group_robots_by_quadrant(robots: &[Robot], width: i32, height: i32) -> Vec<Vec<&Robot>> {
    let mut quadrants = vec![vec![]; 4];
    let center_x = width / 2;
    let center_y = height / 2;

    for robot in robots {
        if robot.position.x == center_x || robot.position.y == center_y {
            continue;
        }

        let i = match (robot.position.x > center_x, robot.position.y > center_y) {
            (false, false) => 0,
            (true, false) => 1,
            (false, true) => 2,
            (true, true) => 3,
        };
        quadrants[i].push(robot);
    }

    quadrants
}

fn simulate(robots: &mut Vec<Robot>, width: i32, height: i32, seconds: i32) {
    for robot in robots {
        robot.position.x += robot.velocity.x * seconds;
        robot.position.y += robot.velocity.y * seconds;
        robot.position.x = robot.position.x.rem_euclid(width);
        robot.position.y = robot.position.y.rem_euclid(height);
    }
}

#[timed::timed]
fn part1(data: &str, width: i32, height: i32) -> u64 {
    let mut robots = parse_input(data);
    simulate(&mut robots, width, height, 100);

    let quadrant_groups = group_robots_by_quadrant(&robots, width, height);
    quadrant_groups
        .iter()
        .map(|group| group.len() as u64)
        .product()
}

fn bfs(
    robot: &Robot,
    robots: &[Robot],
    width: i32,
    height: i32,
    visited: &mut HashSet<(i32, i32)>,
) -> i32 {
    let mut count = 0;
    let mut queue = VecDeque::new();

    const DIRS: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    queue.push_back(robot);

    while let Some(r) = queue.pop_front() {
        let x = r.position.x;
        let y = r.position.y;

        if x < 0 || x >= width || y < 0 || y >= height {
            continue;
        }

        if !visited.insert((r.position.x, r.position.y)) {
            continue;
        }

        count += 1;

        for dir in &DIRS {
            queue.extend(
                robots
                    .iter()
                    .find(|r| r.position.x == x + dir.0 && r.position.y == y + dir.1),
            );
        }
    }

    count
}

fn find_largest_robot_group(robots: &[Robot], width: i32, height: i32) -> i32 {
    let mut max_size = 0;
    let mut visited = HashSet::new();

    for robot in robots {
        if !visited.contains(&(robot.position.x, robot.position.y)) {
            max_size = max_size.max(bfs(robot, robots, width, height, &mut visited));
        }
    }

    max_size
}

#[timed::timed]
fn part2(data: &str, width: i32, height: i32) -> u64 {
    const DETECTION_THRESHOLD: i32 = 50;

    let mut seconds = 0;
    let mut robots = parse_input(data);

    loop {
        simulate(&mut robots, width, height, 1);
        seconds += 1;

        let size = find_largest_robot_group(&robots, width, height);
        if size > DETECTION_THRESHOLD {
            print_grid(&robots, width, height);
            dbg!(size);
            break;
        }
    }

    seconds
}

fn print_grid(robots: &Vec<Robot>, width: i32, height: i32) {
    for i in 0..height {
        for j in 0..width {
            let count = robots
                .iter()
                .filter(|robot| robot.position.x == j && robot.position.y == i)
                .count();

            if count > 0 {
                print!("{}", count);
            } else {
                print!(".");
            }
        }

        println!();
    }
}

fn main() {
    const WIDTH: i32 = 101;
    const HEIGHT: i32 = 103;
    let input = read_input(DAY, InputType::Production);
    println!("Part 1: {}", part1(&input.part1, WIDTH, HEIGHT));
    println!("Part 2: {}", part2(&input.part2, WIDTH, HEIGHT));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&get_test_input().part1, 11, 7), 12);
    }

    fn get_test_input() -> Input {
        read_input(DAY, InputType::Test)
    }
}
