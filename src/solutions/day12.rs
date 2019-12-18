use itertools::iproduct;
use regex::Regex;

struct Position {
    x: i32,
    y: i32,
    z: i32,
}

struct Velocity {
    x: i32,
    y: i32,
    z: i32,
}

struct Moon {
    id: usize,
    position: Position,
    velocity: Velocity,
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

    for (&mut moon_a, &mut moon_b) in iproduct!(moons.iter(), moons.iter()) {
        if moon_a.id == moon_b.id {
            continue;
        }

        compare_moons(*moon_a, *moon_b);
    }
}

fn compare_moons(moon_a: &mut Moon, moon_b: &mut Moon) -> () {
    if moon_a.position.x < moon_b.position.x {
        moon_a.position.x += 1;
        moon_b.position.x -= 1;
    }
}
