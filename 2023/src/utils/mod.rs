use std::fs;

#[derive(PartialEq)]
pub enum InputType {
    Test,
    NotTest,
}

pub fn read_input(day: i32, input_type: InputType) -> String {
    let file_path = format!(
        "inputs/day{:0>2}{}.txt",
        day,
        if InputType::Test == input_type {
            "_test"
        } else {
            ""
        }
    );
    fs::read_to_string(file_path).unwrap()
}
