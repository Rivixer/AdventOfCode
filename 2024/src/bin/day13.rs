use aoc2024::utils::*;
use regex::Regex;

const DAY: u8 = 13;

struct Button {
    dx: i64,
    dy: i64,
    cost: u64,
}

struct Prize {
    x: i64,
    y: i64,
}

fn parse_input(data: &str) -> Vec<(Vec<Button>, Prize)> {
    let re = Regex::new(r"^(Button A|Button B|Prize): X[+=](\d+), Y[+=](\d+)$").unwrap();

    data.split("\n\n")
        .filter_map(|group| {
            let mut buttons = Vec::new();
            let mut prize = None;

            for line in group.lines() {
                if let Some(caps) = re.captures(line) {
                    let label = &caps[1];
                    let x: i64 = caps[2].parse().ok()?;
                    let y: i64 = caps[3].parse().ok()?;

                    match label {
                        "Button A" => buttons.push(Button {
                            dx: x,
                            dy: y,
                            cost: 3,
                        }),
                        "Button B" => buttons.push(Button {
                            dx: x,
                            dy: y,
                            cost: 1,
                        }),
                        "Prize" => prize = Some(Prize { x, y }),
                        _ => (),
                    }
                }
            }

            prize.map(|p| (buttons, p))
        })
        .collect()
}

fn calculate(buttons: &[Button], prize: &Prize) -> Option<u64> {
    let det = buttons[0].dx * buttons[1].dy - buttons[1].dx * buttons[0].dy;

    if det == 0 {
        return None;
    }

    let i = (prize.x * buttons[1].dy - prize.y * buttons[1].dx) as f64 / det as f64;
    let j = (buttons[0].dx * prize.y - buttons[0].dy * prize.x) as f64 / det as f64;

    if i >= 0.0 && j >= 0.0 && i.fract() == 0.0 && j.fract() == 0.0 {
        Some(i as u64 * buttons[0].cost + j as u64 * buttons[1].cost)
    } else {
        None
    }
}

#[timed::timed]
fn part1(data: &str) -> u64 {
    let data = parse_input(data);
    data.iter()
        .map(|(buttons, prize)| calculate(buttons, prize).unwrap_or(0))
        .sum()
}

#[timed::timed]
fn part2(data: &str) -> u64 {
    let data = parse_input(data);
    let offset = 10000000000000;
    data.iter()
        .map(|(buttons, prize)| {
            calculate(
                buttons,
                &Prize {
                    x: prize.x + offset,
                    y: prize.y + offset,
                },
            )
            .unwrap_or(0)
        })
        .sum()
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
        assert_eq!(part1(&get_test_input().part1), 480);
    }

    fn get_test_input() -> Input {
        read_input(DAY, InputType::Test)
    }
}
