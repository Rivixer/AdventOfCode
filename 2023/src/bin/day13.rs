use aoc2023::utils::*;

const DAY: u8 = 13;

fn find_horizontal_line_index(
    pattern: &Vec<Vec<char>>,
    skip_line: Option<&Vec<char>>,
) -> Option<usize> {
    let height = pattern.len();
    let width = pattern[0].len();

    (1..height).find_map(|y| {
        let is_skip_line = skip_line.map_or(false, |skip| *skip == pattern[y]);
        if !is_skip_line {
            if (0..width).all(|x| {
                (0..(height - y).min(y))
                    .all(|offset| pattern[y - offset - 1][x] == pattern[y + offset][x])
            }) {
                Some(y)
            } else {
                None
            }
        } else {
            None
        }
    })
}

fn transpose_pattern(pattern: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    (0..pattern[0].len())
        .map(|i| {
            pattern
                .iter()
                .map(|inner| inner[i].clone())
                .collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<_>>>()
}

fn find_line(
    pattern: &Vec<Vec<char>>,
    skip_at_line: Option<&Vec<char>>,
) -> Option<(Vec<char>, usize)> {
    let transposed_pattern = &transpose_pattern(pattern);
    if let Some(i) = find_horizontal_line_index(pattern, skip_at_line) {
        Some((pattern[i].to_vec(), i * 100))
    } else if let Some(i) = find_horizontal_line_index(transposed_pattern, skip_at_line) {
        Some((transposed_pattern[i].to_vec(), i))
    } else {
        None
    }
}

fn find_with_smudge(pattern: &Vec<Vec<char>>) -> usize {
    let line = find_line(pattern, None).unwrap().0;

    let height = pattern.len();
    let width = pattern[0].len();

    (0..width)
        .flat_map(|x| (0..height).map(move |y| (x, y)))
        .map(|(x, y)| {
            let mut mut_pattern = pattern.clone();
            mut_pattern[y][x] = if pattern[y][x] == '.' { '#' } else { '.' };
            find_line(&mut_pattern, Some(&line))
        })
        .find_map(|new| new.map(|v| v.1))
        .unwrap_or_else(|| unreachable!())
}

fn parse_input(data: &str) -> Vec<Vec<Vec<char>>> {
    data.split("\r\n\r\n")
        .map(|v| v.lines().map(|v| v.trim().chars().collect()).collect())
        .collect()
}

#[timed::timed]
fn part1(data: &str) -> usize {
    let patterns = parse_input(data);
    patterns.iter().map(|v| find_line(v, None).unwrap().1).sum()
}

#[timed::timed]
fn part2(data: &str) -> usize {
    let patterns = parse_input(data);
    patterns.iter().map(|v| find_with_smudge(v)).sum()
}

fn main() {
    let input = read_input(DAY, InputType::NotTest);
    println!("Part 1: {}", part1(&input.part1));
    println!("Part 2: {}", part2(&input.part2));
}
