use std::collections::HashSet;
use std::io::{self, BufRead};
use std::ops::AddAssign;

#[derive(Debug, PartialEq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

#[derive(Debug, PartialEq)]
struct Segment {
    direction: Direction,
    distance: i32,
}

fn draw_path(segments: &[Segment]) -> HashSet<Point> {
    let mut points = HashSet::new();
    let mut current = Point { x: 0, y: 0 };

    for segment in segments {
        let delta = match &segment.direction {
            Direction::Left => Point { x: -1, y: 0 },
            Direction::Right => Point { x: 1, y: 0 },
            Direction::Up => Point { x: 0, y: -1 },
            Direction::Down => Point { x: 0, y: 1 },
        };

        for _ in 0..segment.distance {
            current += delta;

            points.insert(current);
        }
    }

    points
}

fn get_manhattan_distance(p1: Point, p2: Point) -> i32 {
    (p1.x - p2.x).abs() + (p1.y - p2.y).abs()
}

fn parse_segments(source: &str) -> Vec<Segment> {
    source.split(',').map(|segment| {
        let direction = segment.chars().nth(0).unwrap();
        let distance = segment.chars().skip(1).collect::<String>();

        Segment {
            direction: match direction {
                'L' => Direction::Left,
                'R' => Direction::Right,
                'U' => Direction::Up,
                'D' => Direction::Down,
                _ => panic!("Invalid direction! ({})", direction),
            },
            distance: distance.parse::<i32>().unwrap(),
        }
    }).collect::<Vec<_>>()
}

pub fn part1() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let wires = handle.lines().collect::<Result<Vec<_>, _>>().unwrap();
    let segments1 = parse_segments(&wires[0]);
    let segments2 = parse_segments(&wires[1]);
    let path1 = draw_path(&segments1);
    let path2 = draw_path(&segments2);
    let intersections = path1.intersection(&path2);
    let min = intersections.map(|point| get_manhattan_distance(Point { x: 0, y: 0 }, *point)).min().unwrap();

    println!("Manhattan distance to closest intersection: {}", min);
}

#[cfg(test)]
mod tests {
    use maplit::hashset;
    use super::*;

    #[test]
    fn draw_path_works() {
        assert_eq!(hashset!{
            Point { x: 1, y: 0 },
            Point { x: 2, y: 0 },
            Point { x: 3, y: 0 },
            Point { x: 4, y: 0 },
            Point { x: 5, y: 0 },
            Point { x: 6, y: 0 },
            Point { x: 7, y: 0 },
            Point { x: 8, y: 0 },
            Point { x: 8, y: -1 },
            Point { x: 8, y: -2 },
            Point { x: 8, y: -3 },
            Point { x: 8, y: -4 },
            Point { x: 8, y: -5 },
            Point { x: 7, y: -5 },
            Point { x: 6, y: -5 },
            Point { x: 5, y: -5 },
            Point { x: 4, y: -5 },
            Point { x: 3, y: -5 },
            Point { x: 3, y: -4 },
            Point { x: 3, y: -3 },
            Point { x: 3, y: -2 },
        }, draw_path(&[
            Segment {
                direction: Direction::Right,
                distance: 8,
            },
            Segment {
                direction: Direction::Up,
                distance: 5,
            },
            Segment {
                direction: Direction::Left,
                distance: 5,
            },
            Segment {
                direction: Direction::Down,
                distance: 3,
            },
        ]));
    }

    #[test]
    fn get_manhattan_distance_works() {
        let p1 = Point { x: 1, y: 2 };
        let p2 = Point { x: -5, y: 21 };

        assert_eq!(25, get_manhattan_distance(p1, p2));
    }

    #[test]
    fn parse_segments_works() {
        assert_eq!(vec![
            Segment {
                direction: Direction::Right,
                distance: 8,
            },
            Segment {
                direction: Direction::Up,
                distance: 5,
            },
            Segment {
                direction: Direction::Left,
                distance: 5,
            },
            Segment {
                direction: Direction::Down,
                distance: 3,
            },
        ], parse_segments(&"R8,U5,L5,D3".to_string()));
    }
}
