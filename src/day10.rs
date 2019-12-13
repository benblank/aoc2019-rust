use std::collections::HashSet;
use std::fs;
use std::i32;

const INPUT_PATH: &str = "day10.input.txt";

fn get_abs_gcd(a: i32, b: i32) -> i32 {
    let a = i32::abs(a);
    let b = i32::abs(b);

    if b == 0 {
        a
    } else {
        get_abs_gcd(b, a % b)
    }
}

fn get_visible_asteroids(candidate: (i32, i32), asteroids: &HashSet<(i32, i32)>) -> usize {
    let mut visible_asteroid_vectors = HashSet::new();

    for asteroid in asteroids {
        if candidate == *asteroid {
            continue;
        }

        visible_asteroid_vectors.insert(simplify_vector(
            candidate.0 - asteroid.0,
            candidate.1 - asteroid.1,
        ));
    }

    visible_asteroid_vectors.len()
}

fn read_map(map: &[u8]) -> HashSet<(i32, i32)> {
    let mut asteroids = HashSet::new();

    for (y, line) in map.split(|c| *c == b'\n').enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c == b'#' {
                asteroids.insert((x as i32, y as i32));
            }
        }
    }

    asteroids
}

fn simplify_vector(x: i32, y: i32) -> (i32, i32) {
    let gcd = get_abs_gcd(x, y);

    (x / gcd, y / gcd)
}

pub fn part1() {
    let map = fs::read(INPUT_PATH).expect("could not read input");
    let asteroids = read_map(&map);
    let mut max_visible_asteroids = 0;

    for asteroid in &asteroids {
        let visible_asteroids = get_visible_asteroids(*asteroid, &asteroids);

        if visible_asteroids > max_visible_asteroids {
            max_visible_asteroids = visible_asteroids;
        }
    }

    println!("Best asteroid: {}", max_visible_asteroids);
}

#[cfg(test)]
mod tests {
    use super::*;
    use maplit::hashset;

    #[test]
    fn get_abs_gcd_works() {
        assert_eq!(3, get_abs_gcd(6, 9));
    }

    #[test]
    fn get_abs_gcd_works_when_a_is_negative() {
        assert_eq!(3, get_abs_gcd(-6, 9));
    }

    #[test]
    fn get_abs_gcd_works_when_b_is_negative() {
        assert_eq!(3, get_abs_gcd(6, -9));
    }

    #[test]
    fn get_abs_gcd_works_when_both_are_negative() {
        assert_eq!(3, get_abs_gcd(-6, -9));
    }

    #[test]
    fn get_abs_gcd_works_when_a_equals_b() {
        assert_eq!(2, get_abs_gcd(2, 2));
    }

    #[test]
    fn get_abs_gcd_works_when_a_is_multiple_of_b() {
        assert_eq!(2, get_abs_gcd(6, 2));
    }

    #[test]
    fn get_abs_gcd_works_when_b_is_multiple_of_a() {
        assert_eq!(2, get_abs_gcd(2, 6));
    }

    #[test]
    fn get_abs_gcd_works_when_no_common_factor() {
        assert_eq!(1, get_abs_gcd(2, 5));
    }

    #[test]
    fn get_visible_asteroids_works_1() {
        let map = b"......#.#.\n#..#.#....\n..#######.\n.#.#.###..\n.#..#.....\n..#....#.#\n#..#....#.\n.##.#..###\n##...#..#.\n.#....####";
        let asteroids = read_map(map);

        assert_eq!(33, get_visible_asteroids((5, 8), &asteroids));
    }

    #[test]
    fn get_visible_asteroids_works_2() {
        let map = b"#.#...#.#.\n.###....#.\n.#....#...\n##.#.#.#.#\n....#.#.#.\n.##..###.#\n..#...##..\n..##....##\n......#...\n.####.###.";
        let asteroids = read_map(map);

        assert_eq!(35, get_visible_asteroids((1, 2), &asteroids));
    }

    #[test]
    fn get_visible_asteroids_works_3() {
        let map = b".#..#..###\n####.###.#\n....###.#.\n..###.##.#\n##.##.#.#.\n....###..#\n..#.#..#.#\n#..#.#.###\n.##...##.#\n.....#.#..";
        let asteroids = read_map(map);

        assert_eq!(41, get_visible_asteroids((6, 3), &asteroids));
    }

    #[test]
    fn get_visible_asteroids_works_4() {
        let map = b".#..##.###...#######\n##.############..##.\n.#.######.########.#\n.###.#######.####.#.\n#####.##.#.##.###.##\n..#####..#.#########\n####################\n#.####....###.#.#.##\n##.#################\n#####.##.###..####..\n..######..##.#######\n####.##.####...##..#\n.#####..#.######.###\n##...#.##########...\n#.##########.#######\n.####.#.###.###.#.##\n....##.##.###..#####\n.#.#.###########.###\n#.#.#.#####.####.###\n###.##.####.##.#..##";
        let asteroids = read_map(map);

        assert_eq!(210, get_visible_asteroids((11, 13), &asteroids));
    }

    #[test]
    fn read_map_works() {
        let map = b".#..#\n.....\n#####\n....#\n...##";

        let asteroids = hashset! {
            (1, 0),
            (4, 0),
            (0, 2),
            (1, 2),
            (2, 2),
            (3, 2),
            (4, 2),
            (4, 3),
            (3, 4),
            (4, 4),
        };

        assert_eq!(asteroids, read_map(map));
    }

    #[test]
    fn simplify_vector_works() {
        assert_eq!((1, 2), simplify_vector(2, 4));
    }

    #[test]
    fn simplify_vector_works_when_a_is_negative() {
        assert_eq!((-1, 2), simplify_vector(-2, 4));
    }

    #[test]
    fn simplify_vector_works_when_b_is_negative() {
        assert_eq!((1, -2), simplify_vector(2, -4));
    }

    #[test]
    fn simplify_vector_works_when_both_are_negative() {
        assert_eq!((-1, -2), simplify_vector(-2, -4));
    }

    #[test]
    fn simplify_vector_works_when_no_simplification_is_possible() {
        assert_eq!((2, 5), simplify_vector(2, 5));
    }
}
