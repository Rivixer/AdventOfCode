use aoc2023::utils::*;
use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, VecDeque};

const DAY: u8 = 19;

struct Part {
    x: i64,
    m: i64,
    a: i64,
    s: i64,
}

impl Part {
    fn new(values: &[i64]) -> Self {
        Part {
            x: values[0],
            m: values[1],
            a: values[2],
            s: values[3],
        }
    }
}

#[derive(Debug, Clone)]
enum Operator {
    LT,
    GT,
}

#[derive(Debug, Clone)]
struct Condition {
    category: String,
    operator: Operator,
    value: i64,
}

impl Condition {
    fn new(category: String, operator: Operator, value: i64) -> Self {
        Condition {
            category,
            operator,
            value,
        }
    }
}

#[derive(Debug, Clone)]
struct Rule {
    condition: Option<Condition>,
    destination: String,
}

impl Rule {
    fn new(condition: Option<Condition>, destination: String) -> Self {
        Rule {
            condition,
            destination,
        }
    }
}

#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

impl Workflow {
    fn new(name: String, rules: Vec<Rule>) -> Self {
        Workflow { name, rules }
    }
}

fn parse_input(data: &str) -> (Vec<Workflow>, Vec<Part>) {
    let re_workflow = Regex::new(r"([a-z]+)\{([a-zA-Z><\d+:,]+)\}").unwrap();
    let re_rule = Regex::new(r"(?:([xmas])([<>])(\d+):)?([a-z]+|[AR])").unwrap();
    let re_part = Regex::new(r"\d+").unwrap();
    let (workflow_data, parts_data) = data.split("\r\n\r\n").collect_tuple().unwrap();

    let mut workflows = vec![];
    workflow_data.lines().for_each(|line| {
        if let Some(captures) = re_workflow.captures(line) {
            let rules = captures[2]
                .to_string()
                .split(',')
                .filter_map(|rule| parse_rule(rule, &re_rule))
                .collect();
            workflows.push(Workflow::new(captures[1].to_string(), rules));
        }
    });
    workflows.push(Workflow::new("A".to_string(), vec![]));
    workflows.push(Workflow::new("R".to_string(), vec![]));

    let parts = parts_data
        .lines()
        .map(|line| {
            Part::new(
                &re_part
                    .find_iter(line)
                    .map(|v| v.as_str().parse::<i64>().unwrap())
                    .collect_vec(),
            )
        })
        .collect();

    (workflows, parts)
}

fn parse_rule(rule: &str, regex: &Regex) -> Option<Rule> {
    if let Some(captures) = regex.captures(rule) {
        let category = captures.get(1).map_or("", |m| m.as_str()).to_string();
        let destination = captures.get(4).map_or("", |m| m.as_str()).to_string();
        if !category.is_empty() {
            let condition = Some(Condition::new(
                category,
                match captures.get(2).unwrap().as_str() {
                    "<" => Operator::LT,
                    ">" => Operator::GT,
                    _ => unreachable!(),
                },
                captures.get(3).unwrap().as_str().parse::<i64>().unwrap(),
            ));
            return Some(Rule::new(condition, destination));
        } else {
            return Some(Rule::new(None, destination));
        }
    }
    None
}

fn find_matching_rule<'a>(workflow: &'a Workflow, part: &'a Part) -> Option<&'a Rule> {
    workflow.rules.iter().find(|rule| {
        rule.condition.as_ref().map_or(true, |c| {
            let value = match c.category.as_str() {
                "x" => part.x,
                "m" => part.m,
                "a" => part.a,
                "s" => part.s,
                _ => unreachable!(),
            };
            match c.operator {
                Operator::LT => value < c.value,
                Operator::GT => value > c.value,
            }
        })
    })
}

#[timed::timed]
fn part1(data: &str) -> i64 {
    let (workflows, parts) = parse_input(data);
    parts
        .iter()
        .map(|part| {
            let mut workflow = workflows.iter().find(|v| v.name == "in").unwrap();
            while workflow.name != "A" && workflow.name != "R" {
                if let Some(rule) = find_matching_rule(workflow, part) {
                    workflow = workflows
                        .iter()
                        .find(|v| v.name == rule.destination)
                        .unwrap();
                } else {
                    break;
                }
            }
            if workflow.name == "A" {
                part.x + part.m + part.a + part.s
            } else {
                0
            }
        })
        .sum::<i64>()
}

#[timed::timed]
fn part2(data: &str) -> i64 {
    let (workflows, _) = parse_input(data);
    let mut result = 0;
    let mut queue: VecDeque<(String, HashMap<&str, (i64, i64)>)> = VecDeque::from_iter(vec![(
        "in".to_string(),
        HashMap::from_iter(vec![
            ("x", (1, 4000)),
            ("m", (1, 4000)),
            ("a", (1, 4000)),
            ("s", (1, 4000)),
        ]),
    )]);

    while let Some((workflow, mut part)) = queue.pop_front() {
        if workflow == "A" {
            result += part
                .values()
                .map(|range| range.1 - range.0 + 1)
                .product::<i64>();
            continue;
        } else if workflow == "R" {
            continue;
        }

        let rules = &workflows.iter().find(|w| w.name == workflow).unwrap().rules;

        rules.iter().for_each(|rule| {
            if let Some(condition) = &rule.condition {
                let category_range = part.get(condition.category.as_str()).unwrap();
                let mut old_start = category_range.0;
                let mut old_stop = category_range.1;
                let mut new_start = category_range.0;
                let mut new_stop = category_range.1;

                match condition.operator {
                    Operator::LT => {
                        old_start = std::cmp::max(old_start, condition.value);
                        new_stop = std::cmp::min(new_stop, condition.value - 1);
                    }
                    Operator::GT => {
                        new_start = std::cmp::max(new_start, condition.value + 1);
                        old_stop = std::cmp::min(old_stop, condition.value);
                    }
                }

                let mut new_part = part.clone();
                new_part.insert(condition.category.as_str(), (new_start, new_stop));
                queue.push_back((rule.destination.clone(), new_part));
                part.insert(condition.category.as_str(), (old_start, old_stop));
            } else {
                queue.push_back((rule.destination.clone(), part.clone()));
            }
        })
    }

    result
}

fn main() {
    let input = read_input(DAY, InputType::NotTest);
    println!("Part 1: {}", part1(&input.part1));
    println!("Part 2: {}", part2(&input.part1));
}
