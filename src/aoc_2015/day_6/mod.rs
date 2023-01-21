use regex::Regex;
use std::{error::Error, fs::read_to_string};

#[derive(Debug)]
struct Coordinates(i32, i32);

#[derive(Debug)]
struct Instruction(String, (Coordinates, Coordinates));

pub fn solution_first_part() -> Result<usize, Box<dyn Error>> {
    let size_of_map = 999;

    let file_string = read_to_string("data")?.trim().to_string();

    let instructions_lines = file_string.lines();

    let instructions = instructions_lines
        .map(|line| {
            let command = Regex::new(r"(^\D*)\s")?
                .captures(line)
                .ok_or("cant capture line")?
                .get(0)
                .ok_or("out of scope")?
                .as_str()
                .trim()
                .to_string();

            let coordinates = Regex::new(r"\d+")?
                .find_iter(line)
                .map(|v| v.as_str().parse())
                .collect::<Result<Vec<i32>, _>>()?;

            Ok(Instruction(
                command,
                (
                    Coordinates(
                        *coordinates.get(0).ok_or("out of scope")?,
                        *coordinates.get(1).ok_or("out of scope")?,
                    ),
                    Coordinates(
                        *coordinates.get(2).ok_or("out of scope")?,
                        *coordinates.get(3).ok_or("out of scope")?,
                    ),
                ),
            ))
        })
        .collect::<Result<Vec<Instruction>, Box<dyn Error>>>()?;

    let mut lights_map: Vec<Vec<bool>> = vec![];

    for _ in 0..size_of_map + 1 {
        let row: Vec<bool> = (0..size_of_map + 1).map(|_| false).collect();

        lights_map.push(row.clone());
    }

    for instruction in instructions.iter() {
        let (from, to) = &instruction.1;

        for row in from.0..to.0 + 1 {
            for column in from.1..to.1 + 1 {
                lights_map.get_mut(row as usize).map(|row| {
                    let value = row.get_mut(column as usize);
                    if let None = value {
                        panic!("out of scope")
                    }

                    if let Some(v) = value {
                        match instruction.0.as_str() {
                            "turn on" => {
                                *v = true;
                            }
                            "turn off" => {
                                *v = false;
                            }
                            "toggle" => {
                                *v = !v.clone();
                            }
                            _ => panic!("unexpected instruction"),
                        }
                    }
                });
            }
        }
    }

    let mut amount_of_turned_on = 0;

    for row in lights_map.iter() {
        for value in row.iter() {
            if *value {
                amount_of_turned_on += 1;
            }
        }
    }

    Ok(amount_of_turned_on)
}

pub fn solution_second_part() -> Result<usize, Box<dyn Error>> {
    let size_of_map = 999;

    let file_string = read_to_string("data")?.trim().to_string();

    let instructions_lines = file_string.lines();

    let instructions = instructions_lines
        .map(|line| {
            let command = Regex::new(r"(^\D*)\s")?
                .captures(line)
                .ok_or("cant capture line")?
                .get(0)
                .ok_or("out of scope")?
                .as_str()
                .trim()
                .to_string();

            let coordinates = Regex::new(r"\d+")?
                .find_iter(line)
                .map(|v| v.as_str().parse())
                .collect::<Result<Vec<i32>, _>>()?;

            Ok(Instruction(
                command,
                (
                    Coordinates(
                        *coordinates.get(0).ok_or("out of scope")?,
                        *coordinates.get(1).ok_or("out of scope")?,
                    ),
                    Coordinates(
                        *coordinates.get(2).ok_or("out of scope")?,
                        *coordinates.get(3).ok_or("out of scope")?,
                    ),
                ),
            ))
        })
        .collect::<Result<Vec<Instruction>, Box<dyn Error>>>()?;

    let mut lights_map: Vec<Vec<usize>> = vec![];

    for _ in 0..size_of_map + 1 {
        let row: Vec<usize> = (0..size_of_map + 1).map(|_| 0).collect();

        lights_map.push(row.clone());
    }

    for instruction in instructions.iter() {
        let (from, to) = &instruction.1;

        for row in from.0..to.0 + 1 {
            for column in from.1..to.1 + 1 {
                lights_map.get_mut(row as usize).map(|row| {
                    let value = row.get_mut(column as usize);
                    if let None = value {
                        panic!("out of scope")
                    }

                    if let Some(v) = value {
                        match instruction.0.as_str() {
                            "turn on" => {
                                *v = v.clone() + 1;
                            }
                            "turn off" => {
                                if v > &mut 0 {
                                    *v = v.clone() - 1;
                                }
                            }
                            "toggle" => {
                                *v = v.clone() + 2;
                            }
                            _ => panic!("unexpected instruction"),
                        }
                    }
                });
            }
        }
    }

    let mut amount_of_turned_on = 0;

    for row in lights_map.iter() {
        for value in row.iter() {
            amount_of_turned_on += value;
        }
    }

    Ok(amount_of_turned_on)
}
