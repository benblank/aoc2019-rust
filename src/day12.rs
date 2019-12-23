use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashMap;

const INPUT: [[i64; 3]; 4] = [[15, -2, -6], [-5, -4, -11], [0, -6, 0], [5, 9, 6]];

#[derive(Clone, Copy, Debug)]
struct Moon {
    position: (i64, i64, i64),
    velocity: (i64, i64, i64),
}

impl Moon {
    fn get_energy(&self) -> i64 {
        (i64::abs(self.position.0) + i64::abs(self.position.1) + i64::abs(self.position.2))
            * (i64::abs(self.velocity.0) + i64::abs(self.velocity.1) + i64::abs(self.velocity.2))
    }

    fn apply_gravity(&mut self, other: &mut Moon) {
        match self.position.0.cmp(&other.position.0) {
            Ordering::Less => {
                self.velocity.0 += 1;
                other.velocity.0 -= 1;
            }

            Ordering::Greater => {
                self.velocity.0 -= 1;
                other.velocity.0 += 1;
            }

            Ordering::Equal => {}
        }

        match self.position.1.cmp(&other.position.1) {
            Ordering::Less => {
                self.velocity.1 += 1;
                other.velocity.1 -= 1;
            }

            Ordering::Greater => {
                self.velocity.1 -= 1;
                other.velocity.1 += 1;
            }

            Ordering::Equal => {}
        }

        match self.position.2.cmp(&other.position.2) {
            Ordering::Less => {
                self.velocity.2 += 1;
                other.velocity.2 -= 1;
            }

            Ordering::Greater => {
                self.velocity.2 -= 1;
                other.velocity.2 += 1;
            }

            Ordering::Equal => {}
        }
    }
}

fn get_gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        get_gcd(b, a % b)
    }
}

fn get_lcm(a: i64, b: i64) -> i64 {
    a * b / get_gcd(a, b)
}

fn get_moons_from_input(input: &[[i64; 3]; 4]) -> Vec<Moon> {
    input
        .iter()
        .map(|[x, y, z]| Moon {
            position: (*x, *y, *z),
            velocity: (0, 0, 0),
        })
        .collect::<Vec<_>>()
}

fn step(moons: &mut Vec<Moon>) {
    for pair in (0..moons.len()).combinations(2) {
        let mut moon1 = moons[pair[0]].to_owned();
        let mut moon2 = moons[pair[1]].to_owned();

        moon1.apply_gravity(&mut moon2);

        moons[pair[0]] = moon1;
        moons[pair[1]] = moon2;
    }

    for moon in moons {
        moon.position.0 += moon.velocity.0;
        moon.position.1 += moon.velocity.1;
        moon.position.2 += moon.velocity.2;
    }
}

pub fn part1() {
    let mut moons = get_moons_from_input(&INPUT);

    for _ in 0..1000 {
        step(&mut moons);
    }

    println!(
        "Total energy: {}",
        moons.iter().map(|moon| moon.get_energy()).sum::<i64>()
    );
}

pub fn part2() {
    let mut moons = get_moons_from_input(&INPUT);
    let mut history = HashMap::new();
    let mut steps: i64 = 0;

    let x_steps = loop {
        let x_values = moons
            .iter()
            .map(|moon| (moon.position.0, moon.velocity.0))
            .collect::<Vec<_>>();

        if history.contains_key(&x_values) {
            break steps - history.get(&x_values).unwrap();
        }

        history.insert(x_values, steps);
        step(&mut moons);

        steps += 1;
    };

    let y_steps = loop {
        let y_values = moons
            .iter()
            .map(|moon| (moon.position.1, moon.velocity.1))
            .collect::<Vec<_>>();

        if history.contains_key(&y_values) {
            break steps - history.get(&y_values).unwrap();
        }

        history.insert(y_values, steps);
        step(&mut moons);

        steps += 1;
    };

    let z_steps = loop {
        let z_values = moons
            .iter()
            .map(|moon| (moon.position.2, moon.velocity.2))
            .collect::<Vec<_>>();

        if history.contains_key(&z_values) {
            break steps - history.get(&z_values).unwrap();
        }

        history.insert(z_values, steps);
        step(&mut moons);

        steps += 1;
    };

    println!(
        "Moon cycle length: {}",
        get_lcm(x_steps, get_lcm(y_steps, z_steps))
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn moon_get_energy_works_1() {
        assert_eq!(
            290,
            Moon {
                position: (8, -12, -9),
                velocity: (-7, 3, 0)
            }
            .get_energy()
        );
    }

    #[test]
    fn moon_get_energy_works_2() {
        assert_eq!(
            608,
            Moon {
                position: (13, 16, 3),
                velocity: (3, 11, 5)
            }
            .get_energy()
        );
    }

    #[test]
    fn moon_get_energy_works_3() {
        assert_eq!(
            574,
            Moon {
                position: (29, 11, 1),
                velocity: (3, 7, 4)
            }
            .get_energy()
        );
    }

    #[test]
    fn moon_get_energy_works_4() {
        assert_eq!(
            468,
            Moon {
                position: (16, 13, 23),
                velocity: (7, 1, 1)
            }
            .get_energy()
        );
    }

    #[test]
    fn get_gcd_works_1() {
        assert_eq!(12, get_gcd(48, 180));
    }

    #[test]
    fn get_gcd_works_2() {
        assert_eq!(2, get_gcd(62, 36));
    }

    #[test]
    fn get_lcm_works_1() {
        assert_eq!(720, get_lcm(48, 180));
    }

    #[test]
    fn get_lcm_works_2() {
        assert_eq!(42, get_lcm(21, 6));
    }
}
