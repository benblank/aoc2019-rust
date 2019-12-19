use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};
use std::fs;

const DIRECTIONS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
const INPUT_PATH: &str = "day18.input.txt";

fn get_edges(map: &[&[u8]], start: (usize, usize)) -> HashMap<u8, (u32, Vec<u8>)> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut edges = HashMap::new();

    queue.push_back((start, 0, Vec::new()));

    while let Some((current, distance, keys_needed)) = queue.pop_front() {
        let code = map[current.1][current.0];
        let mut keys_needed = keys_needed;

        if (b'a'..=b'z').contains(&code) && distance > 0 {
            edges.insert(code, (distance, keys_needed.clone()));
        } else if (b'A'..=b'Z').contains(&code) {
            keys_needed = {
                let mut keys_needed = keys_needed.clone();

                keys_needed.push(
                    (code as char)
                        .to_lowercase()
                        .next()
                        .expect("CAN'T HAPPEN - no lowercase available") as u8,
                );

                keys_needed
            };
        }

        visited.insert(current);

        for direction in DIRECTIONS.iter() {
            let target = (
                (current.0 as i32 + direction.0) as usize,
                (current.1 as i32 + direction.1) as usize,
            );

            if map[target.1][target.0] != b'#' && !visited.contains(&target) {
                queue.push_back((target, distance + 1, keys_needed.to_owned()));
            }
        }
    }

    edges
}

fn get_points_of_interest(map: &[&[u8]]) -> HashMap<u8, (usize, usize)> {
    let mut points_of_interest = HashMap::new();

    for (y, row) in map.iter().enumerate() {
        for (x, code) in row.iter().enumerate() {
            match code {
                b'@' | b'a'..=b'z' => {
                    points_of_interest.insert(*code, (x, y));
                }
                _ => {}
            }
        }
    }

    points_of_interest
}

fn get_routes(
    edges: &HashMap<u8, HashMap<u8, (u32, Vec<u8>)>>,
) -> HashMap<(u8, BTreeSet<u8>), u32> {
    let mut routes = HashMap::new();
    let all_keys = {
        let mut all_keys = edges
            .keys()
            .filter_map(|key| if *key == b'@' { None } else { Some(*key) })
            .collect::<Vec<_>>();

        all_keys.sort();

        all_keys
    };

    routes.insert((b'@', BTreeSet::new()), 0);

    for _ in &all_keys {
        let mut next_routes = HashMap::new();

        for ((code, keys_owned), current_distance) in routes {
            for key in &all_keys {
                if !keys_owned.contains(key) {
                    let (distance, keys_needed) = edges
                        .get(&code)
                        .unwrap_or_else(|| panic!("code '{}' not found in edges", code))
                        .get(&key)
                        .unwrap_or_else(|| {
                            panic!("key '{}' not found in edges for code '{}'", key, code)
                        });

                    if keys_needed.iter().all(|key| keys_owned.contains(key)) {
                        let next_distance = current_distance + distance;
                        let mut next_keys = keys_owned.clone();

                        next_keys.insert(*key);

                        next_routes
                            .entry((*key, next_keys))
                            .and_modify(|distance| *distance = next_distance)
                            .or_insert(next_distance);
                    }
                }
            }
        }

        routes = next_routes;
    }

    routes
}

pub fn part1() {
    let input = fs::read(INPUT_PATH).expect("could not read input");
    let map = input.split(|tile| *tile == b'\n').collect::<Vec<_>>();
    let points_of_interest = get_points_of_interest(&map);

    let edges = points_of_interest
        .iter()
        .map(|(code, coords)| (*code, get_edges(&map, *coords)))
        .collect::<HashMap<_, _>>();

    let routes = get_routes(&edges);

    println!(
        "Shortest path: {}",
        routes.values().min().expect("CAN'T HAPPEN - no results")
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use maplit::{btreeset, hashmap};

    #[test]
    fn get_edges_works_1() {
        let expected = hashmap! {
            b'a' => (2, Vec::new()),
            b'b' => (4, vec![b'a']),
        };

        let actual = get_edges(
            &[&b"#########"[..], &b"#b.A.@.a#"[..], &b"#########"[..]],
            (5, 1),
        );

        assert_eq!(expected, actual);
    }

    #[test]
    fn get_edges_works_2() {
        let expected = hashmap! {
            b'b' => (6, vec![b'a']),
        };

        let actual = get_edges(
            &[&b"#########"[..], &b"#b.A.@.a#"[..], &b"#########"[..]],
            (7, 1),
        );

        assert_eq!(expected, actual);
    }

    #[test]
    fn get_edges_works_3() {
        let expected = hashmap! {
            b'a' => (6, vec![b'a']),
        };

        let actual = get_edges(
            &[&b"#########"[..], &b"#b.A.@.a#"[..], &b"#########"[..]],
            (1, 1),
        );

        assert_eq!(expected, actual);
    }

    #[test]
    fn get_points_of_interest_works() {
        let expected = hashmap! {
            b'@' => (5, 1),
            b'a' => (7, 1),
            b'b' => (1, 1),
        };

        let actual =
            get_points_of_interest(&[&b"#########"[..], &b"#b.A.@.a#"[..], &b"#########"[..]]);

        assert_eq!(expected, actual);
    }

    #[test]
    fn get_routes_works_1() {
        let expected = hashmap! {
            (b'b', btreeset! {b'a', b'b'}) => 8
        };

        let actual = get_routes(&hashmap! {
            b'@' => hashmap! {
                b'a' => (2, Vec::new()),
                b'b' => (4, vec![b'a']),
            },

            b'a' => hashmap! {
                b'b' => (6, vec![b'a']),
            },

            b'b' => hashmap! {
                b'a' => (6, vec![b'a']),
            },
        });

        assert_eq!(expected, actual);
    }

    #[test]
    fn get_routes_works_2() {
        let expected = hashmap! {
            (b'b', btreeset! {b'a', b'b'}) => 8
        };

        let map = vec![
            &b"#################"[..],
            &b"#i.G..c...e..H.p#"[..],
            &b"########.########"[..],
            &b"#j.A..b...f..D.o#"[..],
            &b"########@########"[..],
            &b"#k.E..a...g..B.n#"[..],
            &b"########.########"[..],
            &b"#l.F..d...h..C.m#"[..],
            &b"#################"[..],
        ];

        let points_of_interest = get_points_of_interest(&map);

        let edges = points_of_interest
            .iter()
            .map(|(code, coords)| (*code, get_edges(&map, *coords)))
            .collect::<HashMap<_, _>>();

        let actual = get_routes(&edges);

        println!(
            "Shortest path: {}",
            actual.values().min().expect("CAN'T HAPPEN - no results")
        );

        assert_eq!(expected, actual);
    }
}
