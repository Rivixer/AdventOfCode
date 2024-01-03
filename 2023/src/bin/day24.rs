use aoc2023::utils::*;
use itertools::Itertools;
use regex::Regex;

const DAY: u8 = 24;

#[derive(Debug)]
struct Hailstone {
    x: i64,
    y: i64,
    z: i64,
    dx: i64,
    dy: i64,
    dz: i64,
}

impl Hailstone {
    fn new(x: i64, y: i64, z: i64, dx: i64, dy: i64, dz: i64) -> Self {
        Hailstone {
            x,
            y,
            z,
            dx,
            dy,
            dz,
        }
    }

    fn position(&self) -> [i64; 3] {
        [self.x, self.y, self.z]
    }

    fn velocity(&self) -> [i64; 3] {
        [self.dx, self.dy, self.dz]
    }
}

fn parse_input(data: &str) -> Vec<Hailstone> {
    let re = Regex::new(r"-?\d+").unwrap();
    data.lines()
        .map(|line| {
            let r = re
                .find_iter(line)
                .flat_map(|v| v.as_str().parse::<i64>())
                .collect_vec();
            Hailstone::new(r[0], r[1], r[2], r[3], r[4], r[5])
        })
        .collect()
}

#[derive(Debug)]
enum HailstoneIntersection {
    Parallel,
    Crossable(bool, f32, f32),
}

fn intersect_hailstones(hailstone1: &Hailstone, hailstone2: &Hailstone) -> HailstoneIntersection {
    let slope1 = hailstone1.dy as f32 / hailstone1.dx as f32;
    let slope2 = hailstone2.dy as f32 / hailstone2.dx as f32;

    if slope1 == slope2 {
        HailstoneIntersection::Parallel
    } else {
        let intercept1 = -slope1 * hailstone1.x as f32 + hailstone1.y as f32;
        let intercept2 = -slope2 * hailstone2.x as f32 + hailstone2.y as f32;

        let intersection_x = (intercept2 - intercept1) / (slope1 - slope2);
        let intersection_y = intersection_x * slope1 + intercept1;

        let crossed = (intersection_x <= hailstone1.x as f32) ^ (hailstone1.dx <= 0)
            || (intersection_x <= hailstone2.x as f32) ^ (hailstone2.dx <= 0);

        HailstoneIntersection::Crossable(!crossed, intersection_x, intersection_y)
    }
}

#[timed::timed]
fn part1(data: &str, min_pos: i64, max_pos: i64) -> usize {
    let hailstones = parse_input(data);

    hailstones
        .iter()
        .enumerate()
        .flat_map(|(i, hailstone1)| {
            hailstones
                .iter()
                .skip(i + 1)
                .map(move |hailstone2| (hailstone1, hailstone2))
        })
        .map(|(hailstone1, hailstone2)| intersect_hailstones(hailstone1, hailstone2))
        .filter(|intersection| match intersection {
            HailstoneIntersection::Crossable(true, x, y) => {
                (min_pos as f32..=max_pos as f32).contains(x)
                    && (min_pos as f32..=max_pos as f32).contains(y)
            }
            _ => false,
        })
        .count()
}

type Vector3 = [i64; 3];
type Vector6 = [i64; 6];
type Matrix3_3 = [[i64; 3]; 3];
type Matrix6_6 = [[f64; 6]; 6];

fn gaussian_elimination(matrix: &Matrix6_6, rhs: &Vector6) -> Vector6 {
    let mut augmented_matrix: Vec<[f64; 7]> = matrix
        .iter()
        .enumerate()
        .map(|(i, &row)| {
            let mut new_row = row.to_vec();
            new_row.push(rhs[i] as f64);
            new_row.try_into().unwrap()
        })
        .collect_vec();

    (0..6).for_each(|pivot_row| {
        let pivot_col = augmented_matrix
            .iter()
            .enumerate()
            .skip(pivot_row)
            .max_by_key(|&(_, row)| row[pivot_row].abs() as i64)
            .map(|(i, _)| i)
            .unwrap();

        augmented_matrix.swap(pivot_row, pivot_col);

        let pivot = augmented_matrix[pivot_row][pivot_row];
        (pivot_row..=6).for_each(|col| {
            augmented_matrix[pivot_row][col] /= pivot;
        });

        (0..6).for_each(|row| {
            if row != pivot_row {
                let factor = augmented_matrix[row][pivot_row];
                (pivot_row..=6).for_each(|col| {
                    augmented_matrix[row][col] -= factor * augmented_matrix[pivot_row][col];
                });
            }
        });
    });

    let solution: Vec<i64> = augmented_matrix
        .into_iter()
        .map(|row| row[6] as i64)
        .collect();

    [
        solution[0],
        solution[1],
        solution[2],
        solution[3],
        solution[4],
        solution[5],
    ]
}

fn cross_product_matrix(vector: &Vector3) -> Matrix3_3 {
    [
        [0, -vector[2], vector[1]],
        [vector[2], 0, -vector[0]],
        [-vector[1], vector[0], 0],
    ]
}

fn vector_product(v1: &Vector3, v2: &Vector3) -> Vector3 {
    [
        v1[1] * v2[2] - v1[2] * v2[1],
        v1[2] * v2[0] - v1[0] * v2[2],
        v1[0] * v2[1] - v1[1] * v2[0],
    ]
}

fn vector_diff(v1: &Vector3, v2: &Vector3) -> Vector3 {
    [v1[0] - v2[0], v1[1] - v2[1], v1[2] - v2[2]]
}

#[timed::timed]
fn part2(data: &str) -> i64 {
    /*
        rock_position + time * rock_velocity = any_hailstone_position + time * any_hailstone_velocity
        rock_position = any_hailstone_position + time * (any_hailstone_velocity - rock_velocity)

        rock_position - any_hailstone_position =   time   * (any_hailstone_velocity - rock_velocity)
           [Vector]   -        [Vector]        = [Scalar] * (       [Vector]        -    [Vector]  )

        Using MS Paint to visualize this, I realized that
            (rock_position - any_hailstone_position) is parallel to (rock_velocity - any_hailstone_velocity), so
            (rock_position - any_hailstone_position) x (rock_velocity - any_hailstone_velocity) = 0
    */

    let hailstones = parse_input(data);

    let mut hailstones_pxv = [[0; 3]; 4];
    for i in 0..4 {
        hailstones_pxv[i] = vector_product(&hailstones[i].position(), &hailstones[i].velocity())
    }

    let rhs_position = vector_diff(&hailstones_pxv[1], &hailstones_pxv[0]);
    let rhs_velocity = vector_diff(&hailstones_pxv[3], &hailstones_pxv[2]);

    let rhs = [
        rhs_position[0],
        rhs_position[1],
        rhs_position[2],
        rhs_velocity[0],
        rhs_velocity[1],
        rhs_velocity[2],
    ];

    let m01_velocity = cross_product_matrix(&vector_diff(
        &hailstones[0].velocity(),
        &hailstones[1].velocity(),
    ));

    let m23_velocity = cross_product_matrix(&vector_diff(
        &hailstones[2].velocity(),
        &hailstones[3].velocity(),
    ));

    let m01_position = cross_product_matrix(&vector_diff(
        &hailstones[0].position(),
        &hailstones[1].position(),
    ));

    let m23_position = cross_product_matrix(&vector_diff(
        &hailstones[2].position(),
        &hailstones[3].position(),
    ));

    let matrix: [[f64; 6]; 6] = [
        [
            m01_velocity[0][0] as f64,
            m01_velocity[0][1] as f64,
            m01_velocity[0][2] as f64,
            m01_position[0][0] as f64,
            m01_position[0][1] as f64,
            m01_position[0][2] as f64,
        ],
        [
            m01_velocity[1][0] as f64,
            m01_velocity[1][1] as f64,
            m01_velocity[1][2] as f64,
            m01_position[1][0] as f64,
            m01_position[1][1] as f64,
            m01_position[1][2] as f64,
        ],
        [
            m01_velocity[2][0] as f64,
            m01_velocity[2][1] as f64,
            m01_velocity[2][2] as f64,
            m01_position[2][0] as f64,
            m01_position[2][1] as f64,
            m01_position[2][2] as f64,
        ],
        [
            m23_velocity[0][0] as f64,
            m23_velocity[0][1] as f64,
            m23_velocity[0][2] as f64,
            m23_position[0][0] as f64,
            m23_position[0][1] as f64,
            m23_position[0][2] as f64,
        ],
        [
            m23_velocity[1][0] as f64,
            m23_velocity[1][1] as f64,
            m23_velocity[1][2] as f64,
            m23_position[1][0] as f64,
            m23_position[1][1] as f64,
            m23_position[1][2] as f64,
        ],
        [
            m23_velocity[2][0] as f64,
            m23_velocity[2][1] as f64,
            m23_velocity[2][2] as f64,
            m23_position[2][0] as f64,
            m23_position[2][1] as f64,
            m23_position[2][2] as f64,
        ],
    ];

    gaussian_elimination(&matrix, &rhs).iter().take(3).sum()
}

fn main() {
    let input_type = InputType::NotTest;
    let (min_pos, max_pos) = match input_type {
        InputType::Test => (7, 27),
        InputType::NotTest => (200000000000000, 400000000000000),
        _ => todo!(),
    };

    let input = read_input(DAY, input_type);
    println!("Part 1: {}", part1(&input.part1, min_pos, max_pos));
    println!("Part 2: {}", part2(&input.part2));
}
