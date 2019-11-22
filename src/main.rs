use std::env;
use std::process;
use std::collections::HashMap;
use advent::{
    common::file,
    solutions
};

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut solvers: HashMap<String, fn(String)> = HashMap::new();
    solvers.insert(String::from("day1"), solutions::day1::solve);

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
        },
        None => {
            eprintln!("Solver `{}` does not exist", advent);
            process::exit(1);
        }
    };
}