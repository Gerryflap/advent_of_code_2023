use std::collections::HashMap;
use std::fs;

pub mod common;
mod day_1;

pub fn run_day(day: i64, example: bool) {
    let days: &mut HashMap<i64, fn(String) -> String> = &mut HashMap::new();
    days.insert(1, day_1::run);

    let func = days
        .get(&day)
        .unwrap_or_else(|| panic!("The given day {day} cannot be found !"));

    let path: &mut String = &mut format!("./inputs/day_{day}");
    if example {
        path.push_str("_example");
    }
    path.push_str(".txt");
    let path: &String = path;

    let contents =
        fs::read_to_string(path).unwrap_or_else(|_| panic!("Could not read file {path}"));
    println!("Result: ");
    println!("{}", func(contents));
}
