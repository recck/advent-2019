use itertools::Itertools;

pub fn solve(input: String) {
    let bounds: Vec<i32> = input
        .trim_end()
        .split('-')
        .map(|s| s.parse().unwrap())
        .collect();

    let mut part1_matches: i32 = 0;
    let mut part2_matches: i32 = 0;

    for num in bounds[0]..=bounds[1] {
        let validity: (bool, bool) = valid_password(num);
        if validity.0 {
            part1_matches += 1;
        }

        if validity.1 {
            part2_matches += 1;
        }
    }

    println!("Part 1: {}", part1_matches);
    println!("Part 2: {}", part2_matches);
}

fn valid_password(number: i32) -> (bool, bool) {
    let mut prev: char = '0';
    let mut has_double: bool = false;
    let number_string: String = number.to_string();
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

    let has_only_adjacent: bool = number_string
        .chars()
        .filter(|c| number_string.matches(*c).collect::<String>().len() == 2)
        .dedup()
        .count()
        > 0;

    (has_double, has_only_adjacent)
}
