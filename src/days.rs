use std::collections::HashMap;
use std::fs;

pub mod common;
mod day_1;

pub fn run_day(day: i64, example: bool) {
    let days: &mut HashMap<i64, fn(String, String) -> String> = &mut HashMap::new();
    days.insert(1, day_1::run);

    let func = days.get(&day)
        .expect(format!("The given day {day} cannot be found !").as_str());

    println!("Result: ");
    if example {
        println!("{}", func(read_contents_example(day, 1), read_contents_example(day, 2)));
    } else {
        println!("{}", func(read_contents_real(day), read_contents_real(day)));
    }
}


fn read_contents_real(day: i64) -> String {
    fs::read_to_string(format!("./inputs/day_{day}.txt"))
        .expect(format!("Could not read file './inputs/day_{day}.txt'!").as_str())
}

fn read_contents_example(day: i64, question: i64) -> String {
    fs::read_to_string(format!("./inputs/day_{day}_example_{question}.txt"))
        .unwrap_or("".to_owned())

}