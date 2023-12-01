pub fn run(input_1: String, input_2: String) -> String {
    format!("Q1: {}\nQ2: {}", q1(input_1), q2(input_2))
}

fn q1(input: String) -> String {
    let result: i64 = input
        .lines()
        .map(|s| s.to_owned())
        .map(combine_last_and_first_num)
        .sum();

    format!("{}", result)
}

fn q2(input: String) -> String {
    let result: i64 = input
        .lines()
        .map(|s| s.to_owned())
        .map(combine_last_and_first_num_q2)
        .sum();

    format!("{}", result)
}

fn combine_last_and_first_num(line: String) -> i64 {
    let line_nums: &Vec<char> = &line.chars().filter(|c| c.is_ascii_digit()).collect();

    let number = format!(
        "{}{}",
        line_nums.first().expect("No numbers in line!"),
        line_nums.last().expect("No numbers in line!")
    );

    number
        .parse()
        .expect("Number should be parsable, but is not")
}

fn combine_last_and_first_num_q2(line: String) -> i64 {
    let nums: &mut Vec<i64> = &mut Vec::new();
    let _dbg_line = &line;
    let mut remaining: &str = &line;

    while !remaining.is_empty() {
        let res = parse_next(remaining);
        if let Some((n, nxt)) = res {
            nums.push(n);
            remaining = nxt;
        } else {
            remaining = remaining.split_at(1).1;
        }
    }

    let number = format!(
        "{}{}",
        nums.first().expect("No numbers in line!"),
        nums.last().expect("No numbers in line!")
    );
    number
        .parse()
        .expect("Number should be parsable, but is not")
}

fn parse_next(text: &str) -> Option<(i64, &str)> {
    parse_num(text).or_else(|| parse_written_num(text))
}

fn parse_num(text: &str) -> Option<(i64, &str)> {
    text.chars()
        .next()
        .and_then(|c| c.to_digit(10))
        .map(|n| (n as i64, text.split_at(1).1))
}

fn parse_written_num(text: &str) -> Option<(i64, &str)> {
    [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]
    .iter()
    .filter_map(|(p, n)| match_str(text, p, *n))
    .next()
}

fn match_str<'a>(text: &'a str, pattern: &str, n: i64) -> Option<(i64, &'a str)> {
    if text.starts_with(pattern) {
        // If you want to consume them all: Some((n, text.split_at(pattern.len()).1))
        Some((n, text.split_at(1).1))
    } else {
        None
    }
}
