use regex::Regex;
use std::collections::HashMap;
use std::{error::Error, fs::read_to_string};

type Graph = HashMap<String, HashMap<String, i32>>;

fn read_data() -> Result<Vec<String>, Box<dyn Error>> {
    Ok(read_to_string("data")?
        .lines()
        .map(|v| v.to_string())
        .collect::<Vec<_>>())
}

pub type Route = (String, String, i32);

fn parse_to_route<'a>(v: &'a str) -> Result<Route, Box<dyn Error>> {
    let caps = Regex::new(r"(\w+)")?.find_iter(v).collect::<Vec<_>>();

    let from = caps.get(0).ok_or("no from")?.as_str().to_string();
    let to = caps.get(2).ok_or("no to")?.as_str().to_string();
    let distance = caps.get(3).ok_or("no distance")?.as_str().parse::<i32>()?;
    Ok((from, to, distance))
}

pub fn collect_to_graph(routes: &Vec<Route>) -> Graph {
    let mut graph: Graph = HashMap::new();
    for route in routes {
        match graph.get_mut(&route.0.to_string()) {
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
        match graph.get_mut(&route.1.to_string()) {
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

pub fn solution() -> Result<i32, Box<dyn Error>> {
    let graph: Graph = read_data()
        .and_then(|line| {
            line.iter()
                .map(|v| parse_to_route(v))
                .collect::<Result<Vec<_>, _>>()
                .map(|v| collect_to_graph(&v))
        })
        .unwrap_or(HashMap::new());
    let paths = find_paths_distances(&graph, vec![], 0);

    let result = paths.last().ok_or("cannot get result")?;

    Ok(*result)
}

fn find_paths_distances(graph: &Graph, been: Vec<String>, distance: i32) -> Vec<i32> {
    if graph.keys().len() == been.len() {
        return vec![distance];
    }

    let mut paths = graph
        .keys()
        .filter(|v| !been.contains(v))
        .flat_map(|city| {
            let last_visit = been.last();

            match last_visit {
                None => find_paths_distances(graph, vec![city.clone()], 0),
                Some(last_visit) => {
                    let mut been_copy = been.clone();
                    been_copy.push(city.clone());

                    let inner_distance = graph
                        .get(city)
                        .map(|v| v.get(last_visit))
                        .flatten()
                        .ok_or("cannot find distance")
                        .unwrap();

                    find_paths_distances(graph, been_copy, distance + inner_distance)
                }
            }
        })
        .collect::<Vec<_>>();
    paths.sort();
    paths
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collect_to_graph() {
        let graph: Graph = HashMap::from([
            ("A".into(), HashMap::from([("B".into(), 12)])),
            ("B".into(), HashMap::from([("A".into(), 12)])),
        ]);

        assert_eq!(collect_to_graph(&vec![("A".into(), "B".into(), 12)]), graph);
    }

    #[test]
    fn test_a() {
        let graph: Graph = HashMap::from([
            (
                "Dublin".into(),
                HashMap::from([("London".into(), 464), ("Belfast".into(), 141)]),
            ),
            (
                "London".into(),
                HashMap::from([("Belfast".into(), 518), ("Dublin".into(), 464)]),
            ),
            (
                "Belfast".into(),
                HashMap::from([("London".into(), 518), ("Dublin".into(), 141)]),
            ),
        ]);

        assert_eq!(
            find_paths_distances(&graph, vec![], 0),
            vec![605, 605, 659, 659, 982, 982]
        );
    }
}
