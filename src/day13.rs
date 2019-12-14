use crate::intcomp::{read_program, Intcomp};
use std::cmp::Ordering;
use std::collections::HashMap;

const INPUT_PATH: &str = "day13.input.txt";

struct Screen {
    tiles: HashMap<(i64, i64), Tile>,
    ball: (i64, i64),
    paddle: (i64, i64),
    score: i64,
}

impl Screen {
    fn count_tiles(&self, target: Tile) -> usize {
        self.tiles.values().filter(|tile| **tile == target).count()
    }

    fn new() -> Screen {
        Screen {
            tiles: HashMap::new(),
            ball: (-1, -1),
            paddle: (-1, -1),
            score: 0,
        }
    }

    fn update(&mut self, intcomp: &mut Intcomp) {
        while let Some(x) = intcomp.receive_output() {
            let y = intcomp.receive_output().expect("no y coordinate available");
            let tile_id = intcomp.receive_output().expect("no tile id availble");

            if (x, y) == (-1, 0) {
                self.score = tile_id;
            } else {
                let tile = Tile::from_tile_id(tile_id);

                if tile == Tile::Ball {
                    self.ball = (x, y);
                } else if tile == Tile::Paddle {
                    self.paddle = (x, y);
                }

                self.tiles.insert((x, y), tile);
            }
        }
    }
}

#[derive(PartialEq)]
enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

impl Tile {
    fn from_tile_id(tile_id: i64) -> Tile {
        match tile_id {
            0 => Tile::Empty,
            1 => Tile::Wall,
            2 => Tile::Block,
            3 => Tile::Paddle,
            4 => Tile::Ball,
            _ => panic!("invalid tile id '{}'", tile_id),
        }
    }
}

pub fn part1() {
    let initializer = read_program(INPUT_PATH);
    let mut intcomp = Intcomp::new(&initializer);
    let mut screen = Screen::new();

    intcomp.execute();
    screen.update(&mut intcomp);

    let block_count = screen.count_tiles(Tile::Block);

    println!("Block tile count: {}", block_count);
}

pub fn part2() {
    let initializer = read_program(INPUT_PATH);
    let mut intcomp = Intcomp::new(&initializer);
    let mut screen = Screen::new();

    intcomp.write_memory(0, 2);
    intcomp.execute();
    screen.update(&mut intcomp);

    while screen.count_tiles(Tile::Block) > 0 {
        intcomp.send_input(match screen.ball.0.cmp(&screen.paddle.0) {
            Ordering::Less => -1,
            Ordering::Equal => 0,
            Ordering::Greater => 1,
        });

        intcomp.execute();
        screen.update(&mut intcomp);
    }

    println!("Final score: {}", screen.score);
}
