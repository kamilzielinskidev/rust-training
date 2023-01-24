use regex::Regex;
use std::{collections::HashMap, error::Error, fs::read_to_string};

trait IsNumeric {
    fn is_numeric(&self) -> bool;
}

impl IsNumeric for String {
    fn is_numeric(&self) -> bool {
        self.parse::<i32>().is_ok()
    }
}

pub fn solution_first_part() -> Result<usize, Box<dyn Error>> {
    let mut instructions = read_to_string("data")?
        .lines()
        .map(|v| v.to_string())
        .collect::<Vec<_>>();

    let mut values: HashMap<String, i32> = HashMap::new();

    loop {
        println!("{}", instructions.len());
        if instructions.len() == 0 {
            break;
        }
        if values.contains_key(&"a".to_string()) {
            break;
        }
        for (i, instruction_string) in instructions.iter().enumerate() {
            let matches = Regex::new(r"(\w+)")?
                .find_iter(&instruction_string)
                .map(|v| v.as_str())
                .collect::<Vec<_>>();

            if instruction_string.contains("RSHIFT") || instruction_string.contains("LSHIFT") {
                let v = matches.get(0).ok_or("")?;
                let by = matches.get(2).ok_or("").map(|v| v.parse::<i32>())??;
                let to = matches.get(3).ok_or("")?;

                if v.to_string().is_numeric() {
                    if instruction_string.contains("RSHIFT") {
                        values.insert(to.to_string(), v.parse::<i32>()? >> by);
                    } else {
                        values.insert(to.to_string(), v.parse::<i32>()? << by);
                    }
                    instructions.remove(i);
                    break;
                } else if values.contains_key(&v.to_string()) {
                    let found_value = values.get(&v.to_string()).ok_or("")?;
                    if instruction_string.contains("RSHIFT") {
                        values.insert(to.to_string(), found_value >> by);
                    } else {
                        values.insert(to.to_string(), found_value << by);
                    }
                    instructions.remove(i);
                    break;
                } else {
                    continue;
                }
            } else if instruction_string.contains("OR") || instruction_string.contains("AND") {
                let v1 = matches.get(0).ok_or("")?.to_string();
                let v2 = matches.get(2).ok_or("")?.to_string();
                let to = matches.get(3).ok_or("")?.to_string();

                if v1.is_numeric() && v2.is_numeric() {
                    if instruction_string.contains("OR") {
                        values.insert(to, v1.parse::<i32>()? | v2.parse::<i32>()?);
                    } else {
                        values.insert(to, v1.parse::<i32>()? & v2.parse::<i32>()?);
                    }
                    instructions.remove(i);
                    break;
                } else if v1.is_numeric() && values.contains_key(&v2) {
                    if instruction_string.contains("OR") {
                        values.insert(to, v1.parse::<i32>()? | values.get(&v2).ok_or("")?);
                    } else {
                        values.insert(to, v1.parse::<i32>()? & values.get(&v2).ok_or("")?);
                    }
                    instructions.remove(i);
                    break;
                } else if v2.is_numeric() && values.contains_key(&v1) {
                    if instruction_string.contains("OR") {
                        values.insert(to, v2.parse::<i32>()? | values.get(&v1).ok_or("")?);
                    } else {
                        values.insert(to, v2.parse::<i32>()? & values.get(&v1).ok_or("")?);
                    }
                    instructions.remove(i);
                    break;
                } else if values.contains_key(&v1) && values.contains_key(&v2) {
                    if instruction_string.contains("OR") {
                        values.insert(to, values.get(&v1).ok_or("")? | values.get(&v2).ok_or("")?);
                    } else {
                        values.insert(to, values.get(&v1).ok_or("")? & values.get(&v2).ok_or("")?);
                    }
                    instructions.remove(i);
                    break;
                } else {
                    continue;
                }
            } else if instruction_string.contains("NOT") {
                let v1 = matches.get(1).ok_or("")?.to_string();
                let to = matches.get(2).ok_or("")?.to_string();

                if v1.is_numeric() {
                    values.insert(to, 65536 - !v1.parse::<i32>()?);
                    instructions.remove(i);
                    break;
                } else if values.contains_key(&v1) {
                    values.insert(to, 65536 - !values.get(&v1).ok_or("")?);
                    instructions.remove(i);
                    break;
                } else {
                    continue;
                }
            } else if instruction_string.contains("->") {
                let v1 = matches.get(0).ok_or("")?.to_string();
                let to = matches.get(1).ok_or("")?.to_string();
                if v1.is_numeric() {
                    values.insert(to, v1.parse::<i32>()?);
                    instructions.remove(i);
                    break;
                } else if values.contains_key(&v1) {
                    values.insert(to, *values.get(&v1).ok_or("")?);
                    instructions.remove(i);
                    break;
                } else {
                    continue;
                }
            }
        }
    }

    values.get("a").map(|v| *v as usize).ok_or("".into())
}
