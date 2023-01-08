use std::{error, fs::read_to_string};

fn split_single_chars(value: &String) -> Vec<String> {
    value
        .split("")
        .filter(|v| !v.is_empty())
        .map(String::from)
        .collect::<Vec<String>>()
}

pub fn solution_first_part() -> Result<usize, Box<dyn error::Error>> {
    let file_string = read_to_string("data").map(|v| String::from(v.trim()))?;
    let floors_signs = split_single_chars(&file_string);
    let floor = floors_signs.iter().fold(0, |acc, v| match v.as_str() {
        "(" => acc + 1,
        ")" => acc - 1,
        _ => panic!("Wrong sign!"),
    });

    Ok(floor)
}

pub fn solution_second_part() -> Result<usize, Box<dyn error::Error>> {
    let file_string = read_to_string("data").map(|v| String::from(v.trim()))?;
    let floors_signs = split_single_chars(&file_string);
    let floors_with_indexes = floors_signs.iter().enumerate();
    let floor_index = floors_with_indexes
        .fold((0, 0), |(floor, index), (next_index, value)| {
            // TODO: redo this with let's instead of matches
            match value.as_str() {
                "(" => (floor + 1, index),
                ")" => match index {
                    0 => match floor {
                        0 => (floor - 1, next_index + 1),
                        _ => (floor - 1, index),
                    },
                    _ => (floor, index),
                },
                _ => panic!("Wrong sign!"),
            }
        })
        .1;

    Ok(floor_index)
}
