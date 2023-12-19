use aoc2023::utils::*;
use itertools::Itertools;
use regex::Regex;
use std::str::FromStr;

const DAY: u8 = 19;

struct Part {
    x: i32,
    m: i32,
    a: i32,
    s: i32,
}

impl Part {
    fn new(values: &[i32]) -> Self {
        Part {
            x: values[0],
            m: values[1],
            a: values[2],
            s: values[3],
        }
    }
}

#[derive(Debug)]
enum Operator {
    LT,
    GT,
}

impl FromStr for Operator {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "<" => Ok(Operator::LT),
            ">" => Ok(Operator::GT),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct Condition {
    category: String,
    operator: Operator,
    value: i32,
}

impl Condition {
    fn new(category: String, operator: Operator, value: i32) -> Self {
        Condition {
            category,
            operator,
            value,
        }
    }
}

#[derive(Debug)]
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
                    .map(|v| v.as_str().parse::<i32>().unwrap())
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
                captures.get(3).unwrap().as_str().parse::<i32>().unwrap(),
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
fn part1(data: &str) -> i32 {
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
        .sum()
}

#[timed::timed]
fn part2(data: &str) -> i32 {
    todo!()
}

fn main() {
    let input = read_input(DAY, InputType::NotTest);
    println!("Part 1: {}", part1(&input.part1));
    // println!("Part 2: {}", part2(&input.part1));
}
