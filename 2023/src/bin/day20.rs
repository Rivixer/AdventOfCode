use aoc2023::utils::*;
use regex::Regex;
use std::collections::{BinaryHeap, HashMap};

const DAY: u8 = 20;

#[derive(Clone, PartialEq, Eq)]
enum ModuleType {
    Broadcast,
    FlipFlop(bool),
    Conjunction(HashMap<String, bool>),
}

struct Module {
    r#type: ModuleType,
    destinations: Vec<String>,
}

impl Module {
    fn new(r#type: ModuleType, destinations: Vec<String>) -> Self {
        Module {
            r#type,
            destinations,
        }
    }
}

fn parse_input(data: &str) -> HashMap<String, Module> {
    let re = Regex::new(r"([%&]?\w+) -> (.+)").unwrap();
    data.lines()
        .map(|line| {
            let captures = re.captures(line).unwrap();
            let name = captures[1].to_string();
            let destinations = captures[2]
                .split(", ")
                .map(|x| x.to_string())
                .collect::<Vec<String>>();

            let (module_type, name) = match name.as_str() {
                "broadcaster" => (ModuleType::Broadcast, name),
                _ => {
                    let (t, name) = name.split_at(1);
                    let module_type = match t {
                        "%" => ModuleType::FlipFlop(false),
                        "&" => ModuleType::Conjunction(
                            destinations.iter().map(|d| (d.clone(), false)).collect(),
                        ),
                        _ => unreachable!(),
                    };
                    (module_type, name.to_string())
                }
            };

            (name, Module::new(module_type, destinations))
        })
        .collect()
}

fn get_instructions(modules: &HashMap<String, Module>) -> HashMap<String, Vec<String>> {
    modules
        .iter()
        .flat_map(|(name, module)| {
            module
                .destinations
                .iter()
                .map(move |d| (d.clone(), name.clone()))
        })
        .fold(HashMap::new(), |mut acc, (d, name)| {
            acc.entry(d).or_default().push(name);
            acc
        })
}

fn get_map(
    modules: &HashMap<String, Module>,
    map: &HashMap<String, Vec<String>>,
) -> HashMap<String, ModuleType> {
    modules
        .iter()
        .filter_map(|(name, module)| match &module.r#type {
            ModuleType::FlipFlop(_) => Some((name.clone(), ModuleType::FlipFlop(false))),
            ModuleType::Conjunction(_) => Some((
                name.clone(),
                ModuleType::Conjunction(
                    map.get(name)
                        .unwrap()
                        .iter()
                        .map(|d| (d.clone(), false))
                        .collect(),
                ),
            )),
            _ => None,
        })
        .collect()
}

#[timed::timed]
fn calculate(data: &str) -> (i32, i64) {
    let modules = parse_input(data);
    let instructions = get_instructions(&modules);
    let mut map = get_map(&modules, &instructions);
    let mut i: usize = 0;

    // Part 1
    let mut low = 0;
    let mut high = 0;
    let mut result1 = None;

    // Part 2
    let rx_sources = &instructions[&instructions["rx"][0]];
    let mut cache: HashMap<String, i64> = HashMap::new();
    let mut result2 = None;

    while result1.is_none() || result2.is_none() {
        if i == 1000 {
            result1 = Some(low * high);
        }

        if cache.len() == rx_sources.len() {
            result2 = Some(cache.values().product())
        }

        i += 1;
        let mut queue: BinaryHeap<(Option<String>, _, _)> =
            [(None, "broadcaster".to_string(), false)].into();

        while !queue.is_empty() {
            let mut new_queue = BinaryHeap::new();

            for (source, name, is_high) in queue {
                if is_high {
                    high += 1;
                } else {
                    low += 1;
                    if rx_sources.contains(&name) && !cache.contains_key(&name) {
                        cache.insert(name.clone(), i as i64);
                    }
                }

                if let Some(module) = modules.get(&name) {
                    if let Some(new_state) = match module.r#type {
                        ModuleType::FlipFlop(_) if !is_high => {
                            if let Some(ModuleType::FlipFlop(state)) = map.get_mut(&name) {
                                *state ^= true;
                                Some(*state)
                            } else {
                                None
                            }
                        }
                        ModuleType::Conjunction(_) => {
                            if let Some(ModuleType::Conjunction(state)) = map.get_mut(&name) {
                                if let Some(source) = source {
                                    state.insert(source.clone(), is_high);
                                }
                                Some(state.values().filter(|&&v| v).count() != state.len())
                            } else {
                                None
                            }
                        }
                        ModuleType::Broadcast => Some(is_high),
                        _ => None,
                    } {
                        module.destinations.iter().for_each(|dest| {
                            new_queue.push((Some(name.clone()), dest.clone(), new_state));
                        });
                    }
                }
            }

            queue = new_queue;
        }
    }

    (result1.unwrap(), result2.unwrap())
}

fn main() {
    let input = read_input(DAY, InputType::NotTest);
    let results = calculate(&input.part1);
    println!("Part 1: {}", results.0);
    println!("Part 2: {}", results.1);
}
