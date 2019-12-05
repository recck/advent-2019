pub fn solve(input: String) {
    let bounds: Vec<i32> = input
        .trim_end()
        .split('-')
        .map(|s| s.parse().unwrap())
        .collect();

    let mut matches: i32 = 0;

    for num in bounds[0]..=bounds[1] {
        if valid_password(num).0 {
            matches += 1;
        }
    }

    println!("Part 1: {}", matches);
}

fn valid_password(number: i32) -> (bool, bool) {
    let mut prev: char = '0';
    let mut has_double: bool = false;
    let number_string = number.to_string();
    let number_chars = number_string.chars();

    for c in number_chars {
        if prev == c {
            has_double = true;
        }

        if c < prev {
            return (false, false);
        }

        prev = c;
    }

    (has_double, has_only_adjacent)
}