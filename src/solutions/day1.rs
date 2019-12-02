pub fn solve(input: String) {
    let numbers: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut part1_sum: i32 = 0;
    let mut part2_sum: i32 = 0;

    for mass in numbers {
        part1_sum += required_fuel(mass);

        let mut addtl_fuel = required_fuel(mass);

        while addtl_fuel >= 0 {
            part2_sum += addtl_fuel;
            addtl_fuel = required_fuel(addtl_fuel);
        }
    }

    println!("Part 1: {}", part1_sum);
    println!("Part 2: {}", part2_sum);
}

fn required_fuel(mass: i32) -> i32 {
    mass / 3 - 2
}