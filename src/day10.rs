use std::collections::HashSet;
use std::f64;
use std::fs;
use std::i32;

const HEIGHT: i32 = 33;
const INPUT_PATH: &str = "day10.input.txt";
const WIDTH: i32 = 33;

fn get_abs_gcd(a: i32, b: i32) -> i32 {
    let a = i32::abs(a);
    let b = i32::abs(b);

    if b == 0 {
        a
    } else {
        get_abs_gcd(b, a % b)
    }
}

fn get_tracking_station(asteroids: &HashSet<(i32, i32)>) -> ((i32, i32), usize) {
    let mut max_visible_asteroids = 0;
    let mut tracking_station = (-1, -1);

    for candidate in asteroids {
        let visible_asteroids = get_visible_asteroid_vectors(*candidate, &asteroids).len();

        if visible_asteroids > max_visible_asteroids {
            max_visible_asteroids = visible_asteroids;
            tracking_station = *candidate;
        }
    }

    (tracking_station, max_visible_asteroids)
}

fn get_visible_asteroid_vectors(
    candidate: (i32, i32),
    asteroids: &HashSet<(i32, i32)>,
) -> HashSet<(i32, i32)> {
    let mut visible_asteroid_vectors = HashSet::new();

    for asteroid in asteroids {
        if candidate == *asteroid {
            continue;
        }

        visible_asteroid_vectors.insert(simplify_vector(
            asteroid.0 - candidate.0,
            asteroid.1 - candidate.1,
        ));
    }

    visible_asteroid_vectors
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
    let ((_, _), max_visible_asteroids) = get_tracking_station(&asteroids);

    println!("Best asteroid: {}", max_visible_asteroids);
}

pub fn part2() {
    let map = fs::read(INPUT_PATH).expect("could not read input");
    let mut asteroids = read_map(&map);

    // Otherise, the loop below will never terminate.
    if asteroids.len() < 200 {
        panic!(
            "Cannot find 200th asteroid when there are only {} asteroids",
            asteroids.len()
        );
    }

    let (tracking_station, _) = get_tracking_station(&asteroids);

    // Otherwise, the tracking station blocks the first shot.
    asteroids.remove(&tracking_station);

    let vectors = get_visible_asteroid_vectors(tracking_station, &asteroids);

    let mut phase_angles = vectors
        .iter()
        .map(|(x, y)| {
            let atan2 = f64::from(*y).atan2(f64::from(*x));

            // Arctangent calculates from the x axis, so rotate to y.
            let mut phase_angle = atan2 + f64::consts::FRAC_PI_2;

            // Wrap negative values around so that 0 comes first.
            while phase_angle < 0.0 {
                phase_angle += 2.0 * f64::consts::PI;
            }

            (phase_angle, (x, y))
        })
        .collect::<Vec<_>>();

    phase_angles.sort_by(|(phase_angle1, _), (phase_angle2, _)| {
        phase_angle1
            .partial_cmp(phase_angle2)
            .expect("CAN'T HAPPEN - non-finite phase angle")
    });

    let mut destroyed = 0;

    while destroyed < 200 {
        for (_, vector) in &phase_angles {
            let mut scalar = 1;

            loop {
                let scaled_vector = (vector.0 * scalar, vector.1 * scalar);
                let candidate = (
                    tracking_station.0 + scaled_vector.0,
                    tracking_station.1 + scaled_vector.1,
                );

                if candidate.0 < 0 || candidate.0 > WIDTH || candidate.1 < 0 || candidate.1 > HEIGHT
                {
                    break;
                }

                if asteroids.remove(&candidate) {
                    destroyed += 1;

                    if destroyed == 200 {
                        println!("200th asteroid destroyed: {:?}", candidate);

                        return;
                    }

                    break;
                }

                scalar += 1;
            }
        }
    }
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
    fn get_tracking_station_works_1() {
        let map = b"......#.#.\n#..#.#....\n..#######.\n.#.#.###..\n.#..#.....\n..#....#.#\n#..#....#.\n.##.#..###\n##...#..#.\n.#....####";
        let asteroids = read_map(map);

        assert_eq!(((5, 8), 33), get_tracking_station(&asteroids));
    }

    #[test]
    fn get_tracking_station_works_2() {
        let map = b"#.#...#.#.\n.###....#.\n.#....#...\n##.#.#.#.#\n....#.#.#.\n.##..###.#\n..#...##..\n..##....##\n......#...\n.####.###.";
        let asteroids = read_map(map);

        assert_eq!(((1, 2), 35), get_tracking_station(&asteroids));
    }

    #[test]
    fn get_tracking_station_works_3() {
        let map = b".#..#..###\n####.###.#\n....###.#.\n..###.##.#\n##.##.#.#.\n....###..#\n..#.#..#.#\n#..#.#.###\n.##...##.#\n.....#.#..";
        let asteroids = read_map(map);

        assert_eq!(((6, 3), 41), get_tracking_station(&asteroids));
    }

    #[test]
    fn get_tracking_station_works_4() {
        let map = b".#..##.###...#######\n##.############..##.\n.#.######.########.#\n.###.#######.####.#.\n#####.##.#.##.###.##\n..#####..#.#########\n####################\n#.####....###.#.#.##\n##.#################\n#####.##.###..####..\n..######..##.#######\n####.##.####...##..#\n.#####..#.######.###\n##...#.##########...\n#.##########.#######\n.####.#.###.###.#.##\n....##.##.###..#####\n.#.#.###########.###\n#.#.#.#####.####.###\n###.##.####.##.#..##";
        let asteroids = read_map(map);

        assert_eq!(((11, 13), 210), get_tracking_station(&asteroids));
    }

    #[test]
    fn get_visible_asteroid_vectors_works_1() {
        let map = b"......#.#.\n#..#.#....\n..#######.\n.#.#.###..\n.#..#.....\n..#....#.#\n#..#....#.\n.##.#..###\n##...#..#.\n.#....####";
        let asteroids = read_map(map);

        assert_eq!(33, get_visible_asteroid_vectors((5, 8), &asteroids).len());
    }

    #[test]
    fn get_visible_asteroid_vectors_works_2() {
        let map = b"#.#...#.#.\n.###....#.\n.#....#...\n##.#.#.#.#\n....#.#.#.\n.##..###.#\n..#...##..\n..##....##\n......#...\n.####.###.";
        let asteroids = read_map(map);

        assert_eq!(35, get_visible_asteroid_vectors((1, 2), &asteroids).len());
    }

    #[test]
    fn get_visible_asteroid_vectors_works_3() {
        let map = b".#..#..###\n####.###.#\n....###.#.\n..###.##.#\n##.##.#.#.\n....###..#\n..#.#..#.#\n#..#.#.###\n.##...##.#\n.....#.#..";
        let asteroids = read_map(map);

        assert_eq!(41, get_visible_asteroid_vectors((6, 3), &asteroids).len());
    }

    #[test]
    fn get_visible_asteroid_vectors_works_4() {
        let map = b".#..##.###...#######\n##.############..##.\n.#.######.########.#\n.###.#######.####.#.\n#####.##.#.##.###.##\n..#####..#.#########\n####################\n#.####....###.#.#.##\n##.#################\n#####.##.###..####..\n..######..##.#######\n####.##.####...##..#\n.#####..#.######.###\n##...#.##########...\n#.##########.#######\n.####.#.###.###.#.##\n....##.##.###..#####\n.#.#.###########.###\n#.#.#.#####.####.###\n###.##.####.##.#..##";
        let asteroids = read_map(map);

        assert_eq!(
            210,
            get_visible_asteroid_vectors((11, 13), &asteroids).len()
        );
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
