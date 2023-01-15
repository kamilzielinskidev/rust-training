use std::{cmp, error, fs::read_to_string};

struct Dimensions(usize, usize, usize);

fn get_dimensions(dimensions_string: &String) -> Result<Dimensions, Box<dyn error::Error>> {
    let parsed_dimensions: Result<Vec<usize>, _> =
        dimensions_string.split("x").map(|v| v.parse()).collect();
    let ok_parsed_dimensions = parsed_dimensions?;

    let first = ok_parsed_dimensions.get(0);
    let second = ok_parsed_dimensions.get(1);
    let third = ok_parsed_dimensions.get(2);

    match (first, second, third) {
        (Some(a), Some(b), Some(c)) => Ok(Dimensions(*a, *b, *c)),
        _ => panic!(""),
    }
}

fn get_box_size(dimensions: &Dimensions) -> usize {
    let first = dimensions.0 * dimensions.1;
    let second = dimensions.0 * dimensions.2;
    let third = dimensions.1 * dimensions.2;

    2 * first + 2 * second + 2 * third
}

fn get_slack_size(dimensions: &Dimensions) -> usize {
    let first = dimensions.0 * dimensions.1;
    let second = dimensions.0 * dimensions.2;
    let third = dimensions.1 * dimensions.2;

    cmp::min(first, cmp::min(second, third))
}

pub fn solution_first_part() -> Result<usize, Box<dyn error::Error>> {
    let file_string: Vec<String> = read_to_string("data")?
        .trim()
        .to_string()
        .split_ascii_whitespace()
        .map(String::from)
        .collect();

    let dimensions_map: Result<Vec<usize>, Box<dyn error::Error>> = file_string
        .iter()
        .map(get_dimensions)
        .map(|dimensions| {
            let dimensions_ok = dimensions?;
            Ok(get_box_size(&dimensions_ok) + get_slack_size(&dimensions_ok))
        })
        .collect();

    let paper_size = dimensions_map?.iter().fold(0, |acc, next| acc + next);

    println!("{:?}", paper_size);
    Ok(1)
}

fn get_box_perimeter(dimensions: &Dimensions) -> usize {
    let mut dimensions_vec = Vec::new();

    dimensions_vec.push(dimensions.0);
    dimensions_vec.push(dimensions.1);
    dimensions_vec.push(dimensions.2);

    dimensions_vec.sort();

    let first = dimensions_vec[0];
    let second = dimensions_vec[1];

    2 * first + 2 * second
}

fn get_ribbon_size(dimensions: &Dimensions) -> usize {
    dimensions.0 * dimensions.1 * dimensions.2
}

pub fn solution_second_part() -> Result<usize, Box<dyn error::Error>> {
    let file_string: Vec<String> = read_to_string("data")?
        .trim()
        .to_string()
        .split_ascii_whitespace()
        .map(String::from)
        .collect();

    let dimensions_map: Result<Vec<usize>, Box<dyn error::Error>> = file_string
        .iter()
        .map(get_dimensions)
        .map(|dimensions| {
            let dimensions_ok = dimensions?;
            Ok(get_box_perimeter(&dimensions_ok) + get_ribbon_size(&dimensions_ok))
        })
        .collect();

    let paper_size = dimensions_map?.iter().fold(0, |acc, next| acc + next);

    println!("{:?}", paper_size);
    Ok(paper_size)
}
