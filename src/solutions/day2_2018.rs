use std::collections::HashMap;
use itertools::Itertools;

pub fn solve(input: String) {
    let id_list: Vec<String> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let (mut twos, mut threes) = (0, 0);

    for id in id_list.iter() {
        let mut freq: HashMap<char, i32> = HashMap::new();

        for c in id.chars() {
            match freq.get_mut(&c) {
                Some(character) => {
                    *character += 1;
                },
                None => {
                    freq.insert(c, 1);
                }
            };
        }

        if freq.values().any(|&f| f == 2) {
            twos += 1;
        }

        if freq.values().any(|&f| f == 3) {
            threes += 1;
        }
    }

    println!("Part 1: {}", twos * threes);

    let mut z = id_list.iter().cartesian_product(id_list.iter()).filter_map(|(a, b)| {
        let mut same = Vec::new();
        let mut diff = 0;

        for (a, b) in a.chars().zip(b.chars()) {
            if a == b {
                same.push(a);
            } else {
                diff += 1;
            }

            if diff > 1 { return None }
        }

        match diff {
            0 => None,
            1 => Some(same.into_iter().collect::<String>()),
            _ => None
        }
    });

    if let Some(yeet) = z.next() {
        println!("Part 2: {}", yeet);
    }
}