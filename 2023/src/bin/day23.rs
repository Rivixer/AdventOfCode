use aoc2023::utils::*;
use itertools::Itertools;

const DAY: u8 = 23;

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct Point {
    x: i16,
    y: i16,
}

impl Point {
    fn new(x: i16, y: i16) -> Self {
        Point { x, y }
    }
}

#[derive(Clone)]
struct Grid(Vec<Vec<char>>);

impl Grid {
    fn new(data: &str) -> Self {
        Grid(
            data.lines()
                .map(|line| line.chars().collect_vec())
                .collect_vec(),
        )
    }

    fn height(&self) -> i16 {
        self.0.len() as i16
    }

    fn width(&self) -> i16 {
        self.0[0].len() as i16
    }

    fn get_neighbors(&self, point: &Point, avoid_uphill: bool) -> Vec<Point> {
        let mut possible_neighbors = vec![];

        match self[point] {
            '>' if avoid_uphill => possible_neighbors.push(Point::new(point.x + 1, point.y)),
            'v' if avoid_uphill => possible_neighbors.push(Point::new(point.x, point.y + 1)),
            _ => possible_neighbors.extend(
                [(1, 0), (0, 1), (-1, 0), (0, -1)]
                    .iter()
                    .map(|&(x, y)| Point::new(point.x + x, point.y + y)),
            ),
        }

        possible_neighbors
            .into_iter()
            .filter(|v| {
                v.x >= 0 && v.y >= 0 && v.x < self.width() && v.y < self.height() && self[v] != '#'
            })
            .collect_vec()
    }
}

impl std::ops::Index<&Point> for Grid {
    type Output = char;

    fn index(&self, point: &Point) -> &Self::Output {
        &self.0[point.y as usize][point.x as usize]
    }
}

#[derive(Hash)]
struct Connection {
    destination: Point,
    length: i32,
}

impl Connection {
    fn new(destination: Point, length: i32) -> Self {
        Connection {
            destination,
            length,
        }
    }
}

#[derive(Hash)]
struct Intersection {
    point: Point,
    connections: Vec<Connection>,
}

impl Intersection {
    fn new(point: Point, edges: Vec<Connection>) -> Self {
        Intersection {
            point,
            connections: edges,
        }
    }

    fn get_connection_length(&self, destination: Point) -> i32 {
        self.connections
            .iter()
            .find(|v| v.destination == destination)
            .unwrap()
            .length
    }
}

fn get_intersections(
    grid: &Grid,
    start: &Point,
    end: &Point,
    avoid_uphill: bool,
) -> Vec<Intersection> {
    let mut points = vec![];
    points.push(*start);
    points.extend((0..grid.0.len()).flat_map(|y| {
        (0..grid.0[y].len())
            .filter(move |&x| {
                grid.0[y][x] == '.'
                    && grid
                        .get_neighbors(&Point::new(x as i16, y as i16), avoid_uphill)
                        .len()
                        > 2
            })
            .map(move |x| Point::new(x as i16, y as i16))
    }));
    points.push(*end);

    points
        .iter()
        .cloned()
        .map(|point| {
            let edges: Vec<Connection> = grid
                .get_neighbors(&point, avoid_uphill)
                .into_iter()
                .filter_map(|n| {
                    let mut current = n;
                    let mut visited = vec![];
                    let mut i = 1;
                    while !points.contains(&current) {
                        let neighbors = grid.get_neighbors(&current, avoid_uphill);
                        if neighbors.len() == 1
                            && (neighbors[0] == point || visited.contains(&neighbors[0]))
                        {
                            return None;
                        }
                        if let Some(neighbor) = neighbors
                            .into_iter()
                            .find(|&n| !visited.contains(&n) && n != point)
                        {
                            visited.push(current);
                            current = neighbor;
                            i += 1;
                        }
                    }
                    if current != point {
                        Some(Connection::new(current, i))
                    } else {
                        None
                    }
                })
                .collect();
            Intersection::new(point, edges)
        })
        .collect()
}

fn dfs(
    intersections: &Vec<Intersection>,
    current: &Intersection,
    visited: &mut Vec<Point>,
    mut current_length: i32,
    end: &Point,
    max_length: &mut i32,
) {
    visited.push(current.point);
    if current.point == *end {
        *max_length = *max_length.max(&mut current_length);
        return;
    }

    for connection in &current.connections {
        if visited.contains(&connection.destination) {
            continue;
        }

        let neighbor = intersections
            .iter()
            .find(|v| v.point == connection.destination)
            .unwrap();

        dfs(
            intersections,
            neighbor,
            &mut visited.clone(),
            current_length + current.get_connection_length(connection.destination),
            end,
            max_length,
        )
    }
}

#[timed::timed]
fn part1(data: &str) -> i32 {
    let grid = Grid::new(data);
    let start = Point::new(1, 0);
    let end = Point::new(grid.0[0].len() as i16 - 2, grid.0.len() as i16 - 1);
    let intersections = get_intersections(&grid, &start, &end, true);

    let mut max_length = 0;
    dfs(
        &intersections,
        &intersections[0],
        &mut vec![],
        0,
        &end,
        &mut max_length,
    );
    max_length
}

#[timed::timed]
fn part2(data: &str) -> i32 {
    let grid = Grid::new(data);
    let start = Point::new(1, 0);
    let end = Point::new(grid.0[0].len() as i16 - 2, grid.0.len() as i16 - 1);
    let intersections = get_intersections(&grid, &start, &end, false);

    let mut max_length = 0;
    dfs(
        &intersections,
        &intersections[0],
        &mut vec![],
        0,
        &end,
        &mut max_length,
    );
    max_length
}

fn main() {
    let input = read_input(DAY, InputType::NotTest);
    println!("Part 1: {}", part1(&input.part1));
    println!("Part 2: {}", part2(&input.part2));
}
