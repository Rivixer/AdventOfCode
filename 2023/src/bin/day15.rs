use aoc2023::utils::*;

const DAY: u8 = 15;

fn hash(item: &str) -> usize {
    item.chars().fold(0, |acc, x| acc * 17 % 256 + x as usize) * 17 % 256
}

#[timed::timed]
fn part1(data: &str) -> usize {
    data.split(',').map(|v| hash(v)).sum()
}

#[timed::timed]
fn part2(data: &str) -> usize {
    let mut boxes: Vec<Vec<(&str, usize)>> = vec![vec![]; 256];
    data.split(',').for_each(|inp| match inp.split_once('=') {
        Some((a, b)) => {
            if let Some(i) = boxes[hash(a)].iter().position(|v| v.0 == a) {
                boxes[hash(a)].remove(i);
                boxes[hash(a)].insert(i, (a, b.parse().unwrap()));
            } else {
                boxes[hash(a)].push((a, b.parse().unwrap()));
            }
        }
        None => {
            let a = inp.get(0..inp.len() - 1).unwrap();
            boxes[hash(a)].retain(|(v, _)| **v != *a)
        }
    });
    boxes
        .iter()
        .enumerate()
        .map(|(box_index, r#box)| {
            r#box
                .iter()
                .map(|v| v.1)
                .enumerate()
                .map(|(box_slot, value)| (box_index + 1) * (box_slot + 1) * value)
                .sum::<usize>()
        })
        .sum()
}

fn main() {
    let input = read_input(DAY, InputType::NotTest);
    println!("Part 1: {}", part1(&input.part1));
    println!("Part 2: {}", part2(&input.part2));
}
