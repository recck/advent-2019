use std::collections::HashMap;

struct Orbit {
    direct_orbits: Vec<String>
}

pub fn solve(input: String) {
    let raw_orbits: Vec<Vec<&str>> = input
        .split_whitespace()
        .map(|s| s.split(')').collect())
        .collect();

    let mut space: HashMap<String, Orbit> = HashMap::new();

    for raw_orbit in raw_orbits {
        if let Some(orbit) = space.get_mut(raw_orbit[1]) {
            orbit.direct_orbits.push(raw_orbit[0].to_string());
        } else {
            space.insert(raw_orbit[1].to_string(), Orbit{
                direct_orbits: vec![raw_orbit[0].to_string()]
            });
        }
    }

    println!("{}", calculate_orbits(space));
}

fn calculate_orbits(space: HashMap<String, Orbit>) -> usize {
    let mut total_orbits: usize = 0;

    for (station, orbit) in space.iter() {
        total_orbits += orbit.direct_orbits.len();

        for sub_orbit in orbit.direct_orbits.iter() {
            if let Some(sub_station) = space.get(sub_orbit) {
                total_orbits += sub_station.direct_orbits.len();
            }
        }
    }

    total_orbits
}