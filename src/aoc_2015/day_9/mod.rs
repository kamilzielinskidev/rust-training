use regex::Regex;
use std::collections::HashMap;
use std::{error::Error, fs::read_to_string};

fn read_data() -> Result<Vec<String>, Box<dyn Error>> {
    Ok(read_to_string("data")
        .expect("no data file")
        .lines()
        .map(|v| v.to_string())
        .collect::<Vec<_>>())
}

#[derive(Debug)]
pub struct Route(String, String, i32);

fn parse_to_route(v: &String) -> Result<Route, Box<dyn Error>> {
    let caps = Regex::new(r"(\w+)")?
        .find_iter(v.as_str())
        .collect::<Vec<_>>();

    let from = caps.get(0).ok_or("no from")?.as_str().to_string();
    let to = caps.get(2).ok_or("no to")?.as_str().to_string();
    let distance = caps.get(3).ok_or("no distance")?.as_str().parse::<i32>()?;
    Ok(Route(from, to, distance))
}

pub fn collect_to_graph(routes: &Vec<Route>) -> HashMap<String, HashMap<String, i32>> {
    let mut graph: HashMap<String, HashMap<String, i32>> = HashMap::new();
    for route in routes {
        match graph.get_mut(&route.0) {
            None => {
                graph.insert(
                    route.0.to_string(),
                    HashMap::from([(route.1.to_string(), route.2)]),
                );
            }
            Some(a) => {
                a.insert(route.1.to_string(), route.2);
            }
        }
        match graph.get_mut(&route.1) {
            None => {
                graph.insert(
                    route.1.to_string(),
                    HashMap::from([(route.0.to_string(), route.2)]),
                );
            }
            Some(a) => {
                a.insert(route.0.to_string(), route.2);
            }
        }
    }

    graph
}

pub fn solution() {
    read_data()
        .and_then(|line| {
            line.iter()
                .map(|v| parse_to_route(v))
                .collect::<Result<Vec<_>, _>>()
        })
        .map(|v| collect_to_graph(&v))
        .inspect(|v| println!("{:?}", v))
        .unwrap_or(HashMap::new());

    return ();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collect_to_graph() {
        let graph: HashMap<String, HashMap<String, i32>> = HashMap::from([
            ("A".into(), HashMap::from([("B".into(), 12)])),
            ("B".into(), HashMap::from([("A".into(), 12)])),
        ]);

        assert_eq!(
            collect_to_graph(&vec![Route("A".into(), "B".into(), 12)]),
            graph
        );
    }
}
