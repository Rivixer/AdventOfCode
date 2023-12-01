use std::fs;

#[derive(PartialEq)]
pub enum InputType {
    Test,
    NotTest,
    Other(String),
    OtherSep(String, String),
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

fn read_from_file(day: u8, input_type: InputType) -> Input {
    match input_type {
        InputType::NotTest => {
            let file_path = format!("inputs/day{:0>2}.txt", day);
            Input::from_one_input(&fs::read_to_string(&file_path).unwrap())
        }
        InputType::Test => {
            let file_path1 = format!("inputs/day{:0>2}_test.txt", day);
            let file_path2 = format!("inputs/day{:0>2}_test2.txt", day);
            let p1_input = fs::read_to_string(file_path1).unwrap();
            let p2_input = fs::read_to_string(file_path2).unwrap_or(p1_input.clone());
            Input::from_two_inputs(p1_input, p2_input)
        }
        _ => panic!(),
    }
}

pub fn read_input(day: u8, input_type: InputType) -> Input {
    match input_type {
        InputType::Test => read_from_file(day, input_type),
        InputType::NotTest => read_from_file(day, input_type),
        InputType::Other(s) => Input::from_one_input(&s),
        InputType::OtherSep(s1, s2) => Input::from_two_inputs(s1, s2),
    }
}
