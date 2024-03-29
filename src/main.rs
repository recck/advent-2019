use advent::{common::file, solutions};
use std::collections::HashMap;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut solvers: HashMap<String, fn(String)> = HashMap::new();
    solvers.insert(String::from("day1-2018"), solutions::day1_2018::solve);
    solvers.insert(String::from("day2-2018"), solutions::day2_2018::solve);
    solvers.insert(String::from("day1"), solutions::day1::solve);
    solvers.insert(String::from("day2"), solutions::day2::solve);
    solvers.insert(String::from("day3"), solutions::day3::solve);
    solvers.insert(String::from("day4"), solutions::day4::solve);
    solvers.insert(String::from("day5"), solutions::day5::solve);
    solvers.insert(String::from("day8"), solutions::day8::solve);
    solvers.insert(String::from("day12"), solutions::day12::solve);

    if args.len() != 3 {
        eprintln!("Usage: cargo run <day> <input>");
        process::exit(1);
    }

    let advent = &args[1];
    let input_path = &args[2];

    if !file::exists(input_path.to_string()) {
        eprintln!("File `{}` does not exist", input_path);
        process::exit(1);
    }

    let contents = file::read(input_path.to_string());
    let solver = solvers.get(advent);

    match solver {
        Some(solver) => {
            println!("Running {} with {}", advent, input_path);
            solver(contents);
        }
        None => {
            eprintln!("Solver `{}` does not exist", advent);
            process::exit(1);
        }
    };
}
