use regex::Regex;
use std::{error::Error, fs::read_to_string};

fn clean_line(string_line: &String) -> Result<String, Box<dyn Error>> {
    let easy_removed_str = string_line.replace("\\\"", "+").replace("\\\\", "+");

    let clean_str = Regex::new(r"\\x\w\w")?
        .replace_all(&easy_removed_str, "+")
        .to_string();

    Ok(clean_str)
}

pub fn solution_first_part() -> Result<usize, Box<dyn Error>> {
    let amount_of_memory_characters = read_to_string("data")?
        .lines()
        .map(|v| v.as_bytes().len())
        .fold(0 as usize, |acc, v| acc + v);

    let amount_of_string_characters = read_to_string("data")?
        .lines()
        .map(|v| v.get(1..v.len() - 1).unwrap())
        .map(|v| v.to_string())
        .map(|v| clean_line(&v.to_string()).unwrap())
        .map(|v| v.len())
        .fold(0 as usize, |acc, v| acc + v);

    Ok(amount_of_memory_characters - amount_of_string_characters)
}
