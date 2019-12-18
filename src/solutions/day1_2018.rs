pub fn solve(input: String) {
    let numbers: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let sum: i32 = numbers.iter().sum();

    println!("Part 1: {}", sum);

    let mut frequency: i32 = 0;
    let mut seen_frequencies: Vec<i32> = Vec::new();

    seen_frequencies.push(frequency);

    let mut it = numbers.iter().cycle();

    while let Some(number) = it.next() {
        frequency += number;

        if seen_frequencies.contains(&frequency) {
            println!("Part 2: {}", frequency);
            break;
        }

        seen_frequencies.push(frequency);
    }
}
