use aoc2024::utils::*;

const DAY: u8 = 3;

#[timed::timed]
fn part1(data: &str) -> usize {
    let pattern = r"mul\((\d+),(\d+)\)";
    let re = regex::Regex::new(pattern).unwrap();
    re.captures_iter(data)
        .map(|cap| {
            let a = cap[1].parse::<usize>().unwrap();
            let b = cap[2].parse::<usize>().unwrap();
            a * b
        })
        .sum()
}

#[timed::timed]
fn part2(data: &str) -> usize {
    let pattern = r"mul\((?P<d1>\d+),(?P<d2>\d+)\)|(?P<do>do\(\))|(?P<dont>don't\(\))";
    let re = regex::Regex::new(pattern).unwrap();

    re.captures_iter(data)
        .scan(true, |enabled, cap| {
            if let (Some(d1), Some(d2)) = (
                cap.name("d1").map(|m| m.as_str().parse::<usize>().unwrap()),
                cap.name("d2").map(|m| m.as_str().parse::<usize>().unwrap()),
            ) {
                return Some(if *enabled { d1 * d2 } else { 0 });
            }

            if cap.name("do").is_some() {
                *enabled = true;
            } else if cap.name("dont").is_some() {
                *enabled = false;
            }

            Some(0)
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

    fn get_test_input() -> Input {
        read_input(DAY, InputType::Test)
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&get_test_input().part1), 161);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&get_test_input().part2), 48);
    }
}
