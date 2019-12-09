use std::collections::HashMap;
use std::fs;
use std::io::BufRead;

const INPUT_PATH: &str = "day06.input.txt";

fn count_orbits(nodes: &HashMap<String, Vec<String>>, name: &str, depth: u32) -> u32 {
    match nodes.get(name) {
        Some(children) => {
            depth
                + children
                    .iter()
                    .map(|child| count_orbits(&nodes, child, depth + 1))
                    .sum::<u32>()
        }
        None => depth,
    }
}

pub fn part1() {
    let input = fs::read(INPUT_PATH).unwrap();
    let mut children = HashMap::new();

    for line in input.lines() {
        let line = line.unwrap();
        let mut parts = line.split(')');
        let parent_name = parts.next().unwrap().to_owned();
        let node_name = parts.next().unwrap().to_owned();

        children
            .entry(parent_name)
            .or_insert_with(Vec::new)
            .push(node_name);
    }

    println!(
        "Total direct and indirect orbits: {}",
        count_orbits(&children, &"COM".to_string(), 0)
    );
}

pub fn part2() {
    let input = fs::read(INPUT_PATH).unwrap();
    let mut parents = HashMap::new();

    for line in input.lines() {
        let line = line.unwrap();
        let mut parts = line.split(')');
        let parent_name = parts.next().unwrap().to_owned();
        let node_name = parts.next().unwrap().to_owned();

        parents.entry(node_name).or_insert(parent_name);
    }

    let mut current = "YOU".to_string();
    let mut distance = 0;
    let mut distances = HashMap::new();

    while current != "COM" {
        let parent = parents.get(&current).unwrap();

        distances.insert(parent, distance);
        distance += 1;
        current = parent.to_owned();
    }

    current = "SAN".to_string();
    distance = 0;

    while !distances.contains_key(&current) {
        distance += 1;
        current = parents.get(&current).unwrap().to_owned();
    }

    println!(
        "Total transfers: {}",
        distance + distances.get(&current).unwrap() - 1
    );
}
