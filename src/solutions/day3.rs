use std::cmp::{max, min};
use std::collections::HashMap;

pub fn solve(input: String) {
    let wires: Vec<Vec<&str>> = input
        .split_whitespace()
        .map(|s| s.split(',').collect())
        .collect();

    let mut grid: HashMap<(i32, i32), Vec<usize>> = HashMap::new();
    let mut inflection_points: HashMap<usize, Vec<(i32, i32)>> = HashMap::new();

    let central_coord = (0, 0);

    grid.insert(central_coord, vec![]);

    let mut cur_coord = central_coord;

    for (wire, wire_path) in wires.iter().enumerate() {
        let mut inflections: Vec<(i32, i32)> = Vec::new();

        for instruction in wire_path {
            let mut instr_chars = instruction.chars();
            let direction = instr_chars.next().unwrap();
            let value = instr_chars.as_str().parse::<i32>().unwrap();
            let (init_x, init_y) = cur_coord;

            match direction {
                'R' => cur_coord.0 += value,
                'U' => cur_coord.1 += value,
                'D' => cur_coord.1 -= value,
                'L' => cur_coord.0 -= value,
                _ => {}
            }

            inflections.push(cur_coord);

            if cur_coord.0 != init_x {
                for x in get_bounds(init_x, cur_coord.0) {
                    if vec![cur_coord, central_coord].contains(&(x, init_y)) {
                        continue;
                    }

                    if let Some(point) = grid.get_mut(&(x, init_y)) {
                        if !point.contains(&wire) {
                            point.push(wire);
                        }
                    } else {
                        grid.insert((x, init_y), vec![wire]);
                    }
                }
            }

            if cur_coord.1 != init_y {
                for y in get_bounds(init_y, cur_coord.1) {
                    if vec![cur_coord, central_coord].contains(&(init_x, y)) {
                        continue;
                    }

                    if let Some(point) = grid.get_mut(&(init_x, y)) {
                        if !point.contains(&wire) {
                            point.push(wire);
                        }
                    } else {
                        grid.insert((init_x, y), vec![wire]);
                    }
                }
            }
        }

        cur_coord = (0, 0);
        inflection_points.insert(wire, inflections);
    }

    let mut min_manhattan: i32 = std::i32::MAX;
    let mut min_steps: i32 = std::i32::MAX;

    let mut intersections = grid.clone();
    intersections.retain(|k, v| k != &central_coord && v.len() > 1);

    for (coords, _) in intersections.iter() {
        min_manhattan = min_manhattan.min(manhattan(central_coord, *coords));

        let mut steps: i32 = 0;

        for (_, inflections) in inflection_points.iter() {
            steps += calculate_steps_to(*coords, inflections);
        }

        min_steps = min_steps.min(steps);
    }

    println!("Part 1: {}", min_manhattan);
    println!("Part 2: {}", min_steps);
}

fn calculate_steps_to(to: (i32, i32), intersections: &Vec<(i32, i32)>) -> i32 {
    let mut cur_coord = (0, 0);
    let mut steps: i32 = 0;

    for intersection in intersections {
        if cur_coord.0 != intersection.0 {
            let bounds = get_bounds(cur_coord.0, intersection.0);
            if bounds.contains(&to.0) && cur_coord.1 == to.1 {
                steps += distance(to, cur_coord);
                break;
            } else {
                steps += distance(cur_coord, *intersection);
            }
        }

        if cur_coord.1 != intersection.1 {
            let bounds = get_bounds(cur_coord.1, intersection.1);
            if bounds.contains(&to.1) && cur_coord.0 == to.0 {
                steps += distance(to, cur_coord);
                break;
            } else {
                steps += distance(cur_coord, *intersection);
            }
        }

        cur_coord = *intersection;
    }

    steps
}

fn distance(a: (i32, i32), b: (i32, i32)) -> i32 {
    ((b.0 - a.0).pow(2) as f64 + (b.1 - a.1).pow(2) as f64).sqrt() as i32
}

fn get_bounds(a: i32, b: i32) -> std::ops::RangeInclusive<i32> {
    min(a, b)..=max(a, b)
}

fn manhattan(vect_p: (i32, i32), vect_q: (i32, i32)) -> i32 {
    (vect_p.0 - vect_q.0).abs() + (vect_p.1 - vect_q.1).abs()
}
