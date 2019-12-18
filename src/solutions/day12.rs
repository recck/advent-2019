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
    id: i32,
    position: Position,
    velocity: Velocity,
}

pub fn solve(input: String) {
    let mut input_string: Vec<String> = input.trim_end().split_whitespace().collect();

    println!("{:?}", input_string);
}
