use std::fs;

#[derive(PartialEq)]
pub enum InputType {
    Test,
    Production,
    Other(String),
}

pub struct Input {
    pub part1: String,
    pub part2: String,
}

impl Input {
    fn from_one_input(input: &str) -> Self {
        Self {
            part1: input.to_string(),
            part2: input.to_string(),
        }
    }

    fn from_two_inputs(i1: String, i2: String) -> Self {
        Self {
            part1: i1,
            part2: i2,
        }
    }
}

pub fn read_input(day: u8, input_type: InputType) -> Input {
    match input_type {
        InputType::Test => {
            let path = format!("inputs/test/day{:0>2}", day);
            if fs::metadata(&path).is_ok() {
                Input::from_one_input(&fs::read_to_string(&path).unwrap())
            } else {
                let path1 = format!("inputs/test/day{:0>2}p1", day);
                let path2 = format!("inputs/test/day{:0>2}p2", day);
                let content1 = fs::read_to_string(&path1).unwrap();
                let content2 = fs::read_to_string(&path2).unwrap();
                Input::from_two_inputs(content1, content2)
            }
        }
        InputType::Production => {
            let path = format!("inputs/day{:0>2}", day);
            Input::from_one_input(&fs::read_to_string(&path).unwrap())
        }
        InputType::Other(s) => {
            let path = format!("inputs/other/day{:0>2}_{s}", day);
            Input::from_one_input(&fs::read_to_string(&path).unwrap())
        }
    }
}