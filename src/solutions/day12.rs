use itertools::iproduct;
use regex::Regex;
use std::clone::Clone;

#[derive(Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Clone, Copy)]
struct Velocity {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Clone, Copy)]
struct Moon {
    id: usize,
    position: Position,
    velocity: Velocity,
}

impl Moon {
    fn update_velocity(&mut self, velocity: Velocity) {
        self.velocity.x += velocity.x;
        self.velocity.y += velocity.y;
        self.velocity.z += velocity.z;
    }

    fn update_position(&mut self) {
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
        self.position.z += self.velocity.z;
    }
}

pub fn solve(input: String) {
    let re = Regex::new(r"[<>]").unwrap();
    let fixed_input = re.replace_all(&input, "");
    let input_string: Vec<Vec<i32>> = fixed_input
        .trim_end()
        .split_terminator("\n")
        .map(|s| {
            s.split(',')
                .map(|s| s.trim().split('=').nth(1).unwrap().parse().unwrap())
                .collect()
        })
        .collect();

    let mut moons: Vec<Moon> = Vec::new();

    for (i, moon) in input_string.iter().enumerate() {
        let velocity: Velocity = Velocity { x: 0, y: 0, z: 0 };
        let position: Position = Position {
            x: moon[0],
            y: moon[1],
            z: moon[2],
        };
        moons.push(Moon {
            id: i,
            velocity: velocity,
            position: position,
        });
    }

    for moon_a in &mut moons {
        for moon_b in &mut moons {
            if moon_a.id == moon_b.id {
                continue;
            }
    
            let (vel_a, vel_b) = compare_moons(&moon_a, &moon_b);
            moon_a.update_velocity(vel_a);
            moon_b.update_velocity(vel_b);
        }
    }
}

fn compare_moons(moon_a: &Moon, moon_b: &Moon) -> (Velocity, Velocity) {
    let mut a_velocity = Velocity {x: 0, y: 0, z: 0};
    let mut b_velocity = Velocity {x: 0, y: 0, z: 0};

    if moon_a.position.x != moon_b.position.x {
        a_velocity.x = if moon_a.position.x < moon_b.position.x { 1 } else { -1 };
        b_velocity.x = if moon_a.position.x < moon_b.position.x { -1 } else { 1 };
    }

    if moon_a.position.y != moon_b.position.y {
        a_velocity.y = if moon_a.position.y < moon_b.position.y { 1 } else { -1 };
        b_velocity.y = if moon_a.position.y < moon_b.position.y { -1 } else { 1 };
    }

    if moon_a.position.z != moon_b.position.z {
        a_velocity.z = if moon_a.position.z < moon_b.position.z { 1 } else { -1 };
        b_velocity.z = if moon_a.position.z < moon_b.position.z { -1 } else { 1 };
    }

    (a_velocity, b_velocity)
}
