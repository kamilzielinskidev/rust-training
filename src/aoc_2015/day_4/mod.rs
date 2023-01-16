use md5;
use std::{error::Error, fs::read_to_string};

pub fn solution_first_and_second_part() -> Result<usize, Box<dyn Error>> {
    // TODO: probably the computations can be done faster if used str instead of String
    // probably can do str like "qweqwe00000000" with fixed size and mutate the numbers only for check
    // so there will be no need for cloning the string every single time
    let file_string = read_to_string("data")?.trim().to_string();
    let mut number = 0;

    loop {
        let hashed_value = md5::compute(file_string.clone() + number.clone().to_string().as_str());
        if format!("{:x}", hashed_value).starts_with("000000") {
            break;
        }
        number += 1;
    }

    Ok(number)
}
