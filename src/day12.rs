use itertools::Itertools;

const INPUT: [[i32; 3]; 4] = [[15, -2, -6], [-5, -4, -11], [0, -6, 0], [5, 9, 6]];

#[derive(Clone, Copy, Debug)]
struct Moon {
    position: (i32, i32, i32),
    velocity: (i32, i32, i32),
}

impl Moon {
    fn get_energy(&self) -> i32 {
        (i32::abs(self.position.0) + i32::abs(self.position.1) + i32::abs(self.position.2))
            * (i32::abs(self.velocity.0) + i32::abs(self.velocity.1) + i32::abs(self.velocity.2))
    }

    fn apply_gravity(&mut self, other: &mut Moon) {
        if self.position.0 < other.position.0 {
            self.velocity.0 += 1;
            other.velocity.0 -= 1;
        } else if self.position.0 > other.position.0 {
            self.velocity.0 -= 1;
            other.velocity.0 += 1;
        }

        if self.position.1 < other.position.1 {
            self.velocity.1 += 1;
            other.velocity.1 -= 1;
        } else if self.position.1 > other.position.1 {
            self.velocity.1 -= 1;
            other.velocity.1 += 1;
        }

        if self.position.2 < other.position.2 {
            self.velocity.2 += 1;
            other.velocity.2 -= 1;
        } else if self.position.2 > other.position.2 {
            self.velocity.2 -= 1;
            other.velocity.2 += 1;
        }
    }
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
    let mut moons = INPUT
        .iter()
        .map(|[x, y, z]| Moon {
            position: (*x, *y, *z),
            velocity: (0, 0, 0),
        })
        .collect::<Vec<_>>();

    for _ in 0..1000 {
        step(&mut moons);
    }

    println!(
        "Total energy: {}",
        moons.iter().map(|moon| moon.get_energy()).sum::<i32>()
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
}
