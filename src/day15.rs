use crate::intcomp::{read_program, Intcomp};
use std::collections::HashMap;
use std::i32;
use std::slice::Iter;
use std::u32;

const INPUT_PATH: &str = "day15.input.txt";

#[derive(Clone, Copy)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn get_delta(self) -> (i32, i32) {
        match self {
            Direction::North => (0, -1),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
            Direction::East => (1, 0),
        }
    }

    fn get_input(self) -> i64 {
        match self {
            Direction::North => 1,
            Direction::South => 2,
            Direction::West => 3,
            Direction::East => 4,
        }
    }

    fn iter() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 4] = [
            Direction::North,
            Direction::South,
            Direction::West,
            Direction::East,
        ];

        DIRECTIONS.iter()
    }
}

pub fn part1() {
    let initializer = read_program(INPUT_PATH);
    let mut intcomp = Intcomp::new(&initializer);
    let mut map = HashMap::new();
    let mut coords = (0, 0);
    let mut distance = 0;

    map.insert(coords, (1, distance));

    loop {
        let mut next = (Direction::North, (i32::MIN, i32::MIN));

        for direction in Direction::iter() {
            let delta = direction.get_delta();
            let candidate = (coords.0 + delta.0, coords.1 + delta.1);

            if !map.contains_key(&candidate) {
                next = (*direction, candidate);

                break;
            }
        }

        if next.1 == (i32::MIN, i32::MIN) {
            let mut candidates = Direction::iter()
                .map(|direction| {
                    let delta = direction.get_delta();
                    let candidate = (coords.0 + delta.0, coords.1 + delta.1);

                    (
                        map.get(&candidate)
                            .expect("CAN'T HAPPEN - candidate not in map")
                            .1,
                        (*direction, candidate),
                    )
                })
                .collect::<Vec<_>>();

            candidates.sort_by(|&(distance_a, _), (distance_b, _)| distance_a.cmp(distance_b));

            next = candidates[0].1;
        }

        intcomp.send_input(next.0.get_input());
        intcomp.execute();

        let candidate = next.1;
        let status = intcomp.receive_output().expect("no status code received");

        map.entry(candidate)
            .or_insert((status, if status == 0 { u32::MAX } else { distance + 1 }));

        if status != 0 {
            coords = candidate;
            distance = map
                .get(&coords)
                .expect("CAN'T HAPPEN - coords not in map")
                .1;
        }

        if status == 2 {
            break;
        }
    }

    println!("Found oxygen system in {} steps.", distance);
}
