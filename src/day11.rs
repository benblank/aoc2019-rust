use crate::intcomp::{read_program, Intcomp};
use std::collections::HashMap;
use std::i32;

const INPUT_PATH: &str = "day11.input.txt";

fn paint_hull(hull: &mut HashMap<(i32, i32), i64>, start_panel_color: i64) {
    hull.insert((0, 0), start_panel_color);

    let initializer = read_program(INPUT_PATH);
    let mut intcomp = Intcomp::new(&initializer);
    let mut position = (0, 0);
    let mut facing = (0, -1); // "Up".

    loop {
        let input = if hull.contains_key(&position) {
            *hull.get(&position).unwrap()
        } else {
            0
        };

        intcomp.send_input(input);
        intcomp.execute();

        if let Some(color) = intcomp.receive_output() {
            let direction = intcomp
                .receive_output()
                .expect("no turn direction supplied");

            hull.insert(position, color);

            facing = if direction == 0 {
                turn_left(facing)
            } else {
                turn_right(facing)
            };

            position = (position.0 + facing.0, position.1 + facing.1);
        } else {
            break;
        }
    }
}

fn turn_left(facing: (i32, i32)) -> (i32, i32) {
    match facing {
        (-1, 0) => (0, 1),
        (0, -1) => (-1, 0),
        (0, 1) => (1, 0),
        (1, 0) => (0, -1),
        _ => panic!("invalid facing {:?}", facing),
    }
}

fn turn_right(facing: (i32, i32)) -> (i32, i32) {
    match facing {
        (-1, 0) => (0, -1),
        (0, -1) => (1, 0),
        (0, 1) => (-1, 0),
        (1, 0) => (0, 1),
        _ => panic!("invalid facing {:?}", facing),
    }
}

pub fn part1() {
    let mut hull = HashMap::new();

    paint_hull(&mut hull, 0);

    println!("Total panels painted: {}", hull.len());
}

pub fn part2() {
    let mut hull = HashMap::new();

    paint_hull(&mut hull, 1);

    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;

    for (x, y) in hull.keys() {
        if *x < min_x {
            min_x = *x;
        } else if *x > max_x {
            max_x = *x;
        }

        if *y < min_y {
            min_y = *y;
        } else if *y > max_y {
            max_y = *y;
        }
    }

    let mut image = Vec::new();

    for _ in min_y..=max_y {
        image.push(vec![0; (max_x - min_x + 1) as usize]);
    }

    for ((x, y), color) in hull.iter() {
        image[*y as usize][*x as usize] = *color;
    }

    for row in image {
        println!(
            "{}",
            row.iter()
                .map(|number| if *number == 0 { " " } else { "#" })
                .collect::<Vec<_>>()
                .join("")
        );
    }
}
