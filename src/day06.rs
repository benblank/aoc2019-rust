use std::collections::HashMap;
use std::fs;
use std::io::BufRead;

const INPUT_PATH: &str = "day06.input.txt";

fn visit(nodes: &HashMap<String, Vec<String>>, name: &str, depth: u32) -> u32 {
    match nodes.get(name) {
        Some(children) => {
            depth
                + children
                    .iter()
                    .map(|child| visit(&nodes, child, depth + 1))
                    .sum::<u32>()
        }
        None => depth,
    }
}

pub fn part1() {
    let input = fs::read(INPUT_PATH).unwrap();
    let mut nodes = HashMap::new();

    for line in input.lines() {
        let line = line.unwrap();
        let mut parts = line.split(')').clone();
        let parent_name = parts.next().unwrap().to_owned();
        let node_name = parts.next().unwrap().to_owned();

        nodes
            .entry(parent_name)
            .or_insert_with(Vec::new)
            .push(node_name);
    }

    println!(
        "Total direct and indirect orbits: {}",
        visit(&nodes, &"COM".to_string(), 0)
    );
}
