use std::{error, fs::read_to_string};

pub fn solution_first_part() -> Result<usize, Box<dyn error::Error>> {
    let file_string = read_to_string("data")?.trim().to_string();
    let directions: Vec<String> = file_string
        .split("")
        .map(String::from)
        .filter(|v| !v.is_empty())
        .collect();

    let mut position = (0, 0);
    let mut houses_visited = Vec::new();

    for direction in directions.iter() {
        houses_visited.push(position.clone());
        // TODO: extract to function
        match direction.as_str() {
            ">" => position = (position.0, position.1 + 1),
            "v" => position = (position.0 - 1, position.1),
            "<" => position = (position.0, position.1 - 1),
            "^" => position = (position.0 + 1, position.1),
            _ => panic!("Wrong direction"),
        }
    }
    houses_visited.push(position.clone());

    let mut unique_houses: Vec<(i32, i32)> = vec![];

    for house_position in houses_visited.iter() {
        let house_to_find = unique_houses
            .iter()
            .find(|v| v.0 == house_position.0 && v.1 == house_position.1);

        if let None = house_to_find {
            unique_houses.push(*house_position);
        }
    }

    let amount_of_unique_houses = unique_houses.len();

    Ok(amount_of_unique_houses)
}

pub fn solution_second_part() -> Result<usize, Box<dyn error::Error>> {
    let file_string = read_to_string("data")?.trim().to_string();
    let directions: Vec<String> = file_string
        .split("")
        .map(String::from)
        .filter(|v| !v.is_empty())
        .collect();

    let mut houses_visited = Vec::new();

    let mut directions_santa: Vec<String> = vec![];
    // TODO: use some partitioning function instead
    for (index, direction) in directions.iter().enumerate() {
        if index % 2 == 1 {
            directions_santa.push(direction.clone())
        }
    }
    let mut position_santa = (0, 0);
    for direction in directions_santa.iter() {
        houses_visited.push(position_santa.clone());
        match direction.as_str() {
            ">" => position_santa = (position_santa.0, position_santa.1 + 1),
            "v" => position_santa = (position_santa.0 - 1, position_santa.1),
            "<" => position_santa = (position_santa.0, position_santa.1 - 1),
            "^" => position_santa = (position_santa.0 + 1, position_santa.1),
            _ => panic!("Wrong direction"),
        }
    }
    houses_visited.push(position_santa.clone());

    let mut directions_robo: Vec<String> = vec![];
    for (index, direction) in directions.iter().enumerate() {
        if index % 2 == 0 {
            directions_robo.push(direction.clone())
        }
    }
    let mut position_robo = (0, 0);
    for direction in directions_robo.iter() {
        houses_visited.push(position_robo.clone());
        match direction.as_str() {
            ">" => position_robo = (position_robo.0, position_robo.1 + 1),
            "v" => position_robo = (position_robo.0 - 1, position_robo.1),
            "<" => position_robo = (position_robo.0, position_robo.1 - 1),
            "^" => position_robo = (position_robo.0 + 1, position_robo.1),
            _ => panic!("Wrong direction"),
        }
    }
    houses_visited.push(position_robo.clone());

    let mut unique_houses: Vec<(i32, i32)> = vec![];
    for house_position in houses_visited.iter() {
        let house_to_find = unique_houses
            .iter()
            .find(|v| v.0 == house_position.0 && v.1 == house_position.1);

        if let None = house_to_find {
            unique_houses.push(*house_position);
        }
    }

    let amount_of_unique_houses = unique_houses.len();

    Ok(amount_of_unique_houses)
}
