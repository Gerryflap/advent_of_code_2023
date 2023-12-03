use regex::Regex;

pub fn run(input_1: String, input_2: String) -> String {
    q1(input_1)
}

fn q1(inp: String) -> String {
    let result: i64 = inp.lines()
        .into_iter()
        .map(parse_game)
        .filter(|(_id, possible)| *possible)
        .map(|(id, _possible)| id)
        .sum();
    format!("Q1: {}", result)
}

fn parse_game(line: &str) -> (i64, bool) {
    let re = Regex::new(r"Game (?<game>\d+):(?<list>.*)").unwrap();
    let Some(caps) = re.captures(line) else {panic!("Cannot parse line {line}")};
    let game: i64 = caps["game"].to_owned()
                        .parse()
                        .expect("There should be a game ID!");
    let possible: bool = is_game_possible(&caps["list"]);
    (game, possible)
}

fn is_game_possible(list: &str) -> bool {
    let subsets = list.split(";");
    subsets.map(is_subset_possible).all(|v| v)
}

fn is_subset_possible(subset: &str) -> bool {
    subset.split(",").map(is_color_possible).all(|v| v)
}

fn is_color_possible(color: &str) -> bool {
    let re = Regex::new(r"(?<count>\d+) (?<color>.*)").unwrap();
    let Some(caps) = re.captures(color) else {panic!("Cannot parse colour {color}")};
    let count: u32 = caps["count"].parse().expect("Count should be parsable");
    // println!("col: {}, count: {}", &caps["color"], count);
    match &caps["color"] {
        "red" => count <= 12,
        "green" => count <= 13,
        "blue" => count <= 14,
        _ => false
    }
}
