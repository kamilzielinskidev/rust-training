use std::{
    error::Error,
    fs::{read_to_string, write},
    io::Write,
};

fn read_data() -> Result<String, Box<dyn Error>> {
    read_to_string("data").map_err(|v| v.into())
}

fn split_by_same_value(v: &String) -> Vec<String> {
    v.chars()
        .fold(vec![], |mut acc, next| match acc.last_mut() {
            None => {
                acc.push(String::from(next));
                return acc;
            }
            Some(last_value) => {
                let last_character = last_value.chars().last().unwrap();
                if last_character == next {
                    last_value.push(last_character);
                } else {
                    acc.push(String::from(next))
                }
                return acc;
            }
        })
}

fn look_n_say_partial(v: &String) -> String {
    let length = v.len();
    let value = v.chars().nth(0).unwrap();
    return length.to_string() + value.to_string().as_str();
}

fn look_n_say(v: &String) -> String {
    split_by_same_value(v)
        .iter()
        .map(look_n_say_partial)
        .collect::<Vec<String>>()
        .join("")
}

pub fn solution() -> Result<String, Box<dyn Error>> {
    let raw_data = read_data()?;
    let mut cloned = raw_data.clone();

    for _ in 0..50 {
        cloned = look_n_say(&cloned);
    }

    return Ok(cloned.len().to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_by_same_value() {
        assert_eq!(
            split_by_same_value(&("111322".into())),
            vec![String::from("111"), String::from("3"), String::from("22")]
        );
    }

    #[test]
    fn test_look_n_say_partial() {
        assert_eq!(look_n_say_partial(&("111".into())), String::from("31"));
    }

    #[test]
    fn test_look_n_say() {
        assert_eq!(look_n_say(&("111322".into())), String::from("311322"));
    }
}
