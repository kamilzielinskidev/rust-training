#![feature(int_roundings)]
use std::{error, io};

fn median(arr: &Vec<usize>) -> usize {
    let len = arr.len();
    let is_even = len % 2 == 0;
    let mut copy = arr.to_vec();
    copy.sort();
    *match is_even {
        true => copy.get(len.div_floor(2)),
        false => copy.get(len.div_ceil(2)),
    }
    .unwrap()
}

fn pig_latin(word: String) -> String {
    let first_letter = word.get(0..1).unwrap();
    let mut copy = word.clone();

    match first_letter {
        "a" => copy.push_str("-hay"),
        "e" => copy.push_str("-hay"),
        "i" => copy.push_str("-hay"),
        "o" => copy.push_str("-hay"),
        "u" => copy.push_str("-hay"),
        "y" => copy.push_str("-hay"),
        other => {
            copy = copy.get(1..).unwrap().to_string();
            copy.push_str("-");
            copy.push_str(other);
            copy.push_str("ay");
        }
    }

    return copy;
}

fn first() {
    let list_of_integers = [7, 1, 1, 2, 3, 3, 4, 3, 5, 6, 11];
    println!("{}", median(&list_of_integers.to_vec()));
}

fn second() {
    let test1 = "first";
    let test2 = "apple";
    println!("{}", pig_latin(String::from(test1)));
    println!("{}", pig_latin(String::from(test2)));
}

fn command() -> Result<(String, Option<String>), Box<dyn error::Error>> {
    println!("Please enter a command:");

    let mut user_command = String::new();
    io::stdin().read_line(&mut user_command)?;

    let splitted_command = user_command.split_ascii_whitespace().collect::<Vec<_>>();

    let user_command_command_option = splitted_command.get(0);
    let user_command_user_option = splitted_command.get(1);

    user_command_command_option.map_or_else(
        || panic!("Cannot read some value."),
        |v1| {
            return Ok((
                v1.to_string(),
                user_command_user_option.map(|v| v.to_string()),
            ));
        },
    )
}

fn third() {
    let mut teams: Vec<String> = Vec::new();

    loop {
        match command() {
            Err(err) => panic!("{}", err),
            Ok((command_command, command_user)) => match command_command.as_str() {
                "add" => teams.push(command_user.unwrap()),
                "list" => {
                    println!("{:?}", teams)
                }
                _ => panic!("Something wrong with command."),
            },
        }
    }
}

fn main() {
    first();
    second();
    third();
}
