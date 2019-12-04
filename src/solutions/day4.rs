extern crate regex;

use regex::Regex;

pub fn solve(input: String) {
    let bounds: Vec<i32> = input
        .trim_end()
        .split('-')
        .map(|s| s.parse().unwrap())
        .collect();

    for num in bounds[0]..=bounds[1] {
        if !has_double(num) {
            continue;
        }
    }
}

fn has_double(number: i32) -> bool {
    let double_re: Regex = Regex::new(r"(\d)\1").unwrap();

    double_re.is_match(&number.to_string())
}